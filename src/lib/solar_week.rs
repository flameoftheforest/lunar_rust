mod helper_traits;
pub use self::helper_traits::{
  RefHelper as SolarWeekRefHelper, SolarWeekRef,
};
use chrono::{Datelike, NaiveDate};
use std::sync::{Arc, Mutex};

///
/// 阳历周
///
#[derive(Clone, Debug)]
pub struct SolarWeek {
  __year: i64,
  __month: i64,
  __day: i64,
  __start: i64,
}

impl SolarWeek {
  pub fn from_date(date: NaiveDate, start: i64) -> SolarWeekRef {
    Arc::new(Mutex::new(Self {
      __year: date.year() as i64,
      __month: date.month() as i64,
      __day: date.day() as i64,
      __start: start,
    }))
  }

  pub fn from_ymd(
    year: i64,
    month: i64,
    day: i64,
    start: i64,
  ) -> SolarWeekRef {
    Arc::new(Mutex::new(Self {
      __year: year,
      __month: month,
      __day: day,
      __start: start,
    }))
  }
}
