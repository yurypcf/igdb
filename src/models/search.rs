use crate::utils::response_handler::timestamp_as_string;
use serde::Deserialize;

pub type SearchResult = Vec<Search>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Search {
    pub id: u64,
    pub alternative_name: Option<String>,
    pub character: Option<u64>,
    pub collection: Option<u64>,
    pub company: Option<u64>,
    pub description: Option<String>,
    pub game: Option<u64>,
    pub name: String,
    pub platform: Option<u64>,
    pub published_at: Option<i64>,
    pub test_dummy: Option<u64>,
    pub theme: Option<u64>,
    pub checksum: Option<String>,
}

impl Search {
  pub fn published_at(&self) -> String {
    timestamp_as_string(self.published_at)
  }
}