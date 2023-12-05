mod helper_traits;
use crate::lunar::{self, LunarRef};
pub use helper_traits::{FotoRef, RefHelper as FotoRefHelper};
use std::sync::{Arc, Mutex};

pub fn from_ymd_hms(
  year: i64,
  month: i64,
  day: i64,
  hour: i64,
  minute: i64,
  second: i64,
) -> FotoRef {
  from_lunar(lunar::from_ymd_hms(
    year + DEAD_YEAR - 1,
    month,
    day,
    hour,
    minute,
    second,
  ))
}

pub fn from_lunar(lunar: LunarRef) -> FotoRef {
  Foto::new(lunar)
}

#[derive(Clone, Debug)]
pub struct Foto {
  __lunar: LunarRef,
}

impl Foto {
  fn new(lunar: LunarRef) -> FotoRef {
    Arc::new(Mutex::new(Self { __lunar: lunar }))
  }
}

const DEAD_YEAR: i64 = -543;
