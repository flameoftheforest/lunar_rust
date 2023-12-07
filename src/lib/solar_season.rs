use crate::{
  solar_month::{SolarMonth, SolarMonthRef, SolarMonthRefHelper},
  util::{
    locked_ref_trait::LockRef,
    mmacro::{__static_funk, static_funk},
    stringx::StringX,
  },
};
use chrono::{Datelike, NaiveDate};
#[allow(unused_imports)]
use once_cell::sync::Lazy;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex, MutexGuard},
};

pub struct SolarSeason {
  __year: i64,
  __month: i64,
}

pub type SolarSeasonRef = Arc<Mutex<SolarSeason>>;

pub trait SolarSeasonRefHelper: LockRef {
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
  fn get_index(&self) -> i64;
  fn get_months(&self) -> Vec<SolarMonthRef>;
  fn next(&self, seasons: i64) -> SolarSeasonRef;
}

impl SolarSeasonRefHelper for SolarSeasonRef {
  fn get_year(&self) -> i64 {
    self.as_locked_ref().__year
  }

  fn get_month(&self) -> i64 {
    self.as_locked_ref().__month
  }

  fn to_string(&self) -> String {
    (StringX::new("") + self.get_year() + "." + self.get_index()).get()
  }

  fn to_full_string(&self) -> String {
    (StringX::new("")
      + self.get_year()
      + "年"
      + self.get_index()
      + "季度")
      .get()
  }

  ///
  /// 获取当月是第几季度
  ///
  /// ## Returns
  /// 季度序号，从1开始: **i64**
  ///
  fn get_index(&self) -> i64 {
    (self.get_month() as f64 / SolarSeason::MONTH_COUNT()).ceil() as i64
  }

  ///
  /// 获取本季度的阳历月列表
  ///
  /// ## Returns
  /// 阳历月列表: Vec<SolarMonthRef>
  ///
  fn get_months(&self) -> Vec<SolarMonthRef> {
    let mut months = vec![];
    let index = self.get_index() - 1;
    for i in 0..(SolarSeason::MONTH_COUNT() as i64) {
      months.push(SolarMonth::from_ym(
        self.get_year(),
        SolarSeason::MONTH_COUNT() as i64 * (index + i) + 1,
      ));
    }
    months
  }

  ///
  /// 季度推移
  ///
  /// ## Arguments
  /// seasons: 推移的季度数，负数为倒推
  ///
  /// ## Returns
  /// 推移后的季度: SolarSeasonRef
  ///
  fn next(&self, seasons: i64) -> SolarSeasonRef {
    let m = SolarMonth::from_ym(self.get_year(), self.get_month())
      .next(SolarSeason::MONTH_COUNT() as i64 * seasons);
    SolarSeason::from_ym(m.get_year(), m.get_month())
  }
}

impl LockRef for SolarSeasonRef {
  type Output<'a> = MutexGuard<'a, SolarSeason,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}

impl SolarSeason {
  fn __init(year: i64, month: i64) -> SolarSeasonRef {
    let key = (StringX::new("") + year + "-" + month).get();
    let found = {
      let c = Self::__CACHE();
      let c = c.lock().unwrap();
      let c = c.get(&key);
      match c {
        Some(c) => Some(c.clone()),
        _ => None,
      }
    };
    if found.is_some() {
      return found.unwrap().clone();
    }

    let s = Arc::new(Mutex::new(Self {
      __year: year,
      __month: month,
    }));

    {
      Self::__CACHE().lock().unwrap().insert(key.clone(), s);
    }

    {
      Self::__CACHE().lock().unwrap().get(&key).unwrap().clone()
    }
  }

  pub fn from_date(date: NaiveDate) -> SolarSeasonRef {
    Self::__init(date.year() as i64, date.month0() as i64 + 1)
  }

  pub fn from_ym(year: i64, month: i64) -> SolarSeasonRef {
    Self::__init(year, month)
  }
}

impl SolarSeason {
  static_funk!(MONTH_COUNT, f64, 3.);
  __static_funk!(
    __CACHE,
    Arc<Mutex<HashMap<String, SolarSeasonRef>>>,
    Arc::new(Mutex::new(HashMap::new()))
  );
}
