extern crate reqwest;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ApiError{
    pub code: u16,
    pub message: String
}


impl From<reqwest::Error> for ApiError {

    fn from(err: reqwest::Error) -> Self {
        let c = match err.status(){
            Some(v) => v.as_u16(),
            None => 0
        };
        ApiError {
            code: c,
            message: err.description().to_string()
        }
    }
}

impl ApiError {
    pub fn new(code: u16, message: String) -> ApiError {
        ApiError {
            code: code, message: message
        }
    }
}


impl<'a> fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message :String = match serde_json::from_str::<serde_json::Value>(&self.message) {
            Ok(v) => {
                let errors: Vec<HashMap<String, String>> = serde_json::from_value(v["errors"].clone()).unwrap();
                errors[0]["message"].clone()
            },
            Err(_) => {
                self.message.to_owned()
            }
        };
        writeln!(
            f, "ERROR Code : {}, {}", self.code, message
        )
    }
}
