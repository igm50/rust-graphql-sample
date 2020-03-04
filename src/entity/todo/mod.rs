use std::marker::{Send, Sync};

pub mod error;
pub mod id;

pub use error::Error;
pub use id::TodoId;

#[derive(Debug)]
pub struct Todo {
  id: TodoId,
  text: String,
}

impl Todo {
  pub fn new(id: TodoId, text: String) -> Self {
    Self { id: id, text: text }
  }

  pub fn new_random_id(text: String) -> Self {
    Self {
      id: TodoId::new_random(),
      text,
    }
  }

  pub fn try_parse(id_str: String, text: String) -> Result<Self, Error> {
    let id = TodoId::parse_str(id_str.as_str());
    match id {
      Ok(id) => Ok(Self::new(id, text)),
      Err(e) => Err(e),
    }
  }

  pub fn id(&self) -> String {
    self.id.to_string()
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

impl PartialEq for Todo {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

pub trait Repository: Sync + Send {
  fn list(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>>;
  fn create(&self, todo: Todo) -> Result<Todo, Box<dyn std::error::Error>>;
}

// tests
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_try_parse() {
    assert!(Todo::try_parse(
      String::from("97103fab-1e50-36b7-0c03-0938362b0809"),
      String::from("sample")
    )
    .is_ok());
    assert!(Todo::try_parse(String::from("invalid"), String::from("sample")).is_err())
  }

  #[test]
  fn test_id() {
    let todo = Todo {
      id: TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
    };

    assert_eq!(todo.id(), "97103fab1e5036b70c030938362b0809");
    assert_ne!(todo.id(), "97103fab-1e50-36b7-0c03-0938362b0809");
  }

  #[test]
  fn test_text() {
    let todo = Todo {
      id: TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
    };

    assert_eq!(todo.text(), &String::from("sample"));
    assert_ne!(todo.text(), &String::from("not equal"));
  }

  #[test]
  fn test_partial_eq() {
    let id = TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap();

    assert_eq!(
      Todo {
        id: id.clone(),
        text: String::from("one")
      },
      Todo {
        id: id.clone(),
        text: String::from("other")
      }
    );

    assert_ne!(
      Todo {
        id: id.clone(),
        text: String::from("one")
      },
      Todo {
        id: TodoId::parse_str("aaaaaaaa-1e50-36b7-0c03-0938362b0809").unwrap(),
        text: String::from("one")
      }
    );
  }
}
