use std::sync::{Arc, Mutex, MutexGuard};

use crate::util::locked_ref_trait::LockRef;

#[derive(Clone, Debug)]
pub struct FotoFestival {
  __name: String,
  __result: String,
  __every_month: bool,
  __remark: String,
}

pub type FotoFestivalRef = Arc<Mutex<FotoFestival>>;

pub trait FotoFestivalRefHelper: LockRef {
  fn get_name(&self) -> String;
  fn get_result(&self) -> String;
  fn is_every_month(&self) -> bool;
  fn get_remark(&self) -> String;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

impl FotoFestivalRefHelper for FotoFestivalRef {
  fn get_name(&self) -> String {
    self.as_locked_ref().__name.clone()
  }

  fn get_result(&self) -> String {
    self.as_locked_ref().__result.clone()
  }

  fn is_every_month(&self) -> bool {
    self.as_locked_ref().__every_month
  }

  fn get_remark(&self) -> String {
    self.as_locked_ref().__remark.clone()
  }

  fn to_string(&self) -> String {
    self.as_locked_ref().__name.clone()
  }

  fn to_full_string(&self) -> String {
    let mut s = self.get_name();
    let result = self.get_result();
    s = format!(
      "{}{}",
      s,
      match result.len() > 0 {
        true => format!(" {}", result),
        _ => format!(""),
      }
    );
    let remark = self.get_remark();
    s = format!(
      "{}{}",
      s,
      match remark.len() > 0 {
        true => format!(" {}", remark),
        _ => format!(""),
      }
    );
    s
  }
}

impl FotoFestival {
  pub fn new_1(name: &str) -> FotoFestivalRef {
    Self::new_4(name, "", false, "")
  }

  pub fn new_2(name: &str, result: &str) -> FotoFestivalRef {
    Self::new_4(name, result, false, "")
  }

  pub fn new_3(
    name: &str,
    result: &str,
    every_month: bool,
  ) -> FotoFestivalRef {
    Self::new_4(name, result, every_month, "")
  }

  pub fn new_4(
    name: &str,
    result: &str,
    every_month: bool,
    remark: &str,
  ) -> FotoFestivalRef {
    Arc::new(Mutex::new(Self {
      __name: name.to_string(),
      __result: result.to_string(),
      __every_month: every_month,
      __remark: remark.to_string(),
    }))
  }
}

impl LockRef for FotoFestivalRef {
  type Output<'a> = MutexGuard<'a, FotoFestival>;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
