use chrono::prelude::{Local, NaiveDateTime};
use std::marker::{Send, Sync};

pub mod error;
pub mod id;

pub use error::Error;
pub use id::TodoId;

#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
  id: TodoId,
  text: String,
  created_at: NaiveDateTime,
}

impl Todo {
  pub fn new(id: TodoId, text: &str, created_at: NaiveDateTime) -> Self {
    Self {
      id,
      text: String::from(text),
      created_at,
    }
  }

  pub fn new_random_id(text: &str) -> Self {
    Self::new(TodoId::new_random(), text, Local::now().naive_local())
  }

  pub fn try_parse(id_str: &str, text: &str, created_at: NaiveDateTime) -> Result<Self, Error> {
    Ok(Self::new(TodoId::parse_str(id_str)?, text, created_at))
  }

  pub fn id(&self) -> String {
    self.id.to_string()
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn created_at(&self) -> &NaiveDateTime {
    &self.created_at
  }
}

pub type BoxedError = Box<dyn std::error::Error>;

pub trait Repository: Sync + Send {
  fn list(&self) -> Result<Vec<Todo>, BoxedError>;
  fn fetch(&self, id: &TodoId) -> Result<Todo, BoxedError>;
  fn create(&self, todo: &Todo) -> Result<(), BoxedError>;
  fn update(&self, id: &TodoId, text: &str) -> Result<Todo, BoxedError>;
  fn delete(&self, id: &TodoId) -> Result<(), BoxedError>;
}

// tests
#[cfg(test)]
mod test {
  use super::*;
  use chrono::NaiveDate;

  #[test]
  fn test_try_parse() {
    let try_parse = |id| Todo::try_parse(id, "sample", Local::now().naive_local());

    assert!(try_parse("97103fab1e5036b70c030938362b0809").is_ok());
    assert!(try_parse("97103fab-1e50-36b7-0c03-0938362b0809").is_ok());

    assert!(try_parse("invalid").is_err());
  }

  #[test]
  fn test_id() {
    let todo = Todo {
      id: TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
      created_at: Local::now().naive_local(),
    };

    assert_eq!(todo.id(), "97103fab1e5036b70c030938362b0809");
    assert_ne!(todo.id(), "97103fab-1e50-36b7-0c03-0938362b0809");
  }

  #[test]
  fn test_text() {
    let todo = Todo {
      id: TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
      created_at: Local::now().naive_local(),
    };

    assert_eq!(todo.text(), &String::from("sample"));
    assert_ne!(todo.text(), &String::from("not equal"));
  }

  #[test]
  fn test_created_at() {
    let todo = Todo {
      id: TodoId::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
      created_at: NaiveDate::from_ymd(2020, 4, 1).and_hms(12, 10, 30),
    };

    assert_eq!(
      todo.created_at(),
      &NaiveDate::from_ymd(2020, 4, 1).and_hms(12, 10, 30)
    );
    assert_ne!(
      todo.created_at(),
      &NaiveDate::from_ymd(2019, 5, 10).and_hms(1, 20, 5)
    );
  }

  #[test]
  fn test_partial_eq() {
    let new = |(id, text, y, m, d): (&str, &str, i32, u32, u32)| Todo {
      id: TodoId::parse_str(id).unwrap(),
      text: String::from(text),
      created_at: NaiveDate::from_ymd(y, m, d).and_hms(12, 10, 30),
    };

    assert_eq!(
      new(("97103fab-1e50-36b7-0c03-0938362b0809", "A", 2020, 4, 1)),
      new(("97103fab-1e50-36b7-0c03-0938362b0809", "A", 2020, 4, 1)),
    );

    assert_ne!(
      new(("97103fab-1e50-36b7-0c03-0938362b0809", "A", 2020, 4, 1)),
      new(("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", "A", 2020, 4, 1)),
    );
    assert_ne!(
      new(("97103fab-1e50-36b7-0c03-0938362b0809", "A", 2020, 4, 1)),
      new(("97103fab-1e50-36b7-0c03-0938362b0809", "B", 2020, 4, 1)),
    );
    assert_ne!(
      new(("97103fab-1e50-36b7-0c03-0938362b0809", "A", 2020, 4, 1)),
      new(("aaaaaaaa-1e50-36b7-0c03-0938362b0809", "A", 2019, 5, 8)),
    );
  }
}
