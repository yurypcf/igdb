mod response_handler;

use reqwest::{ header::HeaderMap, header::HeaderValue, blocking::Client };
use serde::{ de::DeserializeOwned, Deserialize };
use response_handler::Result;


const BASE_URL: &str = "https://api.igdb.com";
const VERSION:  &str = "v4";

pub struct APIWrapper {
    http_client: Client,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Game {
  id: usize,
  name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Character {
  id: usize,
  name: String,
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

    pub fn post<D>(
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
}

#[derive(Deserialize, Debug)]
struct Game {
  id: usize,
  name: String,
}

#[cfg(test)]
mod tests {
  use std::env;
  use crate::{APIWrapper, Game};

  #[test]
  fn zelda_games() {
    let access_token = env::var("TWITCH_ACCESS_TOKEN").unwrap();
    let client_id = env::var("TWITCH_CLIENT_ID").unwrap();

    let api_wrapper = APIWrapper::new(&access_token, &client_id).unwrap();

    let zelda_games: Vec<Game> = api_wrapper.post().unwrap();

    println!("{:?}", zelda_games);
  }
}
