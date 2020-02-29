use juniper::{FieldError, FieldResult, RootNode, Value};
use std::error::Error;
use std::sync::Arc;

use crate::entity::todo::{Repository, Todo};

pub struct QueryRoot<E>
where
  E: Error,
{
  repository: Arc<dyn Repository<E>>,
}

#[juniper::object]
impl<E> QueryRoot<E>
where
  E: Error,
{
  fn todos(&self) -> FieldResult<Vec<Todo>> {
    match self.repository.list() {
      Ok(todos) => Ok(todos),
      Err(e) => Err(FieldError::new(String::from(e.description()), Value::Null)),
    }
  }
}

pub struct MutationRoot<E>
where
  E: Error,
{
  repository: Arc<dyn Repository<E>>,
}

#[juniper::object]
impl<E> MutationRoot<E>
where
  E: Error,
{
  fn register(&self, text: String) -> FieldResult<Todo> {
    let todo = Todo::new_random_id(text);
    match self.repository.create(todo) {
      Ok(todo) => Ok(todo),
      Err(e) => Err(FieldError::new(String::from(e.description()), Value::Null)),
    }
  }
}

pub type Schema<E> = RootNode<'static, QueryRoot<E>, MutationRoot<E>>;

pub fn create_schema<E>(repository: Arc<dyn Repository<E>>) -> Schema<E>
where
  E: Error,
{
  Schema::new(
    QueryRoot::<E> {
      repository: repository.clone(),
    },
    MutationRoot::<E> {
      repository: repository.clone(),
    },
  )
}
