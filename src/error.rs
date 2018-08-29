use std::fmt;
use std::error;

macro_rules! error {
  ($($arg:tt)*) => (::Error::new(format!($($arg)*)));
}

#[derive(Debug)]
pub struct Error {
  msg: String,
}

impl Error {
  pub fn new(msg: String) -> Error {
    Error { msg }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    &self.msg
  }
}
