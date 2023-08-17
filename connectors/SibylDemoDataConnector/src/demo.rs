use std::prelude::v1::*;
use sibyl_base_data_connector::utils::simple_tls_client;
use sibyl_base_data_connector::base::DataConnector;
use sibyl_base_data_connector::serde_json::Value;
use sibyl_base_data_connector::errors::NetworkError;
use std::convert::TryInto;

pub struct DemoConnector {}

impl DataConnector for DemoConnector {
  fn query(&self, query_type: &Value, query_param: &Value) -> Result<Value, NetworkError> {
    let query_type_str = match query_type.as_str() {
      Some(r) => r,
      _ => {
        let err = format!("query_type to str failed");
        println!("{:?}", err);
        return Err(NetworkError::String(err));
      }
    };
    match query_type_str {
      "demo_post" => {
        let host = query_param["host"].as_str().unwrap_or("");
        let port = query_param["port"].as_i64().unwrap_or(443);
        let post_body = query_param["post_body"].as_str().unwrap_or("");
        let url = query_param["url"].as_str().unwrap_or("/");
        let req = format!(
          "POST {} HTTP/1.1\r\n\
          HOST: {}\r\n\
          User-Agent: curl/7.79.1\r\n\
          Accept: */*\r\n\
          Content-Type: application/json\r\n\
          Content-Length: {}\r\n\r\n\
          {}",
          url, host, post_body.len(), post_body 
        );
        simple_tls_client(host, &req, port.try_into().unwrap())
      },
      "demo_get" => {
        let host = query_param["host"].as_str().unwrap_or("");
        let port = query_param["port"].as_i64().unwrap_or(443);
        let url = query_param["url"].as_str().unwrap_or("/");
        let req = format!(
          "GET {} HTTP/1.1\r\n\
          HOST: {}\r\n\
          User-Agent: curl/7.79.1\r\n\
          Accept: */*\r\n\r\n", 
          url, host 
        );
        simple_tls_client(host, &req, port.try_into().unwrap())
      },
      _ => {
        Err(NetworkError::String(format!("Unexpected query_type: {:?}", query_type)))
      }
    }
  }
}