use juniper::FieldResult;
use juniper::RootNode;

use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "Sample query")]
struct Sample {
  name: String,
}

pub struct QueryRoot;
pub struct MutationRoot;

#[juniper::object]
impl QueryRoot {
  fn sample() -> FieldResult<Sample> {
    Ok(Sample {
      name: String::from("sample query"),
    })
  }
}

#[juniper::object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
