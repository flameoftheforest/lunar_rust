use std::ops::{self};

#[derive(Clone, Debug)]
pub struct StringX(String);

impl StringX {
  pub fn new(s: &str) -> Self {
    Self(s.to_string())
  }
  pub fn get(&self) -> String {
    self.0.clone()
  }
}

impl ops::Add<String> for StringX {
  type Output = StringX;
  fn add(self, _rhs: String) -> StringX {
    Self::new(&format!("{}{}", self.0, _rhs))
  }
}

impl ops::Add<&str> for StringX {
  type Output = StringX;
  fn add(self, _rhs: &str) -> StringX {
    Self::new(&format!("{}{}", self.0, _rhs))
  }
}

impl ops::Add<i64> for StringX {
  type Output = StringX;
  fn add(self, _rhs: i64) -> StringX {
    Self::new(&format!("{}{}", self.0, _rhs))
  }
}

impl ops::Add<f64> for StringX {
  type Output = StringX;
  fn add(self, _rhs: f64) -> StringX {
    Self::new(&format!("{}{}", self.0, _rhs))
  }
}
