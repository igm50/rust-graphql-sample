use juniper::{FieldError, FieldResult, RootNode, Value};
use std::sync::Arc;

mod query;

use crate::entity::todo::{Repository, Todo};
use query::Query;

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

pub type Schema = RootNode<'static, Query, MutationRoot>;

pub fn create_schema(repository: Arc<dyn Repository>) -> Schema {
  Schema::new(
    Query::new(repository.clone()),
    MutationRoot {
      repository: repository.clone(),
    },
  )
}
