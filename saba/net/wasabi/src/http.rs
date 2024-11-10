extern crate alloc;
use alloc::string::String;
use alloc::format;
use crate::alloc::string::ToString;
use noli::net::lookup_host;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
    let ips = match lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(Error::Network(format!(
          "Failed to find IP addresses: {:#?}",
          e1
        )))
      }
    }

    if ips.len() < 1 {
      return Err(Error::Network("Failed to find IP addresses".to_string()));
    }
  }
}