use juniper::{graphql_value, FieldError, FieldResult};
use std::sync::Arc;

use crate::entity::todo::{Repository, Todo, TodoId};

pub struct Mutation {
  repository: Arc<dyn Repository>,
}

impl Mutation {
  pub fn new(repository: Arc<dyn Repository>) -> Self {
    Self { repository }
  }

  fn register(&self, text: String) -> FieldResult<Todo> {
    let todo = Todo::new_random_id(text.as_str());
    match self.repository.create(&todo) {
      Ok(_) => Ok(todo),
      Err(e) => Err(FieldError::new(
        String::from(e.description()),
        graphql_value!({ "text": text }),
      )),
    }
  }

  fn update(&self, id: String, text: String) -> FieldResult<Todo> {
    let op_todo = Todo::try_parse(id.as_str(), text.as_str());
    let error = |description| {
      Err(FieldError::new(
        description,
        graphql_value!({ "id": id, "text": text }),
      ))
    };

    if let Err(e) = op_todo {
      return error(&format!("{}", e));
    }

    let todo = op_todo.unwrap();
    match self.repository.update(&todo) {
      Ok(_) => Ok(todo),
      Err(e) => error(&format!("{}", e)),
    }
  }

  fn delete(&self, id_str: String) -> FieldResult<TodoId> {
    let op_id = TodoId::parse_str(id_str.as_str());
    let error = |description| {
      Err(FieldError::new(
        description,
        graphql_value!({ "id": id_str }),
      ))
    };

    if let Err(e) = op_id {
      return error(&format!("{}", e));
    }

    let id = op_id.unwrap();
    match self.repository.delete(&id) {
      Ok(_) => Ok(id),
      Err(e) => error(&format!("{}", e)),
    }
  }
}

#[juniper::object]
impl Mutation {
  fn register(&self, text: String) -> FieldResult<Todo> {
    self.register(text)
  }

  fn update(&self, id: String, text: String) -> FieldResult<Todo> {
    self.update(id, text)
  }

  fn delete(&self, id: String) -> FieldResult<TodoId> {
    self.delete(id)
  }
}
