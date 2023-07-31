use crate::utils::response_handler::timestamp_as_string;
use serde::Deserialize;

pub type FranchiseResult = Vec<Franchise>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Franchise {
    pub id: u64,
    pub created_at: Option<i64>,
    pub games: Option<Vec<u64>>,
    pub name: String,
    pub slug: String,
    pub updated_at: Option<i64>,
    pub url: String,
    pub checksum: String,
}

impl Franchise {
  pub fn created_at(&self) -> String {
    timestamp_as_string(self.created_at)
  }

  pub fn updated_at(&self) -> String {
      timestamp_as_string(self.updated_at)
  }
}