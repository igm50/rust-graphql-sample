use mysql::{error::Error, from_row};
use uuid::Uuid;

use crate::entity::todo::{Repository, Todo};

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

impl Repository<Error> for Connection {
  fn list(&self) -> Result<Vec<Todo>, Error> {
    let mut stmt = self
      .pool
      .prepare(r"SELECT id, text FROM todo.todos ORDER BY created_at, id")
      .unwrap();

    let rows_result = stmt.execute(());

    match rows_result {
      Err(e) => Err(e),
      Ok(rows) => {
        let mut todos = Vec::new();
        for row in rows {
          let (id, text) = from_row::<(String, String)>(row.unwrap());
          todos.push(Todo::new(Uuid::parse_str(id.as_str()).unwrap(), text));
        }

        Ok(todos)
      }
    }
  }

  fn fetch(&self, id: Uuid) -> Result<Todo, Error> {
    let mut stmt = self
      .pool
      .prepare(r"SELECT id, text FROM todo.todos WHERE id = ?")
      .unwrap();

    Ok(Todo::new_random_id(String::from("test")))
    // let rows_result = stmt.execute((id.));

    // match rows_result {
    //   Err(e) => Err(e),
    //   Ok(rows) => {
    //     let mut todos = Vec::new();
    //     for row in rows {
    //       let (id, text) = from_row::<(String, String)>(row.unwrap());
    //       todos.push(Todo::new(Uuid::parse_str(id.as_str()).unwrap(), text));
    //     }

    //     Ok(todos)
    //   }
    // }
  }

  fn create(&self, todo: Todo) -> Result<Todo, Error> {
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
