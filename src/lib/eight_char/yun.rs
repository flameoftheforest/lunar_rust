mod helper_traits;
use crate::eight_char::EightCharRef;
use crate::eight_char::EightCharRefHelper;
use crate::jie_qi::JieQiRefHelper;
use crate::lunar::LunarRef;
use crate::lunar::LunarRefHelper;
use crate::solar::SolarRefHelper;
use crate::util::find_char_index::FindCharIndex;
use crate::util::lunar_util::LunarUtil;
pub use helper_traits::{YunRef, YunRefHelper};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Yun {
  __lunar: LunarRef,
  __gender: i8,
  __forward: bool,
  __start_year: i64,
  __start_month: i64,
  __start_day: i64,
  __start_hour: i64,
}

impl Yun {
  pub fn new(
    eight_char: EightCharRef,
    gender: i64,
    sect: Option<i64>,
  ) -> YunRef {
    assert!(gender == 1 || gender == 0);

    let sect = sect.unwrap_or(1); // default 1
    let __lunar = eight_char.get_lunar();
    let __gender = gender as i8;
    let yang = __lunar.get_year_gan_index_exact() % 2 == 0;
    let man = __gender == 1;
    let __forward = (yang && man) || (!yang && !man);

    let prev_jie = __lunar.get_prev_jie(None).unwrap();
    let next_jie = __lunar.get_next_jie(None).unwrap();
    let current = __lunar.get_solar();

    let start = match __forward {
      true => current.clone(),
      _ => prev_jie.get_solar(),
    };
    let end = match __forward {
      true => next_jie.get_solar(),
      _ => current.clone(),
    };

    let hour = 0;

    let (__start_year, __start_month, __start_day, __start_hour) =
      match sect {
        2 => {
          let mut minutes = end.subtract_minute(start);
          let year = (minutes as f64 / 4320.) as i64;
          minutes = minutes - year * 4320;
          let month = (minutes as f64 / 360.) as i64;
          minutes = minutes - month * 360;
          let day = (minutes as f64 / 12.) as i64;
          minutes = minutes - day * 12;
          let hour = minutes * 2;
          (year, month, day, hour)
        }
        _ => {
          let end_time_zhi_index = match end.get_hour() == 23 {
            true => 11,
            _ => LunarUtil::get_time_zhi_index(
              &end.to_ymdhms().slice_by_char_idx(11, 16),
            ),
          };
          let start_time_zhi_index = match start.get_hour() == 23 {
            true => 11,
            _ => LunarUtil::get_time_zhi_index(
              &start.to_ymdhms().slice_by_char_idx(11, 16),
            ),
          };
          let mut hour_diff = end_time_zhi_index - start_time_zhi_index;
          let mut day_diff = end.subtract(start);
          if hour_diff < 0 {
            hour_diff = hour_diff + 12;
            day_diff = day_diff - 1;
          }
          let month_diff = (hour_diff as f64 * 10. / 30.) as i64;
          let mut month = day_diff * 4 + month_diff;
          let day = hour_diff * 10 - month_diff * 30;
          let year = (month as f64 / 12.) as i64;
          month = month - year * 12;
          (year, month, day, hour)
        }
      };

    Arc::new(Mutex::new(Self {
      __lunar,
      __gender,
      __forward,
      __start_year,
      __start_month,
      __start_day,
      __start_hour,
    }))
  }
}
