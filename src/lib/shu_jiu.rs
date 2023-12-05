use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct ShuJiu {
  __name: String,
  __index: i64,
}

pub type ShuJiuRef = Arc<Mutex<ShuJiu>>;

pub trait ShuJiuRefHelper {
  fn get_name(&self) -> String;
  fn set_name(&self, name: &str);
  fn get_index(&self) -> i64;
  fn set_index(&self, index: i64);
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

trait __ShuJiuRefHelper {}

impl ShuJiu {
  pub fn new(name: &str, index: i64) -> ShuJiuRef {
    Arc::new(Mutex::new(Self {
      __name: name.to_string(),
      __index: index,
    }))
  }
}

impl ShuJiuRefHelper for ShuJiuRef {
  fn get_name(&self) -> String {
    self.lock().unwrap().__name.clone()
  }

  fn set_name(&self, name: &str) {
    self.lock().unwrap().__name = name.to_string();
  }

  fn get_index(&self) -> i64 {
    self.lock().unwrap().__index
  }

  fn set_index(&self, index: i64) {
    self.lock().unwrap().__index = index;
  }

  fn to_string(&self) -> String {
    self.lock().unwrap().__name.clone()
  }

  fn to_full_string(&self) -> String {
    let (name, index) = {
      let s = self.lock().unwrap();
      (s.__name.clone(), s.__index)
    };
    format!("{}第{}天", name, index)
  }
}
