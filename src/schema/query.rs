use juniper::{graphql_value, FieldError, FieldResult, Value};
use std::sync::Arc;

use crate::entity::todo::{Repository, Todo, TodoId};

pub struct Query {
  repository: Arc<dyn Repository>,
}

impl Query {
  pub fn new(repository: Arc<dyn Repository>) -> Self {
    Self { repository }
  }

  fn todos(&self) -> FieldResult<Vec<Todo>> {
    match self.repository.list() {
      Ok(todos) => Ok(todos),
      Err(e) => Err(FieldError::new(&format!("{}", e), Value::Null)),
    }
  }

  fn todo(&self, id_str: String) -> FieldResult<Todo> {
    let id = TodoId::parse_str(id_str.as_str());
    let error = |description| {
      Err(FieldError::new(
        description,
        graphql_value!({ "id": id_str }),
      ))
    };

    if let Err(e) = id {
      return error(&format!("{}", e));
    }

    let result = self.repository.fetch(&id.unwrap());
    match result {
      Ok(todo) => Ok(todo),
      Err(e) => error(&format!("{}", e)),
    }
  }
}

#[juniper::object]
impl Query {
  fn todos(&self) -> FieldResult<Vec<Todo>> {
    self.todos()
  }

  fn todo(&self, id: String) -> FieldResult<Todo> {
    self.todo(id)
  }
}
