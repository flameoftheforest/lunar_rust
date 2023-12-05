mod helper_traits;
use std::sync::{Arc, Mutex};

use super::liu_nian::LiuNianRef;
pub use helper_traits::{LiuYueRef, RefHelper as LiuYueRefHelper};

#[derive(Clone, Debug)]
pub struct LiuYue {
  __liu_nian: LiuNianRef,
  __index: i64,
}

impl LiuYue {
  pub fn new(liu_nian: LiuNianRef, index: i64) -> LiuYueRef {
    Arc::new(Mutex::new(Self {
      __liu_nian: liu_nian,
      __index: index,
    }))
  }
}
