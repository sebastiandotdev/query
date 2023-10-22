use crate::error::CustomError;
use std::fs;

pub struct ReadConfigFetchy {
  pub json: String,
}

impl ReadConfigFetchy {
  pub fn new() -> Result<ReadConfigFetchy, CustomError> {
    let json = fs::read_to_string("fetchy.json");
    if json.is_err() {
      let err = CustomError::new("Error reading file");
      return Err(err);
    }
    let json = json.unwrap();
    Ok(ReadConfigFetchy { json })
  }
}
