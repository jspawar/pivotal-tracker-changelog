use regex::Regex;
use structopt::StructOpt;

mod git;
pub use git::CommitMessageFetcher;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    pub path: std::path::PathBuf,
    #[structopt(long)]
    pub from: String,
    #[structopt(long)]
    pub to: String,
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
    let commits = commit_message_fetcher
        .fetch_messages(&args.from, &args.to)
        .unwrap_or_else(|e| {
            // TODO: log and exit instead of panic
            panic!("Invalid input git repository: {:?}", e);
        });

    let re = Regex::new(r#".*\[#(\d+)\].*"#).unwrap_or_else(|_| {
        todo!("better error message here pls");
    });
    let commits_with_stories: Vec<&(String, String)> =
        commits.iter().filter(|c| re.is_match(&c.1)).collect();
    let sha_story_id_pairs: Vec<(&String, String)> = commits_with_stories
        .iter()
        .map(|c| {
            let cap = re.captures(&c.1).unwrap_or_else(|| {
                todo!("better error message here pls");
            });
            // TODO: check the value exists
            (&c.0, cap[1].to_owned())
        })
        .collect();
    todo!("fetch stories and produce that nice output");
}
