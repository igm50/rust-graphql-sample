use std::fmt;

#[derive(Debug)]
pub enum Error {
  NotFound,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NotFound => write!(f, "Not found."),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::NotFound => None,
    }
  }
}
