pub mod models;
mod utils;

use reqwest::{ header:: { HeaderMap, HeaderValue }, blocking::{ Client, Response } };
use serde::de::DeserializeOwned;
use utils::{ EndpointUtils, response_handler:: { Result, APIError } };

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
    ) -> Result<Response>
    where
      D: DeserializeOwned,
    {
      let url = format!("{}/{}/{}", BASE_URL, VERSION, request_endpoint);
      match self.http_client.post(url).body(body).send() {
        Ok(res) => Ok(res),
        Err(err) => Err(APIError::from(err)),
      }
    }

    #[cfg(feature = "game")]
    pub fn games<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "games"}
    }

    #[cfg(feature = "character")]
    pub fn characters<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "characters"}
    }

    #[cfg(feature = "genre")]
    pub fn genres<'a>(&'a self) -> EndpointUtils<'a>{
      EndpointUtils { wrapper: self, query_string: Vec::new(), endpoint: "genres"}
    }
}

#[cfg(test)]
mod tests;
