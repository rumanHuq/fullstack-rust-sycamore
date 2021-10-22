#[derive(Debug, Default)]
pub struct AppState {
  pub counter: i32,
}

impl AppState {
  pub fn incr(&self) -> Self {
    Self {
      counter: &self.counter + 1,
    }
  }
  pub fn decr(&self) -> Self {
    Self {
      counter: &self.counter - 1,
    }
  }
  pub fn reset(&self) -> Self {
    Self { counter: 0 }
  }
}
