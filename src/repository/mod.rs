use mysql::from_row;

mod error;

use crate::entity::todo::{Repository, Todo, TodoId};
pub use error::Error;

pub struct Connection {
  pool: mysql::Pool,
}

impl Connection {
  pub fn new() -> Connection {
    Connection {
      pool: mysql::Pool::new("mysql://root:password@database:3306/mysql").unwrap(),
    }
  }

  pub fn init_table(&self) {
    let pool = &self.pool;

    pool
      .prep_exec(r"CREATE SCHEMA IF NOT EXISTS todo", ())
      .unwrap();

    pool
      .prep_exec(r"DROP TABLE IF EXISTS todo.todos", ())
      .unwrap();

    pool
      .prep_exec(
        r"CREATE TABLE todo.todos (
            id BINARY(32) NOT NULL,
            text VARCHAR(1000) NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
          )",
        (),
      )
      .unwrap();
  }
}

impl juniper::Context for Connection {}

impl Repository for Connection {
  fn list(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let mut stmt = self
      .pool
      .prepare(r"SELECT id, text FROM todo.todos ORDER BY created_at, id")
      .unwrap();

    let rows = stmt.execute(())?;

    let mut todos = Vec::new();
    for row in rows {
      let (id, text) = from_row::<(String, String)>(row.unwrap());
      todos.push(Todo::try_parse(id, text).unwrap());
    }

    Ok(todos)
  }

  fn fetch(&self, id: TodoId) -> Result<Todo, Box<dyn std::error::Error>> {
    let mut stmt = self
      .pool
      .prepare(r"SELECT id, text FROM todo.todos WHERE id = ?")
      .unwrap();

    let rows = stmt.execute((id.to_string(),))?;

    let mut todos = Vec::new();
    for row in rows {
      let (id, text) = from_row::<(String, String)>(row.unwrap());
      todos.push(Todo::try_parse(id, text).unwrap());
    }

    match todos.len() {
      1 => Ok(todos[0].clone()),
      _ => Err(Box::new(Error::NotFound)),
    }
  }

  fn create(&self, todo: Todo) -> Result<Todo, Box<dyn std::error::Error>> {
    let mut stmt = self
      .pool
      .prepare(r"INSERT INTO todo.todos (id, text) VALUES (?, ?)")
      .unwrap();

    stmt.execute((todo.id(), todo.text()))?;

    Ok(todo)
  }
}
