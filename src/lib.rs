pub mod models;
mod utils;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue},
};
use utils::{
    response_handler::{APIError, Result},
    EndpointUtils,
};

const BASE_URL: &str = "https://api.igdb.com";
const VERSION: &str = "v4";

pub struct APIWrapper {
    http_client: Client,
}

impl APIWrapper {
    pub fn new(access_token: &str, client_id: &str) -> Result<APIWrapper> {
        let mut headers: HeaderMap = HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse().unwrap(),
        );
        headers.insert("Client-ID", HeaderValue::from_str(client_id).unwrap());

        let http_client: Client = Client::builder().default_headers(headers).build().unwrap();

        let wrapper = APIWrapper { http_client };

        Ok(wrapper)
    }

    fn post(&self, body: String, request_endpoint: &str) -> Result<Response> {
        let url = format!("{}/{}/{}", BASE_URL, VERSION, request_endpoint);
        match self.http_client.post(url).body(body).send() {
            Ok(res) => Ok(res),
            Err(err) => Err(APIError::from(err)),
        }
    }

    #[cfg(feature = "game")]
    pub fn games(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "games",
        }
    }

    #[cfg(feature = "character")]
    pub fn characters(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "characters",
        }
    }

    #[cfg(feature = "genre")]
    pub fn genres(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "genres",
        }
    }

    #[cfg(feature = "collection")]
    pub fn collections(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "collections",
        }
    }

    #[cfg(feature = "platform")]
    pub fn platforms(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "platforms",
        }
    }

    #[cfg(feature = "theme")]
    pub fn themes(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "themes",
        }
    }
}

#[cfg(test)]
mod tests;
