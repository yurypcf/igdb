use serde::Deserialize;

pub type CompanyWebsiteResult = Vec<CompanyWebsite>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CompanyWebsite {
    pub id: u64,
    pub category: Option<i32>,
    pub trusted: Option<bool>,
    pub url: Option<String>,
    pub checksum: Option<String>,
}
