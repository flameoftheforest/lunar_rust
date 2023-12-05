use std::sync::{Arc, Mutex, MutexGuard};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::{util::{locked_ref_trait::LockRef, solar_util::{SolarUtilRefHelper, self}, holiday_util::{HolidayUtilRefHelper, self}}, lunar::{LunarRef, self}, solar_month::{SolarMonth, SolarMonthRefHelper}, holiday::HolidayRefHelper};
use super::Solar;

pub type SolarRef = Arc<Mutex<Solar>>;

pub trait RefHelper: LockRef {
  fn is_leap_year(&self) -> bool;
  fn get_week(&self) -> i64;
  fn get_week_in_chinese(self) -> String;
  fn get_festivals(self) -> Vec<String>;
  fn get_other_festivals(&self) -> Vec<String>;
  fn get_xing_zuo(&self) -> String;
  fn get_julian_day(&self) -> f64;
  fn get_lunar(&self) -> LunarRef;
  fn next_day(&self, days: i64) -> SolarRef;
  fn next(&self, days: i64, only_work_day: Option<bool>) -> SolarRef;
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn get_day(&self) -> i64;
  fn get_hour(&self) -> i64;
  fn get_minute(&self) -> i64;
  fn get_second(&self) -> i64;
  fn to_ymd(&self) -> String;
  fn to_naivedate(&self) -> NaiveDate;
  fn to_naivedatetime(&self) -> NaiveDateTime;
  fn to_ymdhms(&self) -> String;
  fn to_full_string(&self) -> String;
  fn to_string(&self) -> String;
  fn subtract(&self, solar: Self) -> i64;
  fn subtract_minute(&self, solar: Self) -> i64;
  fn is_after(&self, solar: Self) -> bool;
  fn is_before(&self, solar: Self) -> bool;
  fn next_year(&self, years: i64) -> SolarRef;
  fn next_month(&self, months: i64) -> SolarRef;
  fn next_hour(&self, hours: i64) -> SolarRef;
}

impl RefHelper for SolarRef {

  /// 
  /// 是否闰年
  /// 
  /// ## Returns
  /// **true** - 闰年
  /// **false** - 非闰年 
  /// 
  fn is_leap_year(&self) -> bool {
    solar_util::get().is_leap_year(self.as_locked_ref().__year)
  }

  /// 
  /// 获取星期，0代表周日，1代表周一
  /// 
  /// ## Returns
  /// + 1234560
  /// 
  fn get_week(&self) -> i64 {
    ((self.get_julian_day() + 0.5) as i64 + 7000001) % 7
  }

  /// 
  /// 获取星期的中文
  /// 
  /// ## Returns
  /// + 日一二三四五六
  fn get_week_in_chinese(self) -> String {
    solar_util::get().WEEK()[self.get_week() as usize].to_string()
  }

  /// 
  /// 获取节日，有可能一天会有多个节日
  /// 
  /// ## Returns
  /// + 劳动节等: **Vec\<String\>**
  /// 
  fn get_festivals(self) -> Vec<String> {
    let mut festivals = vec![];
    let week = self.get_week();
    let (day, month, year) = {
      let s = self.as_locked_ref();
      (s.__day, s.__month, s.__year)
    };
    let _ = solar_util::get().FESTIVAL().get(&(month, day)).map_or((), |v| festivals.push(v.clone()));
    let _ = solar_util::get().WEEK_FESTIVAL()
      .get(&(month, (day as f64 % 7.).ceil() as i64, week))
      .map_or((), |v| festivals.push(v.clone()))
    ;
    if day + 7 > solar_util::get().get_days_of_month(year, month) {
      let _ = solar_util::get().WEEK_FESTIVAL()
        .get(&(month, 0, week))
        .map_or((), |v| festivals.push(v.clone()))
      ;
    }
    festivals
  }

  /// 
  /// 获取非正式的节日，有可能一天会有多个节日
  /// 
  /// ## Returns
  /// + 非正式的节日列表，如中元节: **Vec\<String\>**
  /// 
  fn get_other_festivals(&self) -> Vec<String> {
    let mut festivals = vec![];
    let (day, month) = {
      let s = self.as_locked_ref();
      (s.__day, s.__month)
    };
    let _ = solar_util::get().OTHER_FESTIVAL()
      .get(&(month, day))
      .map_or((), |v| 
        festivals.append(&mut v.clone())
      );
    festivals
  }

  /// 
  /// 获取星座
  /// 
  /// ## Returns
  /// + 星座: **String**
  /// 
  fn get_xing_zuo(&self) -> String {
    let (day, month) = {
      let s = self.as_locked_ref();
      (s.__day, s.__month)
    };
    let y = month * 100 + day;
    let index = match y {
      321..=419 => 0,
      420..=520 => 1,
      521..=621 => 2,
      622..=722 => 3,
      723..=822 => 4,
      823..=922 => 5,
      923..=1023 => 6,
      1024..=1122 => 7,
      1123..=1221 => 8,
      d if d >= 1222 || d <= 119 => 9,
      d if d <= 218 => 10,
      _ => 11
    };
    solar_util::get().XING_ZUO()[index as usize].to_string()
  }

  /// 
  /// 获取儒略日
  /// 
  /// ## Returns
  // + 儒略日: i64
  fn get_julian_day(&self) -> f64 {
    let __julian_day = {self.as_locked_ref().__julian_day}.clone();
    if __julian_day.is_some() {
      return __julian_day.unwrap();
    }

    let (second, minute, hour, day, month, year) = {
      let s = self.as_locked_ref();
      (s.__second, s.__minute, s.__hour, s.__day, s.__month, s.__year)
    };
    let mut y = year;
    let mut m = month;
    let d = day as f64 + (
      (second as f64 / 60.0 + minute as f64) / 60. + hour as f64
    ) / 24.;
    let mut n = 0;
    let g = y * 372 + m * 31 + d as i64 >= 588829;
    if m <= 2 {
      m = m + 12;
      y = y - 1;
    }
    if g {
      n = (y as f64 / 100.) as i64;
      n = 2 - n + (n as f64 / 4.) as i64;
    }

    let v = ((365.25 * (y as f64 + 4716.)) as i64 + (30.6001 * (m as f64 + 1.)) as i64) as f64 + d as f64 + n as f64 - 1524.5;
    self.as_locked_ref().__julian_day = Some(v.clone());
    v
  }

  /// 
  /// 获取农历
  /// 
  /// ## Returns
  /// + 农历: **Lunar**
  /// 
  fn get_lunar(&self) -> LunarRef {
    lunar::from_solar(self)
  }

  fn next_day(&self, days: i64) -> SolarRef {
    let (second, minute, hour, mut d, mut m, mut y) = {
      (self.get_second(), self.get_minute(), self.get_hour(), self.get_day(), self.get_month(), self.get_year())
    };

    if y == 1582 && m == 10 {
      if d > 4 {
        d = d - 10;
      }
    }
    
    match days {
      _d if _d > 0 => {
        d = d + days;
        let mut days_in_month = solar_util::get().get_days_of_month(y, m);
        while d > days_in_month {
          d = d - days_in_month;
          m = m + 1;
          if m > 12 {
            m = 1;
            y = y + 1;
          }
          days_in_month = solar_util::get().get_days_of_month(y, m);
        }
      }
      _d if _d < 0 => {
        while d + days <= 0 {
          m = m - 1;
          if m < 1 {
            m = 1;
            y = y - 1;
          }
          d = d + solar_util::get().get_days_of_month(y, m);
        }
        d = d + days
      }
      _ => ()
    };

    if y == 1582 && m == 10 {
      if d > 4 {
        d = d + 10;
      }
    }

    super::from_ymdhms(y, m, d, hour, minute, second)
  }

  /// 
  /// 获取往后推几天的阳历日期，如果要往前推，则天数用负数
  /// 
  /// ## Arguments
  /// + days: **i64** - 天数
  /// + only_work_day: **bool** - 是否仅工作日 (default: false)
  /// 
  /// ## Returns
  /// + 阳历日期: **Solar**
  /// 
  fn next(&self, days: i64, only_work_day: Option<bool>) -> SolarRef {
    let only_work_day = only_work_day.unwrap_or(false);
    if !only_work_day {
      return self.next_day(days);
    }
    let mut solar = self.clone();
    if days == 0 {
      return solar;
    }
    let mut rest = days.abs();
    let add = match days {
      _d if _d < 0 => -1,
      _ => 1
    };

    while rest > 0 {
      solar = solar.next(add, None);
      
      let holiday = holiday_util::get().get_holiday(
        solar.get_year(), 
        Some(solar.get_month()), 
        Some(solar.get_day())
      );

      let work = match holiday {
        None => {
          let week = solar.get_week();
          !(0 == week || 6 == week)
        }
        Some(h) => {
          h.is_work()
        }
      };

      if work {
        rest = rest - 1;
      }
    }

    solar
  }

  fn get_year(&self) -> i64 {
    self.as_locked_ref().__year
  }

  fn get_month(&self) -> i64 {
    self.as_locked_ref().__month
  }

  fn get_day(&self) -> i64 {
    self.as_locked_ref().__day
  }

  fn get_hour(&self) -> i64 {
    self.as_locked_ref().__hour
  }

  fn get_minute(&self) -> i64 {
    self.as_locked_ref().__minute
  }

  fn get_second(&self) -> i64 {
    self.as_locked_ref().__second
  }

  fn to_ymd(&self) -> String {
    format!("{:0>4}-{:0>2}-{:0>2}", self.get_year(), self.get_month(), self.get_day())
  }

  fn to_naivedate(&self) -> NaiveDate {
    NaiveDate::from_ymd_opt(self.get_year() as i32, self.get_month() as u32, self.get_day() as u32).unwrap()
  }

  fn to_naivedatetime(&self) -> NaiveDateTime {
    self.to_naivedate()
      .and_time(
        NaiveTime::from_hms_opt(
          self.get_hour() as u32, 
          self.get_minute() as u32, 
          self.get_second() as u32)
        .unwrap()
      )
  }

  fn to_ymdhms(&self) -> String {
    format!("{} {:0>2}:{:0>2}:{:0>2}", 
      self.to_ymd(), 
      self.get_hour(), self.get_minute(), self.get_second())
  }

  fn to_full_string(&self) -> String {
    let mut s = self.to_ymdhms();
    if self.is_leap_year() {
      s = format!("{s} 闰年");
    }
    s = format!("{} 星期{}", s, self.clone().get_week_in_chinese());
    self.clone().get_festivals().iter().for_each(|f| s = format!("{s} ({f})"));
    self.get_other_festivals().iter().for_each(|f| s = format!("{s} ({f})"));
    s = format!("{s} {}座", self.get_xing_zuo());
    s
  }

  fn to_string(&self) -> String {
    self.to_ymd()
  }

  fn subtract(&self, solar: Self) -> i64 {
    solar_util::get().get_days_between(
      solar.get_year(), solar.get_month(), solar.get_day(), 
      self.get_year(), self.get_month(), self.get_day()
    )
  }

  fn subtract_minute(&self, solar: Self) -> i64 {
    let mut days = self.subtract(solar.clone());
    let cm = self.get_hour() * 60 + self.get_minute();
    let sm = solar.clone().get_hour() * 60 + solar.clone().get_minute();
    let mut m = cm - sm;
    if m < 0 {
      m = m + 1440;
      days = days - 1;
    }
    m = days * 1440 + m;
    m
  }

  fn is_after(&self, solar: Self) -> bool {
    if  self.get_year() > solar.get_year() { return true; }
    if  self.get_year() < solar.get_year() { return false; }
    if  self.get_month() > solar.get_month() { return true; }
    if  self.get_month() < solar.get_month() { return false; }
    if  self.get_day() > solar.get_day() { return true; }
    if  self.get_day() < solar.get_day() { return false; }
    if  self.get_hour() > solar.get_hour() { return true; }
    if  self.get_hour() < solar.get_hour() { return false; }
    if  self.get_minute() > solar.get_minute() { return true; }
    if  self.get_minute() < solar.get_minute() { return false; }
    self.get_second() > solar.get_second()
  }

  fn is_before(&self, solar: Self) -> bool {
    if  self.get_year() > solar.get_year() { return false; }
    if  self.get_year() < solar.get_year() { return true; }
    if  self.get_month() > solar.get_month() { return false; }
    if  self.get_month() < solar.get_month() { return true; }
    if  self.get_day() > solar.get_day() { return false; }
    if  self.get_day() < solar.get_day() { return true; }
    if  self.get_hour() > solar.get_hour() { return false; }
    if  self.get_hour() < solar.get_hour() { return true; }
    if  self.get_minute() > solar.get_minute() { return false; }
    if  self.get_minute() < solar.get_minute() { return true; }
    self.get_second() < solar.get_second()
  }

  fn next_year(&self, years: i64) -> SolarRef {
    let y = self.get_year() + years;
    let m = self.get_month();
    let mut d = self.get_day();
    if 1582 == y && 10 == m {
      if d > 4 && d < 15 {
        d = d + 10;
      }
    } else if 2 == m {
      if d > 28 {
        if !solar_util::get().is_leap_year(y) {
          d = 28;
        }
      }
    }
    super::from_ymdhms(y, m, d, self.get_hour(), self.get_minute(), self.get_second())
  }

  fn next_month(&self, months: i64) -> SolarRef {
    let month = SolarMonth::from_ym(self.get_year(), self.get_month()).next(months);
    let y = month.get_year();
    let m = month.get_month();
    let mut d = self.get_day();
    if 1582 == y && 10 == m {
      if d > 4 && d < 15 {
        d = d + 10;
      }
    } else {
      let days = solar_util::get().get_days_of_month(y, m);
      if d > days { d = days; }
    }
    super::from_ymdhms(y, m, d, self.get_hour(), self.get_minute(), self.get_second())
  }

  fn next_hour(&self, hours: i64) -> SolarRef {
    let h = self.get_hour() + hours;
    let n = match h < 0 {
      true => -1,
      _ => 1
    };
    let mut hour = h.abs();
    let mut days = (hour as f64 / 24.) as i64 * n;
    hour = (hour % 24) * n;
    if hour < 0 {
      hour = hour + 24;
      days = days - 1;
    }
    let solar = self.next(days, None);
    super::from_ymdhms(solar.get_year(), solar.get_month(), solar.get_day(), hour, solar.get_minute(), solar.get_second())
  }

}

impl LockRef for SolarRef {
  type Output<'a> = MutexGuard<'a, Solar,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}