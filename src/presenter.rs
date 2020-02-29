use crate::entity::todo::Todo;

#[juniper::object]
impl Todo {
  fn id(&self) -> String {
    self.id()
  }

  fn text(&self) -> &String {
    self.text()
  }
}
