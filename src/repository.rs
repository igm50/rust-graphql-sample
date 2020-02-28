use mysql::{error::Error, from_row};
use uuid::Uuid;

use crate::entity::todo::{Repository, ToDo};

pub struct Connection {
  pool: mysql::Pool,
}

impl Connection {
  pub fn new() -> Connection {
    let connection = Connection {
      pool: mysql::Pool::new("mysql://root:password@database:3306/mysql").unwrap(),
    };

    connection.init_table();
    connection
  }

  fn init_table(&self) {
    self
      .pool
      .prep_exec(r"CREATE SCHEMA IF NOT EXISTS todo", ())
      .unwrap();

    self
      .pool
      .prep_exec(r"DROP TABLE IF EXISTS todo.todos", ())
      .unwrap();

    // todo: created_atカラムの追加
    self
      .pool
      .prep_exec(
        r"CREATE TABLE todo.todos (
            id BINARY(32) NOT NULL,
            text VARCHAR(1000) NULL,
            PRIMARY KEY (id)
          )",
        (),
      )
      .unwrap();
  }
}

impl juniper::Context for Connection {}

impl Repository<Error> for Connection {
  fn list(&self) -> Result<Vec<ToDo>, Error> {
    let mut stmt = self
      .pool
      .prepare(r"SELECT id, text FROM todo.todos ORDER BY id")
      .unwrap();

    let mut result = Vec::new();
    for row in stmt.execute(()).unwrap() {
      let (id, text) = from_row::<(String, String)>(row.unwrap());
      result.push(ToDo::new(Uuid::parse_str(id.as_str()).unwrap(), text));
    }

    Ok(result)
  }

  fn create(&self, todo: ToDo) -> Result<ToDo, Error> {
    let mut stmt = self
      .pool
      .prepare(r"INSERT INTO todo.todos (id, text) VALUES (?, ?)")
      .unwrap();

    let result = stmt.execute((todo.id(), todo.text()));

    match result {
      Ok(_t) => Ok(todo),
      Err(e) => Err(e),
    }
  }
}
