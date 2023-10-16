use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CustomError {
  message: String,
}

impl CustomError {
  pub fn new(message: &str) -> CustomError {
    CustomError {
      message: String::from(message),
    }
  }
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Error: {} \n", self.message)
  }
}
