use std::sync::{Arc, Mutex, MutexGuard};
use crate::{util::{locked_ref_trait::LockRef, solar_util::{SolarUtilRefHelper, self}}, solar::{SolarRefHelper, SolarRef, self}};
use super::SolarWeek;

pub type SolarWeekRef = Arc<Mutex<SolarWeek>>;

pub trait RefHelper: LockRef {
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn get_day(&self) -> i64;
  fn get_start(&self) -> i64;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
  fn get_index(&self) -> i64;
  fn get_index_in_year(&self) -> i64;
  fn get_first_day(&self) -> SolarRef;
  fn get_first_day_in_month(&self) -> Option<SolarRef>;
  fn get_days(&self) -> Vec<SolarRef>;
  fn get_days_in_month(&self) -> Vec<SolarRef>;
  fn next(&self, weeks: i64, separate_month: bool) -> SolarWeekRef;
}

impl RefHelper for SolarWeekRef {
  fn get_year(&self) -> i64 {
    self.as_locked_ref().__year
  }

  fn get_month(&self) -> i64 {
    self.as_locked_ref().__month
  }

  fn get_day(&self) -> i64 {
    self.as_locked_ref().__day
  }

  fn get_start(&self) -> i64 {
    self.as_locked_ref().__start
  }

  fn to_string(&self) -> String {
    format!("{}-{}-{}", self.get_year(), self.get_month(), self.get_index())
  }
  
  fn to_full_string(&self) -> String {
    format!("{}年{}月第{}周", self.get_year(), self.get_month(), self.get_index())
  }

  /// 
  /// 获取当前日期是在当月第几周
  /// 
  /// ## Returns
  /// + 周序号: **i64** - 从1开始
  /// 
  fn get_index(&self) -> i64 {
    let mut offset = solar::from_ymd(self.get_year(), self.get_month(), 1).get_week() - self.get_start();
    if offset < 0 {
      offset = offset + 7;
    }
    ((self.get_day() + offset) as f64 / 7.).ceil() as i64
  }

  fn get_index_in_year(&self) -> i64 {
    let mut offset = solar::from_ymd(self.get_year(), 1, 1).get_week() - self.get_start();
    if offset < 0 {
      offset = offset + 7;
    }
    ((
      solar_util::get().get_days_in_year(
        self.get_year(), 
        self.get_month(), 
        self.get_day()) + offset
    ) as f64 / 7.).ceil() as i64
  }

  /// 
  /// 获取本周第一天的阳历日期（可能跨月）
  /// 
  /// ## Returns
  /// + 本周第一天的阳历日期: **Solar**
  /// 
  fn get_first_day(&self) -> SolarRef {
    let solar = solar::from_ymd(self.get_year(), self.get_month(), self.get_day());
    let mut prev = solar.get_week() - self.get_start();
    if prev < 0 {
      prev = prev + 7;
    }
    solar.next(-prev, None)
  }

  ///
  /// 获取本周第一天的阳历日期（仅限当月）
  /// 
  /// ## Returns
  /// + 本周第一天的阳历日期: **Option<Solar>**
  /// 
  fn get_first_day_in_month(&self) -> Option<SolarRef> {
    for day in self.get_days().iter() {
      if self.get_month() == day.get_month() {
        return Some(day.clone())
      }
    }
    None
  }

  /// 
  /// 获取本周的阳历日期列表（可能跨月）
  /// + 本周的阳历日期列表: **Vec\<Solar\>**
  /// 
  fn get_days(&self) -> Vec<SolarRef> {
    let mut days = vec![];
    let first = self.get_first_day();
    days.push(first.clone());
    for i in 1..7 {
      days.push(first.next(i, None));
    }
    days
  }

  /// 
  /// 获取本周的阳历日期列表（仅限当月）
  /// 
  /// ## Returns
  /// + 本周的阳历日期列表（仅限当月: **Vec\<Solar\>**
  /// 
  fn get_days_in_month(&self) -> Vec<SolarRef> {
    let mut days = vec![];
    for solar in self.get_days().iter() {
      if self.get_month() == solar.get_month() {
        days.push(solar.clone());
      }
    }
    days
  }

  /// 
  /// 周推移
  /// 
  /// ## Arguments
  /// + weeks: i64 - 推移的周数，负数为倒推
  /// + separate_month: **bool** - 是否按月单独计算
  /// 
  /// ## Returns
  /// + 推移后的阳历周: **SolarWeek**
  /// 
  fn next(&self, weeks: i64, separate_month: bool) -> SolarWeekRef {
    if 0 == weeks {
      return self.clone();
    }

    let get_solar_week_from_solar = |solar: &SolarRef| -> SolarWeekRef {
      SolarWeek::from_ymd(solar.get_year(), solar.get_month(), solar.get_day(), self.get_start())
    };

    let mut solar = solar::from_ymd(self.get_year(), self.get_month(), self.get_day());
    if separate_month {
      let mut n = weeks;
      let mut week = 
        // SolarWeek::from_ymd(solar.get_year(), solar.get_month(), solar.get_day(), self.__start);
        get_solar_week_from_solar(&solar);
      let mut month = self.get_month();
      let plus = n > 0;
      let days = match plus {
        true => 7,
        _ => -7
      };
      while 0 != n {
        solar = solar.next(days, None);
        week = get_solar_week_from_solar(&solar);
        let mut week_month = week.get_month();
        if month != week_month {
          let index = week.get_index();
          match plus {
            true => {
              match 1 == index {
                true => {
                  let first_day = week.get_first_day();
                  week = get_solar_week_from_solar(&first_day);
                  week_month = week.get_month();
                }
                _ => {
                  solar = solar::from_ymd(week.get_year(), week.get_month(), 1);
                  week = get_solar_week_from_solar(&solar);
                }
              };
            }
            _ => {
              match solar_util::get().get_weeks_of_month(week.get_year(), week.get_month(), self.get_start()) == index {
                true => {
                  let last_day = week.get_first_day().next(6, None);
                  week = get_solar_week_from_solar(&last_day);
                  week_month = week.get_month();
                }
                _ => {
                  solar = solar::from_ymd(
                    week.get_year(), 
                    week.get_month(), 
                    solar_util::get().get_days_of_month(week.get_year(), week.get_month())
                  );
                  week = get_solar_week_from_solar(&solar);
                }
              };
            }
          };
          month = week_month;
        }

        n = n - match plus {
          true => 1,
          _ => -1
        };
      }
      return week;
    }
    
    solar = solar.next(weeks * 7, None);
    get_solar_week_from_solar(&solar)
  }
}

impl LockRef for SolarWeekRef {
  type Output<'a> = MutexGuard<'a, SolarWeek,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}