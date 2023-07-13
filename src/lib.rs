/*!
    Wraps IGDB API calls into Rust usage.
    The crate enables to call endpoints from [IGDB API](https://api-docs.igdb.com/) and performs a query on behalf of the consumer chosen endpoint.
    See [project repository](https://github.com/yurypcf/igdb) for installation, configuration and usage.
    When not using the `.request_json()` public method, you should bring into your project scope
    the structs you wish to parse. See [models]
*/

pub mod models;
pub mod utils;

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue},
};
use utils::response_handler::{APIError, Result};

const BASE_URL: &str = "https://api.igdb.com";
const VERSION: &str = "v4";

/// Responsible for starting the API connection to IGDB Twitch and performing requests
pub struct APIWrapper {
    http_client: Client,
}

/// Responsible for chaining the query methods in itself and performing the http request in the chosen consumer endpoint.
/// Each query method implemented on EndpointUtils represents a APICalypse query build.
/// See [APICalypse official documentation](https://apicalypse.io/syntax/) for more information
pub struct EndpointUtils<'a> {
  /// Holds the API Wrapper reference so we can perform the post http method
  pub wrapper: &'a APIWrapper,
  /// Vector of string slices that represents the query body that will be sent
  pub query_string: Vec<&'a str>,
  /// Endpoint representation
  pub endpoint: &'a str,
}

impl APIWrapper {
    /// Returns Self with a ready http_client accordingly to access token and client id provisioned
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

    /// Returns EndpointUtils struct pointing to `/games` endpoint from IGDB.
    #[cfg(feature = "game")]
    pub fn games(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "games",
        }
    }

    /// Returns EndpointUtils struct pointing to `/characters` endpoint from IGDB.
    #[cfg(feature = "character")]
    pub fn characters(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "characters",
        }
    }

    /// Returns EndpointUtils struct pointing to `/genres` endpoint from IGDB.
    #[cfg(feature = "genre")]
    pub fn genres(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "genres",
        }
    }

    /// Returns EndpointUtils struct pointing to `/collections` endpoint from IGDB.
    #[cfg(feature = "collection")]
    pub fn collections(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "collections",
        }
    }

    /// Returns EndpointUtils struct pointing to `/platforms` endpoint from IGDB.
    #[cfg(feature = "platform")]
    pub fn platforms(&self) -> EndpointUtils<'_> {
        EndpointUtils {
            wrapper: self,
            query_string: Vec::new(),
            endpoint: "platforms",
        }
    }

    /// Returns EndpointUtils struct pointing to `/themes` endpoint from IGDB.
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
