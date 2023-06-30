use serde::{ Deserialize };

#[derive(Deserialize, Debug, PartialEq)]
pub struct Character {
  pub id: usize,
  pub name: String,
}
