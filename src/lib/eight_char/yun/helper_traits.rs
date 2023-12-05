use super::Yun;
use crate::eight_char::da_yun::{DaYun, DaYunRef};
use crate::eight_char::LunarRef;
use crate::lunar::LunarRefHelper;
use crate::solar::{SolarRef, SolarRefHelper};
use std::sync::{Arc, Mutex};

pub type YunRef = Arc<Mutex<Yun>>;

pub trait YunRefHelper {
  fn get_gender(&self) -> i8;
  fn get_start_year(&self) -> i64;
  fn get_start_month(&self) -> i64;
  fn get_start_day(&self) -> i64;
  fn get_start_hour(&self) -> i64;
  fn is_forward(&self) -> bool;
  fn get_lunar(&self) -> LunarRef;
  fn get_start_solar(&self) -> SolarRef;
  fn get_da_yun(&self, n: Option<i64>) -> Vec<DaYunRef>;
}

impl YunRefHelper for YunRef {
  ///
  /// 获取性别
  ///
  /// ## Returns
  /// 性别(1男 ， 0女): **i64**
  ///
  fn get_gender(&self) -> i8 {
    self.lock().unwrap().__gender
  }

  ///
  /// 获取起运年数
  ///
  /// ## Returns
  /// 起运年数: **i64**
  ///
  fn get_start_year(&self) -> i64 {
    self.lock().unwrap().__start_year
  }

  ///
  /// 获取起运月数
  ///
  /// ## Returns
  /// 起运月数: **i64**
  ///
  fn get_start_month(&self) -> i64 {
    self.lock().unwrap().__start_month
  }

  ///
  /// 获取起运天数
  ///
  /// ## Returns
  /// 起运天数: **i64**
  ///
  fn get_start_day(&self) -> i64 {
    self.lock().unwrap().__start_day
  }

  ///
  /// 获取起运小时数
  ///
  /// ## Returns
  /// 起运小时数: **i64**
  ///
  fn get_start_hour(&self) -> i64 {
    self.lock().unwrap().__start_hour
  }

  ///
  /// 是否顺推
  ///
  /// ## Returns
  /// true/false
  ///
  fn is_forward(&self) -> bool {
    self.lock().unwrap().__forward
  }

  fn get_lunar(&self) -> LunarRef {
    self.lock().unwrap().__lunar.clone()
  }

  ///
  /// 获取起运的阳历日期
  ///
  /// ## Returns
  /// 阳历日期: Solar
  ///
  fn get_start_solar(&self) -> SolarRef {
    let (solar, year, month, day, hour) = {
      let s = self.lock().unwrap();
      (
        s.__lunar.get_solar(),
        s.__start_year,
        s.__start_month,
        s.__start_day,
        s.__start_hour,
      )
    };
    let mut solar = solar.next_year(year);
    solar = solar.next_month(month);
    solar = solar.next(day, None);
    solar = solar.next_hour(hour);
    solar
  }

  ///
  /// 获取大运
  ///
  /// ## Arguments
  /// n: **Option<i64>** -  轮数
  ///
  /// ## Returns
  /// 大运
  ///
  fn get_da_yun(&self, n: Option<i64>) -> Vec<DaYunRef> {
    let n = n.unwrap_or(10);
    let mut da_yun = vec![];
    for i in 0..n {
      da_yun.push(DaYun::new(self.clone(), i))
    }
    da_yun
  }
}
