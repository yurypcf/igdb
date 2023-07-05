use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Genre {
  pub id: usize,
  pub checksum: Option<String>,
  pub created_at: Option<usize>,
  pub name: Option<String>,
  pub slug: Option<String>,
  pub updated_at: Option<usize>,
  pub url: Option<String>,
}