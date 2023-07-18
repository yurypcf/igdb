use chrono::naive::NaiveDateTime;
use serde::Deserialize;

pub type Result<V> = std::result::Result<V, APIError>;

#[derive(Hash, Clone, Debug, PartialEq, Deserialize)]
pub struct APIError {
    code: String,
    message: String,
}

impl APIError {
    pub fn from_raw(code: String, message: String) -> APIError {
        Self { code, message }
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl From<reqwest::Error> for APIError {
    fn from(value: reqwest::Error) -> APIError {
        APIError::from_raw(
            "HttpClientError".to_string(),
            format!("Unable to parse successful response: {}", value),
        )
    }
}

impl From<serde_json::Error> for APIError {
    fn from(value: serde_json::Error) -> APIError {
        APIError::from_raw(
            "HttpClientError".to_string(),
            format!("Sort options parse error: {}", value),
        )
    }
}

pub fn timestamp_as_string(s: Option<i64>) -> String {
    NaiveDateTime::from_timestamp_opt(s.unwrap(), 0)
        .unwrap()
        .to_string()
}
