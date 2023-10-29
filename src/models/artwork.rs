use serde::Deserialize;

pub type ArtworkResult = Vec<Artwork>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Artwork {
    pub id: u64,
    pub alpha_channel: Option<bool>,
    pub animated: Option<bool>,
    pub game: Option<u64>,
    pub height: Option<i32>,
    pub image_id: Option<String>,
    pub url: Option<String>,
    pub width: Option<i32>,
    pub checksum: Option<String>,
}
