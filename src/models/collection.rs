use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Collection {
    pub id: usize,
    pub checksum: Option<String>,
    pub created_at: Option<usize>,
    pub games: Option<Vec<usize>>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub updated_at: Option<usize>,
    pub url: Option<String>,
}
