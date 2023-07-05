use serde::{ Deserialize };

#[derive(Deserialize, Debug, PartialEq)]
pub struct Character {
  pub id: usize,
  pub akas: Option<Vec<String>>,
  pub checksum: Option<String>,
  pub country_name: Option<String>,
  pub created_at: Option<usize>,
  pub description: Option<String>,
  pub games: Option<Vec<usize>>,
  pub gender: Option<u8>,
  pub mug_shot: Option<usize>,
  pub name: Option<String>,
  pub slug: Option<String>,
  pub species: Option<u8>,
  pub updated_at: Option<usize>,
  pub url: Option<String>,
}

}
