pub mod models;
mod utils;

use reqwest::{ header::HeaderMap, header::HeaderValue, blocking::Client };
use serde::{ de::DeserializeOwned };
use utils::EndpointUtils;
use utils::response_handler::Result;

const BASE_URL: &str = "https://api.igdb.com";
const VERSION:  &str = "v4";

pub struct APIWrapper {
    http_client: Client,
}

impl APIWrapper {
    pub fn new(
        access_token: &str,
        client_id: &str
    ) -> Result<APIWrapper> {
        let mut headers: HeaderMap = HeaderMap::new();
        
        headers.insert("Authorization", format!("Bearer {}", access_token).parse().unwrap());
        headers.insert("Client-ID", HeaderValue::from_str(client_id).unwrap());
    
        let http_client: Client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let wrapper = APIWrapper { http_client };

        Ok(wrapper)
    }

    fn post<D>(
      &self,
      body: String,
      request_endpoint: &str
    ) -> Result<D>
    where
      D: DeserializeOwned,
    {
        let response = self
          .http_client
          .post(format!("{}/{}/{}", BASE_URL, VERSION, request_endpoint))
          .body(body)
          .send()
          .unwrap();

        Ok(response.json().unwrap())
    }

    pub fn games<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "games"}
    }

    pub fn characters<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "characters"}
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use models::{Game, Character};
  use std::env;

  #[test]
  fn search_games() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let zelda_games: Vec<Game> = api_wrapper.games().search("zelda").limit("2").fields("name").request().unwrap();

    let expected_result: Vec<Game> = vec![
      Game { id: 1025, name: String::from("Zelda II: The Adventure of Link") },
      Game { id: 1022, name: String::from("The Legend of Zelda") }
    ];

    assert_eq!(expected_result, zelda_games)
  }

  #[test]
  fn search_characters() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let mario_characters: Vec<Character> = api_wrapper.characters().search("mario").fields("name").request().unwrap();

    let expected_result: Vec<Character> = vec![
      Character { id: 3726, name: String::from("Mario") },
      Character { id: 1564, name: String::from("Mario Alcalde") },
      Character { id: 1112, name: String::from("Mario Auditore") }
    ];

    assert_eq!(expected_result, mario_characters)
  }
}
