use crate::util::locked_ref_trait::LockRef;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Fu {
  __name: String,
  __index: i64,
}

pub type FuRef = Arc<Mutex<Fu>>;
pub trait FuRefHelper: LockRef {
  fn get_name(&self) -> String;
  fn set_name(&self, name: &str);
  fn get_index(&self) -> i64;
  fn set_index(&self, index: i64);
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

impl LockRef for FuRef {
  type Output<'a> = MutexGuard<'a, Fu>;

  fn as_locked_ref<'a>(&'a self) -> MutexGuard<'a, Fu> {
    self.lock().unwrap()
  }
}

impl FuRefHelper for FuRef {
  fn get_name(&self) -> String {
    self.as_locked_ref().__name.clone()
  }

  fn set_name(&self, name: &str) {
    self.as_locked_ref().__name = name.to_string()
  }

  fn get_index(&self) -> i64 {
    self.as_locked_ref().__index
  }

  fn set_index(&self, index: i64) {
    self.as_locked_ref().__index = index
  }

  fn to_string(&self) -> String {
    self.as_locked_ref().__name.clone()
  }

  fn to_full_string(&self) -> String {
    let (name, index) = {
      let s = self.as_locked_ref();
      (s.__name.clone(), s.__index)
    };
    format!("{}第{}天", name, index)
  }
}

impl Fu {
  pub fn new(name: &str, index: i64) -> FuRef {
    Arc::new(Mutex::new(Self {
      __name: name.to_string(),
      __index: index,
    }))
  }
}
