use crate::utils::response_handler::timestamp_as_string;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Collection {
    pub id: usize,
    pub checksum: Option<String>,
    pub created_at: Option<i64>,
    pub games: Option<Vec<usize>>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub updated_at: Option<i64>,
    pub url: Option<String>,
}

impl Collection {
    pub fn created_at(&self) -> String {
        timestamp_as_string(self.created_at)
    }

    pub fn updated_at(&self) -> String {
        timestamp_as_string(self.updated_at)
    }
}
