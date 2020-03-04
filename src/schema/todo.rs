use crate::entity::todo::{Todo, TodoId};

#[juniper::object]
impl Todo {
  fn id(&self) -> String {
    self.id()
  }

  fn text(&self) -> &String {
    self.text()
  }
}

#[juniper::object]
impl TodoId {
  fn id(&self) -> String {
    self.to_string()
  }
}
