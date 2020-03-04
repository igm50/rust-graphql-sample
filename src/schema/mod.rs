use juniper::{FieldError, FieldResult, RootNode, Value};
use std::sync::Arc;

use crate::entity::todo::{Repository, Todo};

pub struct QueryRoot {
  repository: Arc<dyn Repository>,
}

#[juniper::object]
impl QueryRoot {
  fn todos(&self) -> FieldResult<Vec<Todo>> {
    match self.repository.list() {
      Ok(todos) => Ok(todos),
      Err(e) => Err(FieldError::new(String::from(e.description()), Value::Null)),
    }
  }
}

pub struct MutationRoot {
  repository: Arc<dyn Repository>,
}

#[juniper::object]
impl MutationRoot {
  fn register(&self, text: String) -> FieldResult<Todo> {
    let todo = Todo::new_random_id(text);
    match self.repository.create(todo) {
      Ok(todo) => Ok(todo),
      Err(e) => Err(FieldError::new(String::from(e.description()), Value::Null)),
    }
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema(repository: Arc<dyn Repository>) -> Schema {
  Schema::new(
    QueryRoot {
      repository: repository.clone(),
    },
    MutationRoot {
      repository: repository.clone(),
    },
  )
}
