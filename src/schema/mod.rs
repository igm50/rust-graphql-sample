use juniper::RootNode;
use std::sync::Arc;

mod mutation;
mod query;
mod todo;

use crate::entity::todo::Repository;
use mutation::Mutation;
use query::Query;

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema(repository: Arc<dyn Repository>) -> Schema {
  Schema::new(
    Query::new(repository.clone()),
    Mutation::new(repository.clone()),
  )
}
