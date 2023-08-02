use serde::Deserialize;

pub type CompanyLogoResult = Vec<CompanyLogo>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CompanyLogo {
    pub id: u64,
    pub alpha_channel: Option<bool>,
    pub animated: Option<bool>,
    pub height: Option<i32>,
    pub image_id: Option<String>,
    pub url: String,
    pub width: Option<i32>,
    pub checksum: String,
}
