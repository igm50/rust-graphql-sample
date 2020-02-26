use juniper::graphql_object;
use uuid::Uuid;

#[derive(Debug)]
pub struct ToDo {
  id: Uuid,
  text: String,
}

impl ToDo {
  pub fn new(id: Uuid, text: String) -> ToDo {
    ToDo { id: id, text: text }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn text(&self) -> String {
    self.text.clone()
  }
}

impl PartialEq for ToDo {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

graphql_object!(ToDo: () |&self| {
  description: "Task to do."

  field id() -> String {
    self.id.to_string()
  }

  field text() -> &String {
    &self.text
  }
});

pub trait Repository<E> {
  fn create(&self, todo: ToDo) -> Result<ToDo, E>;
}

#[cfg(test)]
mod test {
  use super::*;
  use uuid::Uuid;

  #[test]
  fn test_id() {
    let todo = ToDo {
      id: Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
    };

    assert_eq!(
      todo.id().to_string(),
      "97103fab-1e50-36b7-0c03-0938362b0809"
    );
    assert_ne!(
      todo.id().to_string(),
      "aaaaaaaa-1e50-36b7-0c03-0938362b0809"
    );
  }

  #[test]
  fn test_text() {
    let todo = ToDo {
      id: Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
    };

    assert_eq!(todo.text(), "sample");
    assert_ne!(todo.text(), "not equal");
  }

  #[test]
  fn test_partial_eq() {
    let id = Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap();

    assert_eq!(
      ToDo {
        id: id,
        text: String::from("one")
      },
      ToDo {
        id: id,
        text: String::from("other")
      }
    );

    assert_ne!(
      ToDo {
        id: id,
        text: String::from("one")
      },
      ToDo {
        id: Uuid::parse_str("aaaaaaaa-1e50-36b7-0c03-0938362b0809").unwrap(),
        text: String::from("one")
      }
    );
  }
}
