use serde::Deserialize;

pub type CompanyWebsiteResult = Vec<CompanyWebsite>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CompanyWebsite {
    pub id: u64,
    pub category: i32,
    pub trusted: bool,
    pub url: String,
    pub checksum: String,
}
