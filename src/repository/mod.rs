use chrono::NaiveDateTime;
use mysql::prelude::Queryable;
use mysql::{from_row, PooledConn};

mod error;

use crate::entity::todo::{BoxedError, Repository, Todo, TodoId};
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

  pub fn conn(&self) -> mysql::Result<PooledConn> {
    self.pool.get_conn()
  }

  pub fn init_table(&self) -> mysql::Result<()> {
    let mut conn = self.conn()?;

    conn.query_drop(r"CREATE SCHEMA IF NOT EXISTS todo")?;
    conn.query_drop(r"DROP TABLE IF EXISTS todo.todos")?;
    conn.query_drop(
      r"CREATE TABLE todo.todos (
            id BINARY(32) NOT NULL,
            text VARCHAR(1000) NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
          )",
    )?;

    Ok(())
  }
}

impl Repository for Connection {
  fn list(&self) -> Result<Vec<Todo>, BoxedError> {
    let todos = self.conn()?.query_map(
      r"SELECT id, text, created_at FROM todo.todos ORDER BY created_at, id",
      |(id, text, created_at): (String, String, NaiveDateTime)| {
        Todo::try_parse(id.as_str(), text.as_str(), created_at).unwrap()
      },
    )?;

    Ok(todos)
  }

  fn fetch(&self, id: &TodoId) -> Result<Todo, BoxedError> {
    let row = self.conn()?.exec_first(
      r"SELECT id, text, created_at FROM todo.todos WHERE id = ?",
      (id.to_string(),),
    )?;

    match row {
      None => Err(Box::new(Error::NotFound)),
      Some(r) => {
        let (id, text, created_at) = from_row::<(String, String, NaiveDateTime)>(r);
        Ok(Todo::try_parse(id.as_str(), text.as_str(), created_at).unwrap())
      }
    }
  }

  fn create(&self, todo: &Todo) -> Result<(), BoxedError> {
    self.conn()?.exec_drop(
      r"INSERT INTO todo.todos (id, text, created_at) VALUES (?, ?, ?)",
      (todo.id(), todo.text(), todo.created_at()),
    )?;

    Ok(())
  }

  fn update(&self, id: &TodoId, text: &str) -> Result<Todo, BoxedError> {
    self.conn()?.exec_drop(
      r"UPDATE todo.todos SET text = ? WHERE id = ?",
      (text, id.to_string()),
    )?;

    Ok(self.fetch(id)?)
  }

  fn delete(&self, id: &TodoId) -> Result<(), BoxedError> {
    self
      .conn()?
      .exec_drop(r"DELETE FROM todo.todos WHERE id = ?", (id.to_string(),))?;

    Ok(())
  }
}
