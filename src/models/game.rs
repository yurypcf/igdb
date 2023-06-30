use serde::{ Deserialize };

#[derive(Deserialize, Debug, PartialEq)]
pub struct Game {
  pub id: usize,
  pub name: String,
}