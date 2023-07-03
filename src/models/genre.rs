use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Genre {
  pub id: usize,
  pub name: String,
  pub slug: Option<String>,
}

