use serde::Deserialize;

pub type AlternativeNameResult = Vec<AlternativeName>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AlternativeName {
    pub id: u64,
    pub comment: Option<String>,
    pub game: Option<u64>,
    pub name: String,
    pub checksum: String,
}
