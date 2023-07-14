/*!
    See [EndpointUtils] for full documentation
*/

use crate::{
    utils::response_handler::{APIError, Result},
    EndpointUtils,
};
use serde::de::DeserializeOwned;

/// JSON type representation to parse JSON request
type JSONValue = serde_json::Value;

impl<'a> EndpointUtils<'a> {
    /// Inserts consumer inputs to `fields` statement.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["fields name, description;"]`
    pub fn fields(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["fields ", input, ";"])
    }

    /// Inserts consumer inputs to `exclude` statement.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["exclude version;"]`
    pub fn exclude(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["exclude ", input, ";"])
    }

    /// Inserts consumer inputs to `where` statement.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["where id = 55;"]`
    pub fn where_like(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["where ", input, ";"])
    }

    /// Inserts consumer inputs to `sort` statement.
    /// This is used to sort descending order.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["sort created_at desc;"]`
    pub fn sort_desc(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["sort ", input, " desc;"])
    }

    /// Inserts consumer inputs to `sort` statement.
    /// This is used to sort ascending order.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["sort created_at asc;"]`
    pub fn sort_asc(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["sort ", input, " asc;"])
    }

    /// Inserts consumer inputs to `offset` statement.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["offset 30;"]`
    pub fn offset(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["offset ", input, ";"])
    }

    /// Inserts consumer inputs to `search` statement.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["search "\"mario bros"\";"]`
    pub fn search(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["search ", "\"", input, "\"", ";"])
    }

    /// Inserts consumer inputs to `limit` statement.
    /// EndpointUtils push formed input into its `query_string` vector attribute. e.g. `["limit 10;"]`
    pub fn limit(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
        self.str_iterator(vec!["limit ", input, ";"])
    }

    fn str_iterator(&'a mut self, str_vector: Vec<&'a str>) -> &'a mut EndpointUtils {
        for str_slice in str_vector {
            self.query_string.push(str_slice)
        }
        self
    }

    /// Try to performs request to IGDB API using formed query_string.
    /// Returns an Result containing either a succesfully response converted to user specified Struct or a APIError
    pub fn request<D>(&'a mut self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        let response = self.build_response();

        if let Ok(res) = response {
            if res.status() != 200 {
                return Err(APIError::from_raw(
                    res.status().as_str().to_string(),
                    res.text().unwrap(),
                ));
            }

            match res.json() {
                Ok(result) => Ok(result),
                Err(err) => Err(APIError::from(err)),
            }
        } else {
            Err(response.err().unwrap())
        }
    }

    /// Try to performs request to IGDB API using formed query_string.
    /// Returns an Result containing either a succesfully response converted to JSON or a APIError
    pub fn request_json(&'a mut self) -> Result<Vec<JSONValue>> {
        let response = self.build_response();

        match response {
            Ok(res) => {
                let response_raw_text = res.text();

                if let Ok(raw_text) = response_raw_text {
                    match serde_json::from_str(&raw_text) {
                        Ok(result) => Ok(result),
                        Err(err) => Err(APIError::from(err)),
                    }
                } else {
                    Err(APIError::from(response_raw_text.err().unwrap()))
                }
            }
            Err(_) => Err(response.err().unwrap()),
        }
    }

    fn build_response(&'a mut self) -> Result<reqwest::blocking::Response> {
        let mut body = self.query_string.join("");

        if should_append_body(&body) {
            body.push_str("fields *;")
        }

        self.wrapper.post(body, &format!("{}/", self.endpoint))
    }
}

fn should_append_body(body: &str) -> bool {
    !body.contains("fields")
}
