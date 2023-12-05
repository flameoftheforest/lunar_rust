mod helper_traits;
use crate::lunar::{self, LunarRef};
pub use helper_traits::{RefHelper as TaoRefHelper, TaoRef};
use std::sync::{Arc, Mutex};

pub fn from_lunar(lunar: LunarRef) -> TaoRef {
  Tao::new(lunar)
}

pub fn from_ymd_hms(
  year: i64,
  month: i64,
  day: i64,
  hour: i64,
  minute: i64,
  second: i64,
) -> TaoRef {
  from_lunar(lunar::from_ymd_hms(
    year + BIRTH_YEAR,
    month,
    day,
    hour,
    minute,
    second,
  ))
}

pub fn from_ymd(year: i64, month: i64, day: i64) -> TaoRef {
  from_ymd_hms(year, month, day, 0, 0, 0)
}

#[derive(Clone, Debug)]
pub struct Tao {
  __lunar: LunarRef,
}

impl Tao {
  fn new(lunar: LunarRef) -> TaoRef {
    Arc::new(Mutex::new(Self { __lunar: lunar }))
  }
}

const BIRTH_YEAR: i64 = -2697;
