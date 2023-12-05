use crate::{
  eight_char::{
    da_yun::DaYunRefHelper,
    liu_yue::{LiuYue, LiuYueRef},
  },
  lunar::LunarRefHelper,
  solar::SolarRefHelper,
  util::lunar_util::{LunarUtil, JIA_ZI},
};
use std::sync::{Arc, Mutex};

use super::LiuNian;

pub type LiuNianRef = Arc<Mutex<LiuNian>>;

pub trait RefHelper {
  fn get_index(&self) -> i64;
  fn get_year(&self) -> i64;
  fn get_age(&self) -> i64;
  fn get_gan_zhi(&self) -> String;
  fn get_xun(&self) -> String;
  fn get_xun_kong(&self) -> String;
  fn get_liu_yue(&self) -> Vec<LiuYueRef>;
}

impl RefHelper for LiuNianRef {
  fn get_index(&self) -> i64 {
    self.lock().unwrap().__index
  }

  fn get_year(&self) -> i64 {
    self.lock().unwrap().__year
  }

  fn get_age(&self) -> i64 {
    self.lock().unwrap().__age
  }

  ///
  /// 获取干支
  ///
  /// ## Returns
  /// 干支: **String**
  ///
  fn get_gan_zhi(&self) -> String {
    let (lunar, index, da_yun) = {
      let s = self.lock().unwrap();
      (s.__lunar.clone(), s.__index, s.__da_yun.clone())
    };
    let mut offset = LunarUtil::get_jia_zi_index(
      &lunar.get_jie_qi_table()["立春"]
        .get_lunar()
        .get_year_in_gan_zhi_exact(),
    ) + index;
    if da_yun.get_index() > 0 {
      offset = offset + da_yun.get_start_age() - 1;
    }
    offset = offset % JIA_ZI().len() as i64;
    JIA_ZI()[offset as usize].to_string()
  }

  ///
  /// 获取所在旬
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_gan_zhi())
  }

  ///
  /// 获取旬空(空亡)
  ///
  /// ## Returns
  /// 旬空(空亡): **String**
  ///
  fn get_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_gan_zhi())
  }

  ///
  /// 获取流月
  ///
  /// ## Returns
  /// 流月: i64
  ///
  fn get_liu_yue(&self) -> Vec<LiuYueRef> {
    let n = 12;
    let mut liu_yue = vec![];
    for i in 0..n {
      liu_yue.push(LiuYue::new(self.clone(), i));
    }
    liu_yue
  }
}
