use crate::{APIWrapper, utils::response_handler::{ Result, APIError }};
use serde::de::DeserializeOwned;

pub struct EndpointUtils<'a>{
  pub wrapper: &'a APIWrapper,
  pub query_string: Vec<&'a str>,
  pub endpoint: &'a str,
}

type JSONValue = serde_json::Value;

impl<'a> EndpointUtils<'a> {
  pub fn fields(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["fields ", input, ";"])
  }

  pub fn exclude(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["exclude ", input, ";"])
  }

  pub fn where_like(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["where ", input, ";"])
  }

  pub fn sort_desc(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["sort ", input, " desc;"])
  }

  pub fn sort_asc(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["sort ", input, " asc;"])
  }

  pub fn offset(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["offset ", input, ";"])
  }
  
  pub fn search(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["search ", "\"", input, "\"", ";"])
  }

  pub fn limit(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["limit ", input, ";"])
  }

  pub fn str_iterator(&'a mut self, str_vector: Vec<&'a str>) -> &'a mut EndpointUtils {
    for str_slice in str_vector {
        self.query_string.push(str_slice)
    }
    self
  }

  pub fn request<D>(&'a self) -> Result<Vec<D>>
  where
    D: DeserializeOwned,
  {
    let body = self.query_string.join("");
    let response = self.wrapper.post::<D>(body, &format!("{}/", self.endpoint));
    

    if let Ok(res) =  response {
      if res.status() != 200 {
        return Err(APIError::from_raw(
          res.status().as_str().to_string(),
          res.text().unwrap()
        ))  
      }

      match res.json() {
        Ok(result) => Ok(result),
        Err(err) => Err(APIError::from(err))
      }
    } else {
      Err(APIError::from(response.err().unwrap()))
    }
  }

  pub fn request_json(&'a self) -> Result<Vec<JSONValue>>
  {
    let body = self.query_string.join("");
    let response = self.wrapper.post::<JSONValue>(body, &format!("{}/", self.endpoint));

    if let Ok(res) =  response {
      let response_raw_text = res.text();

      if let Ok(raw_text) = response_raw_text {
        match serde_json::from_str(&raw_text) {
          Ok(result) => Ok(result),
          Err(err) => Err(APIError::from(err))
        }
      } else {
        Err(APIError::from(response_raw_text.err().unwrap()))
      }
    } else {
      Err(APIError::from(response.err().unwrap()))
    }
  }
}