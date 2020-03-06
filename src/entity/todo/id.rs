use uuid::Uuid;

use crate::entity::todo::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TodoId(Uuid);

impl TodoId {
  pub fn new_random() -> Self {
    Self(Uuid::new_v4())
  }

  pub fn parse_str(input: &str) -> Result<TodoId, Error> {
    let id = Uuid::parse_str(input);
    match id {
      Ok(id) => Ok(Self(id)),
      Err(e) => Err(Error::new(e)),
    }
  }
}

impl ToString for TodoId {
  fn to_string(&self) -> String {
    self.0.to_simple().to_string()
  }
}

// test
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_create() {
    assert!(TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").is_ok());
    assert!(TodoId::parse_str("invalid_strings").is_err());
  }
}
