use crate::error::CustomError;
use std::fs;

#[derive(Debug)]
pub struct ReadConfigQuery {
  pub json: String,
}

impl ReadConfigQuery {
  pub fn new() -> Result<ReadConfigQuery, CustomError> {
    let json: Result<String, std::io::Error> = fs::read_to_string("query.json");
    if json.is_err() {
      let err = CustomError::new("Error reading file");
      return Err(err);
    }
    let json = json.unwrap();
    Ok(ReadConfigQuery { json })
  }
}
