#[derive(Debug)]
pub enum Error {
    RegexError(regex::Error),
}
impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::RegexError(e)
    }
}
