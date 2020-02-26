use juniper::FieldResult;
use juniper::RootNode;
use uuid::Uuid;

use crate::entity::todo::ToDo;

pub struct QueryRoot;
pub struct MutationRoot;

#[juniper::object]
impl QueryRoot {
  fn todos() -> FieldResult<Vec<ToDo>> {
    let result = vec![
      ToDo::new(
        Uuid::parse_str("87103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
        String::from("sample A"),
      ),
      ToDo::new(
        Uuid::parse_str("97103fab-2e50-46b7-1c03-1938362b0809").unwrap(),
        String::from("sample B"),
      ),
    ];

    Ok(result)
  }

  fn todo(text: String) -> FieldResult<ToDo> {
    Ok(ToDo::new(
      Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text,
    ))
  }
}

#[juniper::object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
