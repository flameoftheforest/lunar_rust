use crate::{
  eight_char::da_yun::DaYunRefHelper,
  lunar::LunarRefHelper,
  util::lunar_util::{LunarUtil, JIA_ZI},
};
use std::sync::{Arc, Mutex};

use super::XiaoYun;

pub type XiaoYunRef = Arc<Mutex<XiaoYun>>;

pub trait RefHelper {
  fn get_index(&self) -> i64;
  fn get_year(&self) -> i64;
  fn get_age(&self) -> i64;
  fn get_gan_zhi(&self) -> String;
  fn get_xun(&self) -> String;
  fn get_xun_kong(&self) -> String;
}

impl RefHelper for XiaoYunRef {
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
    let (lunar, index, da_yun, forward) = {
      let s = self.lock().unwrap();
      (
        s.__lunar.clone(),
        s.__index,
        s.__da_yun.clone(),
        s.__forward,
      )
    };
    let mut offset =
      LunarUtil::get_jia_zi_index(&lunar.get_time_in_gan_zhi());
    let mut add = index + 1;
    if da_yun.get_index() > 0 {
      add = add + da_yun.get_start_age() - 1;
    }
    offset = offset
      + match forward {
        true => add,
        _ => -add,
      };
    let size = JIA_ZI().len() as i64;
    while offset < 0 {
      offset = offset + size;
    }
    offset = offset % size;
    JIA_ZI()[offset as usize].to_string()
  }

  ///
  /// 获取所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_gan_zhi())
  }

  ///
  /// 获取旬空(空亡)
  ///
  /// ## Returns
  /// 旬空(空亡): String
  ///
  fn get_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_gan_zhi())
  }
}
