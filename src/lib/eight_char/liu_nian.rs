mod helper_traits;
use super::da_yun::DaYunRef;
use crate::eight_char::da_yun::DaYunRefHelper;
use crate::lunar::LunarRef;
pub use helper_traits::{LiuNianRef, RefHelper as LiuNianRefHelper};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct LiuNian {
  __da_yun: DaYunRef,
  __lunar: LunarRef,
  __index: i64,
  __year: i64,
  __age: i64,
}

impl LiuNian {
  pub fn new(da_yun: DaYunRef, index: i64) -> LiuNianRef {
    Arc::new(Mutex::new(Self {
      __da_yun: da_yun.clone(),
      __lunar: da_yun.get_lunar(),
      __index: index,
      __year: da_yun.get_start_year() + index,
      __age: da_yun.get_start_age() + index,
    }))
  }
}
