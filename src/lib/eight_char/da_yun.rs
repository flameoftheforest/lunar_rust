mod helper_traits;
pub use self::helper_traits::{DaYunRef, DaYunRefHelper};
use super::yun::{YunRef, YunRefHelper};
use crate::{
  lunar::{LunarRef, LunarRefHelper},
  solar::SolarRefHelper,
};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct DaYun {
  __yun: YunRef,
  __lunar: LunarRef,
  __index: i64,
  __start_year: i64,
  __start_age: i64,
  __end_year: i64,
  __end_age: i64,
}

impl DaYun {
  pub fn new(yun: YunRef, index: i64) -> DaYunRef {
    let __yun = yun.clone();
    let __lunar = yun.get_lunar();
    let __index = index;

    let birth_year = yun.get_lunar().get_solar().get_year();
    let year = yun.get_start_solar().get_year();
    let (__start_year, __start_age, __end_year, __end_age) =
      match index < 1 {
        true => (birth_year, 1, year - 1, year - birth_year),
        _ => {
          let add = (index - 1) * 10;
          let start_year = year + add;
          (
            start_year,
            start_year - birth_year + 1,
            start_year + 9,
            start_year + 9,
          )
        }
      };

    Arc::new(Mutex::new(Self {
      __yun,
      __lunar,
      __index,
      __start_year,
      __start_age,
      __end_year,
      __end_age,
    }))
  }
}
