use crate::pivotal_tracker::{Error, Story};
use futures::{prelude::*, stream::futures_unordered::FuturesUnordered};
use regex::Regex;

const TRACKER_API_TOKEN_HEADER: &'static str = "X-TrackerToken";

pub struct StoryFetcher {
    tracker_api_token: String,
    client: reqwest::Client,
}

impl StoryFetcher {
    pub fn new(tracker_api_token: String) -> Result<Self, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            TRACKER_API_TOKEN_HEADER,
            reqwest::header::HeaderValue::from_str(&tracker_api_token)?,
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(StoryFetcher { client })
    }

    pub fn story_ids_from_commit_messages<'a>(
        commit_messages: &'a [String],
    ) -> Result<Vec<&'a str>, Error> {
        let re = Regex::new(r#"\[(?:finishes\s){0,1}#(\d+)\]"#)?;

        Ok(commit_messages
            .iter()
            .map(|s| {
                re.captures_iter(&s)
                    .map(|c| {
                        if let Some(m) = c.get(1) {
                            m.as_str()
                        } else {
                            ""
                        }
                    })
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<&str>>()
            })
            .flatten()
            .collect::<Vec<&str>>())
    }

    pub async fn fetch_stories(&self, story_ids: Vec<&str>) -> Result<Vec<Story>, Error> {
        let mut futures = FuturesUnordered::new();
        for story_id in story_ids {
            // TODO: deserialize to a union type to handle error responses (e.g. any 4xx error)
            futures.push(async move {
                self.client
                    .get(&format!(
                        "https://www.pivotaltracker.com/services/v5/stories/{}",
                        story_id
                    ))
                    .send()
                    .await?
                    .json::<Story>()
                    .await
            });
        }

        let mut stories: Vec<Story> = Vec::new();
        while let Some(result) = futures.next().await {
            stories.push(result?);
        }
        Ok(stories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_story_ids_from_commit_messages_correctly() {
        let commit_messages: Vec<String> = vec![
            "commit with id: [#1234]".to_owned(),
            "no story id here!".to_owned(),
            "another one with id: [#5678]".to_owned(),
            "message with three ids: [#9012], [finishes #3456], [#7890]".to_owned(),
        ];
        let mut story_ids =
            StoryFetcher::story_ids_from_commit_messages(&commit_messages[..]).unwrap();
        // sort result to make it easier to assert against
        story_ids.sort();

        assert_eq!(story_ids.len(), 5);
        assert_eq!(story_ids[0], "1234");
        assert_eq!(story_ids[1], "3456");
        assert_eq!(story_ids[2], "5678");
        assert_eq!(story_ids[3], "7890");
        assert_eq!(story_ids[4], "9012");
    }

    #[tokio::test]
    async fn fetch_stories_returns_expected_stories() {
        let tracker_api_token = env!("TRACKER_API_TOKEN");
        if tracker_api_token.is_empty() {
            panic!("need to set `TRACKER_API_TOKEN` in environment");
        }
        let fetcher = StoryFetcher::new(tracker_api_token.to_owned()).unwrap();
        let story_ids = vec!["173185328", "173185318", "173185203"];

        let mut stories = fetcher.fetch_stories(story_ids).await.unwrap();
        // sort result to make it easier to assert against
        stories.sort_by(|a, b| a.id.cmp(&b.id));

        assert_eq!(stories.len(), 3);
        // using stories from the following project: https://www.pivotaltracker.com/n/projects/2196383
        assert_eq!(
            stories[0].name,
            "**API** client can **discover** app usage events on the v3 API"
        );
        assert_eq!(
            stories[1].name,
            "**API** client can **view** an app usage event"
        );
        assert_eq!(
            stories[2].name,
            "**API** client can **list** app usage events"
        );
    }
}
