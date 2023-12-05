use crate::{
  solar_month::{SolarMonth, SolarMonthRef, SolarMonthRefHelper},
  util::locked_ref_trait::LockRef,
};
use chrono::{Datelike, NaiveDate};
use std::sync::{Arc, Mutex, MutexGuard};

pub fn from_year(year: i64) -> SolarYearRef {
  __init(year)
}

pub fn from_date(date: NaiveDate) -> SolarYearRef {
  __init(date.year() as i64)
}

fn __init(year: i64) -> Arc<Mutex<SolarYear>> {
  Arc::new(Mutex::new(SolarYear { __year: year }))
}

pub struct SolarYear {
  __year: i64,
}
pub type SolarYearRef = Arc<Mutex<SolarYear>>;

pub trait SolarYearRefHelper: LockRef {
  fn get_year(&self) -> i64;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
  fn get_months(&self) -> Vec<SolarMonthRef>;
  fn next(&self, years: i64) -> SolarYearRef;
}

impl SolarYearRefHelper for SolarYearRef {
  fn get_year(&self) -> i64 {
    self.as_locked_ref().__year
  }

  fn to_string(&self) -> String {
    format!("{}", self.as_locked_ref().__year)
  }

  fn to_full_string(&self) -> String {
    format!("{}年", self.as_locked_ref().__year)
  }

  ///
  /// 获取本年的阳历月列表
  /// 阳历月列表: Vec<SolarMonthRef>
  ///
  fn get_months(&self) -> Vec<SolarMonthRef> {
    let mut months = vec![];
    let m = SolarMonth::from_ym(self.as_locked_ref().__year, 1);
    months.push(m.clone());
    for i in 1..MONTH_COUNT {
      months.push(m.clone().next(i))
    }
    months
  }

  ///
  /// 获取往后推几年的阳历年，如果要往前推，则月数用负数
  ///
  /// ## Arguments
  /// years: i64 - 年数
  ///
  /// ## Returns
  /// 阳历年: SolarYearRef
  ///
  fn next(&self, years: i64) -> SolarYearRef {
    from_year(self.get_year() + years)
  }
}

impl LockRef for SolarYearRef {
  type Output<'a> = MutexGuard<'a, SolarYear,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}

const MONTH_COUNT: i64 = 12;
