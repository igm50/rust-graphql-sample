pub struct Connection {
  pool: mysql::Pool,
}

impl Connection {
  pub fn connect() -> Connection {
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

    self
      .pool
      .prep_exec(
        r"CREATE TABLE todo.todos (
            id INT NOT NULL,
            text VARCHAR(1000) NULL,
            PRIMARY KEY (id)
          )",
        (),
      )
      .unwrap();
  }
}
