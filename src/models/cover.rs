use serde::Deserialize;

pub type CoverResult = Vec<Cover>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Cover {
    pub id: u64,
    pub alpha_channel: Option<bool>,
    pub animated: Option<bool>,
    pub game: Option<u64>,
    pub height: Option<i32>,
    pub image_id: Option<String>,
    pub url: String,
    pub width: Option<i32>,
    pub checksum: String,
    pub game_localization: Option<u64>,
}