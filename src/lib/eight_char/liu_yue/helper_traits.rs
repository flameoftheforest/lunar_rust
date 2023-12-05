use super::LiuYue;
use crate::{
  eight_char::liu_nian::LiuNianRefHelper,
  util::{
    find_char_index::FindCharIndex,
    lunar_util::{LunarUtil, BASE_MONTH_ZHI_INDEX, GAN, MONTH, ZHI},
  },
};
use serde_json::Value;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
pub type LiuYueRef = Arc<Mutex<LiuYue>>;

pub trait RefHelper {
  fn get_index(&self) -> i64;
  fn get_month_in_chinese(&self) -> String;
  fn get_gan_zhi(&self) -> String;
  fn get_xun(&self) -> String;
  fn get_xun_kong(&self) -> String;
}

impl RefHelper for LiuYueRef {
  fn get_index(&self) -> i64 {
    self.lock().unwrap().__index
  }

  ///
  /// 获取中文的月
  ///
  /// ## Returns
  /// 中文月，如正: **String**
  ///
  fn get_month_in_chinese(&self) -> String {
    MONTH()[self.get_index() as usize + 1].to_string()
  }

  ///
  /// 获取干支
  ///
  /// 《五虎遁》<br>
  /// 甲己之年丙作首，<br>
  /// 乙庚之年戊为头，<br>
  /// 丙辛之年寻庚上，<br>
  /// 丁壬壬寅顺水流，<br>
  /// 若问戊癸何处走，<br>
  /// 甲寅之上好追求。<br>
  ///
  /// ## Returns
  /// 干支: String
  ///
  fn get_gan_zhi(&self) -> String {
    let (liu_nian, index) = {
      let s = self.lock().unwrap();
      (s.__liu_nian.clone(), s.__index)
    };
    let year_gan_zhi = liu_nian.get_gan_zhi();
    let year_gan = year_gan_zhi.slice_by_char_idx(0, 1);

    let year_gan_to_offset = {
      let mut hm = HashMap::new();
      let v: Value = serde_json::from_str(
        r#"{
        "甲": 2,
        "己": 2,
        "乙": 4,
        "庚": 4,
        "丙": 6,
        "辛": 6,
        "丁": 8,
        "壬": 8
      }"#,
      )
      .unwrap();
      for (key, value) in v.as_object().unwrap() {
        hm.insert(key.to_string(), value.as_i64().unwrap());
      }
      hm
    };
    let offset =
      year_gan_to_offset.get(&year_gan).unwrap_or(&0).clone();
    let gan = &GAN()[(index + offset) as usize % 10 + 1];
    let zhi =
      &ZHI()[(index + BASE_MONTH_ZHI_INDEX()) as usize % 12 + 1];
    format!("{}{}", gan, zhi)
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
}
