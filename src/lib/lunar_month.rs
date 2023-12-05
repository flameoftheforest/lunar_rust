use crate::{
  lunar::LunarRefHelper,
  lunar_year::{LunarYear, LunarYearRefHelper},
  nine_star::NineStar,
  solar::{self, SolarRefHelper},
  util::{
    locked_ref_trait::LockRef,
    lunar_util::{
      BASE_MONTH_ZHI_INDEX, GAN, MONTH, POSITION_CAI, POSITION_DESC,
      POSITION_FU, POSITION_FU_2, POSITION_GAN, POSITION_XI,
      POSITION_YANG_GUI, POSITION_YIN_GUI, ZHI,
    },
  },
};
use std::sync::{Arc, Mutex, MutexGuard};

///
/// 农历月
///
pub fn new(
  lunar_year: i64,
  lunar_month: i64,
  day_count: i64,
  first_julian_day: f64,
  index: i64,
) -> LunarMonthRef {
  Arc::new(Mutex::new(LunarMonth {
    __year: lunar_year,
    __month: lunar_month,
    __day_count: day_count,
    __first_julian_day: first_julian_day,
    __index: index,
    __zhi_index: (index - 1 + BASE_MONTH_ZHI_INDEX()) % 12,
  }))
}

pub fn from_ym(lunar_year: i64, lunar_month: i64) -> LunarMonthRef {
  LunarYear::from_lunar_year(lunar_year)
    .get_month(lunar_month)
    .unwrap()
}

#[derive(Clone, Debug)]
pub struct LunarMonth {
  __year: i64,
  __month: i64,
  __day_count: i64,
  __first_julian_day: f64,
  __index: i64,
  __zhi_index: i64,
}

pub type LunarMonthRef = Arc<Mutex<LunarMonth>>;
pub trait LunarMonthRefHelper: LockRef {
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn get_index(&self) -> i64;
  fn get_zhi_index(&self) -> i64;
  fn get_gan_index(&self) -> i64;
  fn get_gan(&self) -> String;
  fn get_zhi(&self) -> String;
  fn get_gan_zhi(&self) -> String;
  fn get_position_xi(&self) -> String;
  fn get_position_xi_desc(&self) -> String;
  fn get_position_yang_gui(&self) -> String;
  fn get_position_yang_gui_desc(&self) -> String;
  fn get_position_yin_gui(&self) -> String;
  fn get_position_yin_gui_desc(&self) -> String;
  fn get_position_fu(self, sect: Option<i64>) -> String;
  fn get_position_fu_desc(self, sect: Option<i64>) -> String;
  fn get_position_cai(&self) -> String;
  fn get_position_cai_desc(&self) -> String;
  fn is_leap(&self) -> bool;
  fn get_day_count(&self) -> i64;
  fn get_first_julian_day(&self) -> f64;
  fn get_position_tai_sui(&self) -> String;
  fn get_position_tai_sui_desc(&self) -> String;
  fn get_nine_star(&self) -> Arc<Mutex<NineStar>>;
  fn to_string(&self) -> String;
  fn next(&self, n: i64) -> LunarMonthRef;
}

impl LunarMonthRefHelper for LunarMonthRef {
  fn get_year(&self) -> i64 {
    self.as_locked_ref().__year
  }

  fn get_month(&self) -> i64 {
    self.as_locked_ref().__month
  }

  #[allow(dead_code)]
  fn get_index(&self) -> i64 {
    self.as_locked_ref().__index
  }

  fn get_zhi_index(&self) -> i64 {
    self.as_locked_ref().__zhi_index
  }

  fn get_gan_index(&self) -> i64 {
    let offset =
      (LunarYear::from_lunar_year(self.get_year()).get_gan_index() + 1)
        % 5
        * 2;
    (self.get_index() - 1 + offset) % 10
  }

  fn get_gan(&self) -> String {
    GAN()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_zhi(&self) -> String {
    ZHI()[self.get_zhi_index() as usize + 1].to_string()
  }

  fn get_gan_zhi(&self) -> String {
    format!("{}{}", self.get_gan(), self.get_zhi())
  }

  fn get_position_xi(&self) -> String {
    POSITION_XI()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_position_xi_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_xi())
      .unwrap()
      .clone()
  }

  fn get_position_yang_gui(&self) -> String {
    POSITION_YANG_GUI()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_position_yang_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_yang_gui())
      .unwrap()
      .clone()
  }

  fn get_position_yin_gui(&self) -> String {
    POSITION_YIN_GUI()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_position_yin_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_yin_gui())
      .unwrap()
      .clone()
  }

  fn get_position_fu(self, sect: Option<i64>) -> String {
    let position_fu = match sect.unwrap_or(2) {
      1 => POSITION_FU.clone(),
      _ => POSITION_FU_2.clone(),
    };
    position_fu()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_position_fu_desc(self, sect: Option<i64>) -> String {
    POSITION_DESC()
      .get(&self.get_position_fu(sect))
      .unwrap()
      .clone()
  }

  fn get_position_cai(&self) -> String {
    POSITION_CAI()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_position_cai_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_cai())
      .unwrap()
      .clone()
  }

  fn is_leap(&self) -> bool {
    self.get_month() < 0
  }

  fn get_day_count(&self) -> i64 {
    self.as_locked_ref().__day_count
  }

  fn get_first_julian_day(&self) -> f64 {
    self.as_locked_ref().__first_julian_day
  }

  fn get_position_tai_sui(&self) -> String {
    let m = self.get_month().abs() % 4;
    let position = POSITION_GAN();
    match m {
      0 => "巽",
      1 => "艮",
      3 => "坤",
      _ => position[solar::from_julian_day(self.get_first_julian_day())
        .get_lunar()
        .get_month_gan_index() as usize]
        .as_str(),
    }
    .to_string()
  }

  fn get_position_tai_sui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_tai_sui())
      .unwrap()
      .clone()
  }

  fn get_nine_star(&self) -> Arc<Mutex<NineStar>> {
    let index =
      LunarYear::from_lunar_year(self.get_year()).get_zhi_index() % 3;
    let m = self.get_month().abs();
    let month_zhi_index = (13 + m) % 12;
    let mut n = 27 - (index * 3);
    if month_zhi_index < BASE_MONTH_ZHI_INDEX().clone() {
      n = n - 3;
    }
    let offset = (n - month_zhi_index) % 9;
    NineStar::from_index(offset as usize)
  }

  fn to_string(&self) -> String {
    format!(
      "{}年{}{}月({}天)",
      self.get_year(),
      match self.is_leap() {
        true => "闰",
        _ => "",
      },
      MONTH()[self.get_month().abs() as usize],
      self.get_day_count()
    )
  }

  ///
  /// 获取往后推几个月的阴历月，如果要往前推，则月数用负数
  ///
  /// ## Arguments
  /// + n: i64 - 月数
  ///
  /// ## Returns
  /// + 阴历月: **LunarMonth**
  ///
  ///
  fn next(&self, n: i64) -> LunarMonthRef {
    if n == 0 {
      return self.clone();
    }

    if n > 0 {
      let mut rest = n;
      let mut ny = self.get_year();
      let mut iy = ny;
      let mut im = self.get_month();
      let mut index = 0;
      let mut months = LunarYear::from_lunar_year(ny).get_months();
      loop {
        let size = months.len();
        for i in 0..size {
          let m = &months[i];
          if m.get_year() == iy && m.get_month() == im {
            index = 1;
            break;
          }
        }
        let more = size - index - 1;
        if (rest as usize) < more {
          break;
        }
        rest = rest - more as i64;
        let last_month = months[size - 1].clone();
        iy = last_month.get_year();
        im = last_month.get_month();
        ny = ny + 1;
        months = LunarYear::from_lunar_year(ny).get_months();
      }
      return months[index + rest as usize].clone();
    }

    let mut rest = -n;
    let mut ny = self.get_year();
    let mut iy = ny;
    let mut im = self.get_month();
    let mut index = 0;
    let mut months = LunarYear::from_lunar_year(ny).get_months();
    loop {
      let size = months.len();
      for i in 0..size {
        let m = months[i].clone();
        if m.get_year() == iy && m.get_month() == im {
          index = i;
          break;
        }
      }
      if (rest as usize) <= index {
        break;
      }
      rest = rest - index as i64;
      let first_month = months[0].clone();
      iy = first_month.get_year();
      im = first_month.get_month();
      ny = ny - 1;
      months = LunarYear::from_lunar_year(ny).get_months();
    }
    months[index - rest as usize].clone()
  }
}

impl LockRef for LunarMonthRef {
  type Output<'a> = MutexGuard<'a, LunarMonth,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
