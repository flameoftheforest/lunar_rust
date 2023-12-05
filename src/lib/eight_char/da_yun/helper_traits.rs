use super::DaYun;
use crate::{
  eight_char::{
    liu_nian::{LiuNian, LiuNianRef},
    xiao_yun::{XiaoYun, XiaoYunRef},
    yun::YunRefHelper,
  },
  lunar::{LunarRef, LunarRefHelper},
  util::lunar_util::{LunarUtil, JIA_ZI},
};
use std::sync::{Arc, Mutex};

pub type DaYunRef = Arc<Mutex<DaYun>>;

pub trait DaYunRefHelper {
  fn get_start_year(&self) -> i64;
  fn get_end_year(&self) -> i64;
  fn get_start_age(&self) -> i64;
  fn get_end_age(&self) -> i64;
  fn get_index(&self) -> i64;
  fn get_lunar(&self) -> LunarRef;
  fn get_gan_zhi(&self) -> String;
  fn get_xun(&self) -> String;
  fn get_xun_kong(&self) -> String;
  fn get_liu_nian(&self, n: Option<i64>) -> Vec<LiuNianRef>;
  fn get_xiao_yun(&self, n: Option<i64>) -> Vec<XiaoYunRef>;
}

impl DaYunRefHelper for DaYunRef {
  fn get_start_year(&self) -> i64 {
    self.lock().unwrap().__start_year
  }

  fn get_end_year(&self) -> i64 {
    self.lock().unwrap().__end_year
  }

  fn get_start_age(&self) -> i64 {
    self.lock().unwrap().__start_age
  }

  fn get_end_age(&self) -> i64 {
    self.lock().unwrap().__end_age
  }

  fn get_index(&self) -> i64 {
    self.lock().unwrap().__index
  }

  fn get_lunar(&self) -> LunarRef {
    self.lock().unwrap().__lunar.clone()
  }

  ///
  /// 获取干支
  ///
  /// ## Returns
  /// 干支: **String**
  ///
  fn get_gan_zhi(&self) -> String {
    let (index, lunar, yun) = {
      let s = self.lock().unwrap();
      (s.__index, s.__lunar.clone(), s.__yun.clone())
    };

    if index < 1 {
      return format!("");
    }
    let mut offset =
      LunarUtil::get_jia_zi_index(&lunar.get_month_in_gan_zhi_exact());
    offset = offset
      + match yun.is_forward() {
        true => index,
        _ => -index,
      };
    let size = JIA_ZI().len() as i64;

    offset = offset
      + match offset {
        d if d >= size => -size,
        d if d < 0 => size,
        _ => 0,
      };

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

  ///
  /// 获取流年
  ///
  /// ## Arguments
  /// n: **Option\<i64\>** - 轮数
  ///
  /// ## Returns
  /// 流年: **i64**
  ///
  fn get_liu_nian(&self, n: Option<i64>) -> Vec<LiuNianRef> {
    let mut n = n.unwrap_or(10);

    let (index, end_year, start_year) = {
      let s = self.lock().unwrap();
      (s.__index, s.__end_year, s.__start_year)
    };

    if index < 1 {
      n = end_year - start_year + 1;
    }
    let mut liu_nian = vec![];
    for i in 0..n {
      liu_nian.push(LiuNian::new(self.clone(), i));
    }
    liu_nian
  }

  ///
  /// 获取小运
  ///
  /// ## Arguments
  /// n: **Option\<i64\>** - 轮数 (default=10)
  ///
  /// ## Returns
  /// 小运:
  ///
  fn get_xiao_yun(&self, n: Option<i64>) -> Vec<XiaoYunRef> {
    let n = n.unwrap_or(10);
    let yun = {
      let s = self.lock().unwrap();
      s.__yun.clone()
    };
    let mut xiao_yun = vec![];
    for i in 0..n {
      xiao_yun.push(XiaoYun::new(self.clone(), i, yun.is_forward()));
    }
    xiao_yun
  }
}
