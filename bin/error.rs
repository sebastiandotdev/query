use core::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug)]
pub struct CustomError {
  message: String,
}

impl CustomError {
  pub fn new(message: &str) -> Self {
    Self {
      message: String::from(message),
    }
  }
}

impl Error for CustomError {}

impl Display for CustomError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "Error: {}", self.message)
  }
}

#[test]
fn test_error() {
  let err = CustomError::new("Error message test");
  let formatted_err = format!("{}", err);

  assert_eq!(formatted_err, "Error: Error message test");
}
