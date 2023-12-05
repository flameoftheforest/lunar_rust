use crate::{
  lunar::{self, LunarRef, LunarRefHelper},
  util::lunar_util::LunarUtil,
};
use std::sync::{Arc, Mutex};

pub use self::helper_triats::{
  LunarTimeRef, RefHelper as LunarTimeRefHelper,
};

mod helper_triats;

pub struct LunarTime {
  __lunar: LunarRef,
  __zhi_index: i64,
  __gan_index: i64,
}

impl LunarTime {
  fn new(
    lunar_year: i64,
    lunar_month: i64,
    lunar_day: i64,
    hour: i64,
    minute: i64,
    second: i64,
  ) -> LunarTimeRef {
    let __lunar = lunar::from_ymd_hms(
      lunar_year,
      lunar_month,
      lunar_day,
      hour,
      minute,
      second,
    );
    let __zhi_index = LunarUtil::get_time_zhi_index(&format!(
      "{}:{}",
      {
        match hour < 10 {
          true => format!("0{}", hour),
          _ => format!("{}", hour),
        }
      },
      {
        match minute < 10 {
          true => format!("0{}", minute),
          _ => format!("{}", minute),
        }
      }
    ));
    let __gan_index =
      (__lunar.get_day_gan_index_exact() % 5 * 2 + __zhi_index) % 10;

    Arc::new(Mutex::new(Self {
      __lunar,
      __zhi_index,
      __gan_index,
    }))
  }

  pub fn from_ymd_hms(
    lunar_year: i64,
    lunar_month: i64,
    lunar_day: i64,
    hour: i64,
    minute: i64,
    second: i64,
  ) -> LunarTimeRef {
    Self::new(lunar_year, lunar_month, lunar_day, hour, minute, second)
  }
}
