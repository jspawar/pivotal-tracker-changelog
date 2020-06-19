use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    pub id: i32,
    pub kind: String,
    pub name: String,
    pub url: String,
    // TODO: the rest
}
impl std::cmp::PartialEq for Story {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
// TODO: tests?
