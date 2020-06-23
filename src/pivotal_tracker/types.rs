use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub kind: String,
    pub code: String,
    pub error: String,
    pub requirement: Option<String>,
    pub general_problem: Option<String>,
    pub possible_fix: Option<String>,
    // TODO: the rest
    // pub validation_errors: Option<Vec<ValidationErrorResponse>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    pub kind: String,
    pub id: i32,
    pub name: String,
    pub url: String,
    // TODO: the rest
}
impl std::cmp::PartialEq for Story {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TrackerResponse {
    StoryResponse(Story),
    ErrorResponse(ErrorResponse),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn story_response_from_enum_deserializes_correctly() {
        let raw = r#"{"kind":"story","id":1,"name":"some story title","url":"pivotal.tracker"}"#;
        let response: TrackerResponse = serde_json::from_str(raw).unwrap();
        match response {
            TrackerResponse::StoryResponse(story) => {
                assert_eq!(story.kind, "story");
                assert_eq!(story.id, 1);
                assert_eq!(story.name, "some story title");
                assert_eq!(story.url, "pivotal.tracker");
            }
            _ => {
                panic!("deserialized to incorrect variant");
            }
        }
    }

    #[test]
    fn error_response_from_enum_deserializes_correctly() {
        let raw = r#"{"kind":"error","code":"invalid_parameter","error":"One or more request parameters was missing or invalid."}"#;
        let response: TrackerResponse = serde_json::from_str(raw).unwrap();
        match response {
            TrackerResponse::ErrorResponse(error) => {
                assert_eq!(error.kind, "error");
                assert_eq!(error.code, "invalid_parameter");
                assert_eq!(
                    error.error,
                    "One or more request parameters was missing or invalid."
                );
            }
            _ => {
                panic!("deserialized to incorrect variant");
            }
        }
    }
}
