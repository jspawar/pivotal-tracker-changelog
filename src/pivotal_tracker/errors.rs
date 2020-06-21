#[derive(Debug)]
pub enum Error {
    RegexError(regex::Error),
    ReqwestError(reqwest::Error),
    ReqwestInvalidHeaderError(reqwest::header::InvalidHeaderValue),
}
impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::RegexError(e)
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}
impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::ReqwestInvalidHeaderError(e)
    }
}
