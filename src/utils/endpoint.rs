use crate::{APIWrapper, utils::response_handler::Result};
use serde::de::DeserializeOwned;

pub struct EndpointUtils<'a>{
  pub wrapper: &'a APIWrapper,
  pub query_string: Vec<&'a str>,
  pub endpoint: &'a str,
}


impl<'a> EndpointUtils<'a> {
  pub fn fields(&'a mut self, input: &'a str) -> &'a mut EndpointUtils {
    self.str_iterator(vec!["fields ", input, ";"])
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
    self.wrapper.post(body, &format!("{}/", self.endpoint))
  }
}