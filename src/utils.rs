use crate::{APIWrapper, response_handler};
use serde::de::DeserializeOwned;
use response_handler::Result;

pub struct EndpointHelper<'a>{
  pub wrapper: &'a APIWrapper,
  pub query_string: Vec<&'a str>,
  pub endpoint: &'a str,
}


impl<'a> EndpointHelper<'a> {
  pub fn fields(&'a mut self, input: &'a str) -> &'a mut EndpointHelper {
    self.str_iterator(vec!["fields ", input, ";"])
  }
  
  pub fn search(&'a mut self, input: &'a str) -> &'a mut EndpointHelper {
    self.str_iterator(vec!["search ", "\"", input, "\"", ";"])
  }

  pub fn limit(&'a mut self, input: &'a str) -> &'a mut EndpointHelper {
    self.str_iterator(vec!["limit ", input, ";"])
  }

  pub fn str_iterator(&'a mut self, str_vector: Vec<&'a str>) -> &'a mut EndpointHelper {
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