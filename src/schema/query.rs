use juniper::{graphql_value, FieldError, FieldResult, Value};
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::todo::{Repository, Todo};

pub struct Query<E>
where
  E: Error,
{
  repository: Arc<dyn Repository<E>>,
}

#[juniper::object]
impl<E> Query<E>
where
  E: Error,
{
  fn todos(&self) -> FieldResult<Vec<Todo>> {
    match self.repository.list() {
      Ok(todos) => Ok(todos),
      Err(e) => Err(build_error(e.description(), Value::Null)),
    }
  }

  fn todo(&self, id_str: String) -> FieldResult<Todo> {
    let error = |description| build_error(description, graphql_value!({ "arg_id": id_str }));
    let id = Uuid::parse_str(id_str.as_str());

    if let Err(e) = id {
      return Err(error("Invalid uuid format"));
    }

    match &self.repository.fetch(id.unwrap()) {
      Ok(todo) => Ok(*todo),
      Err(e) => Err(error(e.description())),
    }
  }
}

fn build_error(message: &str, value: Value) -> FieldError {
  FieldError::new(String::from(message), value)
}
