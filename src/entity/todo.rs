use uuid::Uuid;

#[derive(Debug)]
pub struct ToDo {
  id: Uuid,
  text: String,
}

impl PartialEq for ToDo {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use uuid::Uuid;

  #[test]
  fn partial_eq() {
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
  }
}
