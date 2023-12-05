use crate::util::locked_ref_trait::LockRef;
use std::sync::{Arc, Mutex, MutexGuard};

pub fn new_1(name: &str) -> TaoFestivalRef {
  new_2(name, "")
}

pub fn new_2(name: &str, remark: &str) -> TaoFestivalRef {
  Arc::new(Mutex::new(TaoFestival {
    __name: name.to_string(),
    __remark: remark.to_string(),
  }))
}

#[derive(Clone, Debug)]
pub struct TaoFestival {
  __name: String,
  __remark: String,
}

pub type TaoFestivalRef = Arc<Mutex<TaoFestival>>;

pub trait TaoFestivalRefHelper: LockRef {
  fn get_name(&self) -> String;
  fn get_remark(&self) -> String;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

impl TaoFestivalRefHelper for TaoFestivalRef {
  fn get_name(&self) -> String {
    self.as_locked_ref().__name.clone()
  }

  fn get_remark(&self) -> String {
    self.as_locked_ref().__remark.clone()
  }

  fn to_string(&self) -> String {
    self.get_name()
  }

  fn to_full_string(&self) -> String {
    let mut s = self.get_name();
    let remark = self.get_remark();
    if remark.len() > 0 {
      s = format!("{}[{}]", s, remark);
    }
    s
  }
}

impl LockRef for TaoFestivalRef {
  type Output<'a> = MutexGuard<'a, TaoFestival>;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
