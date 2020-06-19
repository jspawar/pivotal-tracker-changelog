mod git;
mod pivotal_tracker;

use regex::Regex;
use structopt::StructOpt;

use git::CommitMessageFetcher;
use pivotal_tracker::Story;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    pub path: std::path::PathBuf,
    #[structopt(long)]
    pub from: String,
    #[structopt(long)]
    pub to: String,
    #[structopt(long, env = "TRACKER_API_TOKEN")]
    pub token: String,
}

fn main() {
    let args = Cli::from_args();
    // 1. get story IDs
    // 2. fetch story object from remote for each story ID
    // 3. produce something like "- {story.name} [details](story.url)"
    let mut commit_message_fetcher = CommitMessageFetcher::new(
        args.path
            .to_str()
            .unwrap_or_else(|| {
                todo!("better error message here pls");
            })
            .to_owned(),
    )
    .unwrap_or_else(|e| {
        // TODO: log and exit instead of panic
        panic!("Invalid input git repository: {}", e);
    });
    let commit_message_pairs = commit_message_fetcher
        .fetch_messages(&args.from, &args.to)
        .unwrap_or_else(|e| {
            // TODO: log and exit instead of panic
            panic!("Invalid input git repository: {:?}", e);
        });

    let re = Regex::new(r#".*\[(?:finishes\s){0,1}#(\d+)\].*"#).unwrap_or_else(|_| {
        todo!("better error message here pls");
    });
    let sha_message_with_story_pairs: Vec<&(String, String)> = commit_message_pairs
        .iter()
        .filter(|c| re.is_match(&c.1))
        .collect();
    let sha_story_id_pairs: Vec<(&String, String)> = sha_message_with_story_pairs
        .iter()
        .map(|c| {
            let cap = re.captures(&c.1).unwrap_or_else(|| {
                todo!("better error message here pls");
            });
            // TODO: check the value exists
            (&c.0, cap[1].to_owned())
        })
        .collect();

    // TODO: send requests concurrently/in parallel for better pef pls
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-TrackerToken",
        reqwest::header::HeaderValue::from_str(&args.token).unwrap_or_else(|_| {
            todo!("omg how many errors do i need to handle");
        }),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_else(|_| {
            todo!("handle this error pls");
        });
    let mut stories: Vec<Story> = Vec::new();
    // TODO: if first part of tuple isn't needed...maybe don't need a tuple :shrug:
    for (_, story_id) in sha_story_id_pairs {
        let resp = client
            .get(&format!(
                "https://www.pivotaltracker.com/services/v5/stories/{}",
                &story_id
            ))
            .send()
            .unwrap_or_else(|e| {
                todo!("better error handling here pls: {:?}", e);
            });
        match resp.json::<Story>() {
            Ok(story) => {
                stories.push(story);
            }
            _ => {}
        };
    }
    stories.sort_by(|a, b| a.id.cmp(&b.id));
    stories.dedup();

    for story in stories {
        println!("- {} [details]({})", &story.name, &story.url);
    }
}
