use crate::entity::todo::ToDo;

#[juniper::object]
impl ToDo {
  fn id(&self) -> String {
    self.id()
  }

  fn text(&self) -> &String {
    self.text()
  }
}
