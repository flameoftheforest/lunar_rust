use super::SolarMonth;
use crate::{
  solar::{self, SolarRef, SolarRefHelper},
  solar_week::{SolarWeek, SolarWeekRef, SolarWeekRefHelper},
  util::{
    locked_ref_trait::LockRef,
    solar_util::{self, SolarUtilRefHelper},
  },
};
use std::sync::{Arc, Mutex, MutexGuard};

pub type SolarMonthRef = Arc<Mutex<SolarMonth>>;

pub trait RefHelper: LockRef {
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
  fn get_days(&self) -> Vec<SolarRef>;
  fn get_weeks(&self, start: i64) -> Vec<SolarWeekRef>;
  fn next(self, months: i64) -> SolarMonthRef;
}

impl RefHelper for SolarMonthRef {
  fn get_year(&self) -> i64 {
    self.as_locked_ref().__year
  }

  fn get_month(&self) -> i64 {
    self.as_locked_ref().__month
  }

  fn to_string(&self) -> String {
    format!("{}-{}", self.get_year(), self.get_month())
  }

  fn to_full_string(&self) -> String {
    format!("{}年{}月", self.get_year(), self.get_month())
  }

  ///
  /// 获取本月的阳历日期列表
  ///
  /// ## Returns
  /// + 阳历日期列表: **Vec\<Solar\>**
  ///
  fn get_days(&self) -> Vec<SolarRef> {
    let mut days = vec![];
    let d = solar::from_ymd(self.get_year(), self.get_month(), 1);
    days.push(d.clone());
    let days_of_month = solar_util::get()
      .get_days_of_month(self.get_year(), self.get_month());
    for i in 1..days_of_month {
      days.push(d.next(i, None))
    }
    days
  }

  ///
  /// 获取本月的阳历日期列表
  ///
  /// ## Arguments
  /// + start: i64 - 星期几作为一周的开始，1234560分别代表星期一至星期天
  ///
  /// ## Returns
  /// + 阳历日期列表: **Vec\<Solar\>**
  ///
  fn get_weeks(&self, start: i64) -> Vec<SolarWeekRef> {
    let mut weeks = vec![];
    let mut week =
      SolarWeek::from_ymd(self.get_year(), self.get_month(), 1, start);
    loop {
      weeks.push(week.clone());
      week = week.next(1, false);
      let first_day = week.get_first_day();
      if first_day.get_year() > self.get_year()
        || first_day.get_month() > self.get_month()
      {
        break;
      }
    }
    weeks
  }

  ///
  /// 获取往后推几个月的阳历月，如果要往前推，则月数用负数
  ///
  /// ## Arguments
  /// + months: **i64** - 月数
  ///
  /// ## Returns
  /// + 阳历月: **SolarMonth**
  ///
  fn next(self, months: i64) -> SolarMonthRef {
    let mut n = 1;
    if months < 0 {
      n = -1;
    }
    let mut m = months.abs();
    let mut y = self.get_year() + (m as f64 / 12.) as i64 * n;
    m = self.get_month() + m % 12 * n;
    if m > 12 {
      m = m - 12;
      y = y + 1;
    } else if m < 1 {
      m = m + 12;
      y = y - 1;
    }
    SolarMonth::from_ym(y, m)
  }
}

impl LockRef for SolarMonthRef {
  type Output<'a> = MutexGuard<'a, SolarMonth,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
