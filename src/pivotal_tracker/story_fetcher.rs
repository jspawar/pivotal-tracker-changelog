use crate::pivotal_tracker::Error;
use regex::Regex;

pub struct StoryFetcher {}

impl StoryFetcher {
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
}
