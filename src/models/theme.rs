use serde::Deserialize;
use crate::utils::response_handler::timestamp_as_string;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Theme {
    pub id: usize,
    pub checksum: Option<String>,
    pub created_at: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub updated_at: Option<i64>,
    pub url: Option<String>,
}

impl Theme {
  pub fn created_at(&self) -> String {
    timestamp_as_string(self.created_at)
  }

  pub fn updated_at(&self) -> String {
    timestamp_as_string(self.updated_at)
  }
}
