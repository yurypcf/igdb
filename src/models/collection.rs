use crate::utils::response_handler::timestamp_as_string;
use serde::Deserialize;

pub type CollectionResult = Vec<Collection>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Collection {
    pub id: u64,
    pub created_at: Option<i64>,
    pub games: Option<Vec<u64>>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub updated_at: Option<i64>,
    pub url: Option<String>,
    pub checksum: Option<String>,
}

impl Collection {
    pub fn created_at(&self) -> String {
        timestamp_as_string(self.created_at)
    }

    pub fn updated_at(&self) -> String {
        timestamp_as_string(self.updated_at)
    }
}
