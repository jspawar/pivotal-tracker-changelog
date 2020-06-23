mod git;
mod pivotal_tracker;

use git::CommitMessageFetcher;
use pivotal_tracker::{Story, StoryFetcher, TrackerResponse};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Utility to generate GitHub release notes for a repository containing commits referencing Pivotal Tracker stories
struct Cli {
    /// Absolute path to git repository
    #[structopt(short, long)]
    pub path: std::path::PathBuf,
    /// Reference to begin range of commits to inspect
    #[structopt(long)]
    pub from: String,
    /// Reference to end range of commits to inspect
    #[structopt(long)]
    pub to: String,
    /// Pivotal Tracker API token used to fetch story information
    #[structopt(long, env = "TRACKER_API_TOKEN")]
    pub token: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Cli::from_args();

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

    let messages = commit_message_pairs
        .into_iter()
        .map(|(_, s)| s)
        .collect::<Vec<String>>();
    let story_ids =
        StoryFetcher::story_ids_from_commit_messages(&messages[..]).unwrap_or_else(|e| {
            todo!("handle this error better: {:?}", e);
        });

    let story_fetcher = StoryFetcher::new(args.token).unwrap_or_else(|e| {
        todo!(
            "how to handle this error when constructing story fetcher: {:?}",
            e
        );
    });
    let responses = story_fetcher
        .fetch_stories(story_ids)
        .await
        .unwrap_or_else(|e| {
            todo!("how to handle this error when fetching stories: {:?}", e);
        });
    let mut stories: Vec<Story> = responses
        .into_iter()
        .filter_map(|r| match r {
            TrackerResponse::StoryResponse(story) => Some(story),
            TrackerResponse::ErrorResponse(error) => {
                eprintln!("received error when fetching story: {:?}", error.error);
                None
            }
        })
        .collect();
    stories.sort_by(|a, b| a.id.cmp(&b.id));
    stories.dedup();

    for story in stories {
        println!("- {} [details]({})", &story.name, &story.url);
    }

    Ok(())
}
