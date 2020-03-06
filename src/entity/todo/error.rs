use std::fmt;

#[derive(Debug)]
pub enum Error {
  ParseError(uuid::Error),
}

impl Error {
  pub fn new(error: uuid::Error) -> Self {
    Self::ParseError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ParseError(e) => write!(f, "UUID parse error. {}", e),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::ParseError(e) => Some(e),
    }
  }
}
