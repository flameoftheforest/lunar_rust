mod helper_traits;
use std::sync::{Arc, Mutex};

pub use self::helper_traits::{
  RefHelper as SolarMonthRefHelper, SolarMonthRef,
};
use chrono::{Datelike, NaiveDateTime};

///
/// 阳历月
///
#[derive(Clone, Debug)]
pub struct SolarMonth {
  __month: i64,
  __year: i64,
}

impl SolarMonth {
  pub fn from_date(date: NaiveDateTime) -> SolarMonthRef {
    Arc::new(Mutex::new(SolarMonth {
      __year: date.year() as i64,
      __month: date.month() as i64,
    }))
  }

  pub fn from_ym(year: i64, month: i64) -> SolarMonthRef {
    Arc::new(Mutex::new(SolarMonth {
      __month: month,
      __year: year,
    }))
  }
}
