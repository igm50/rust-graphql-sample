#[get("/")]
pub fn index() -> &'static str {
  "Hello, world!"
}

#[cfg(test)]
mod test {
  use super::index;

  #[test]
  fn index_test() {
    assert_eq!(index(), "Hello, world!");
    assert_ne!(index(), "Good bye, world!");
  }
}
