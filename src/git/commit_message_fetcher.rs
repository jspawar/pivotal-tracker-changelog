use git2::Repository;

pub struct CommitMessageFetcher {
    pub repository: Repository,
}

#[derive(Debug)]
pub enum Error {
    GitError(git2::Error),
    NoneError(String),
}
impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Error::GitError(error)
    }
}

impl CommitMessageFetcher {
    pub fn new(repository_path: String) -> Result<Self, git2::Error> {
        let repository = Repository::open(repository_path)?;
        Ok(CommitMessageFetcher {
            repository: repository,
        })
    }

    pub fn fetch_messages(
        &mut self,
        from_sha: &str,
        to_sha: &str,
    ) -> Result<Vec<(String, String)>, Error> {
        let mut result = Vec::<(String, String)>::new();

        let mut rev_walk = self.repository.revwalk()?;
        rev_walk.push_range(&format!("{}^..{}", from_sha, to_sha))?;

        for oid_result in rev_walk {
            let oid = oid_result?;
            let commit = self.repository.find_commit(oid)?;
            if let Some(commit_message) = commit.message() {
                result.push((oid.to_string(), commit_message.to_owned()));
            } else {
                return Err(Error::NoneError(format!(
                    "commit with empty message: {:?}",
                    &commit
                )));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const HOME_DIR: &'static str = env!("HOME");

    #[test]
    fn construct_fetcher_with_valid_local_repo() {
        // TODO: use something that can run in CI
        let valid_local_repo = format!("{}/workspace/pivotal-tracker-changelog", HOME_DIR);
        let fetcher = CommitMessageFetcher::new(valid_local_repo);
        assert!(fetcher.is_ok());
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn construct_fetcher_with_invalid_local_repo() {
        let fetcher = CommitMessageFetcher::new("/this/does/not/exist/i/hope".to_string());
        fetcher.unwrap();
    }

    #[test]
    fn fetch_messages_with_valid_range() {
        // TODO: use something that can run in CI
        let path_for_this_repo = format!("{}/workspace/pivotal-tracker-changelog", HOME_DIR);
        let mut fetcher = CommitMessageFetcher::new(path_for_this_repo).unwrap();

        let result = fetcher.fetch_messages(
            "cdfa788ae3caf7d9bbb3d74fe4419b339c0dadd2",
            "a7ff5a7c14755e1ab438256437d0798dde0b487c",
        );

        let commit_messages = result.unwrap();
        assert_eq!(commit_messages.len(), 2);
        assert_eq!(
            commit_messages[0].1,
            "Begin implementing logic to fetch git commits\n"
        );
        assert_eq!(
            commit_messages[1].1,
            "Add some dependencies that'll be needed\n"
        );
    }
}
