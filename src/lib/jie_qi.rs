// use lazy_static::lazy_static;

use crate::{lunar::Lunar, solar::SolarRef};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct JieQi {
  __name: String,
  __jie: bool,
  __qi: bool,
  __solar: SolarRef,
}

pub type JieQiRef = Arc<Mutex<JieQi>>;

pub trait JieQiRefHelper {
  fn get_name(&self) -> String;
  fn set_name(&self, name: &str);
  fn get_solar(&self) -> SolarRef;
  fn set_solar(&self, solar: &SolarRef);
  fn is_jie(&self) -> bool;
  fn is_qi(&self) -> bool;
  fn to_string(&self) -> String;
}

impl JieQi {
  pub fn new(name: &str, solar: &SolarRef) -> Arc<Mutex<Self>> {
    let r = Self {
      __name: name.to_string(),
      __jie: false,
      __qi: false,
      __solar: solar.clone(),
    };
    let r = Arc::new(Mutex::new(r));
    r.set_name(name);
    r
  }
}

impl JieQiRefHelper for Arc<Mutex<JieQi>> {
  ///
  /// 获取名称
  ///
  /// ## Returns
  /// + 名称: String
  ///
  fn get_name(&self) -> String {
    self.lock().unwrap().__name.clone()
  }

  ///
  /// 设置名称
  ///
  /// ## Arguments
  /// + name: **&str** - 名称
  ///
  fn set_name(&self, name: &str) {
    let mut s = self.lock().unwrap();
    s.__name = name.to_string();
    let jie_qi = Lunar::get_jie_qi_in_use().clone();

    jie_qi.iter().enumerate().find(|(i, jq)| {
      if name == jq.as_str() {
        match i % 2 {
          0 => s.__qi = true,
          _ => s.__jie = true,
        }
        return true;
      }
      false
    });
  }

  ///
  /// 获取阳历日期
  ///
  /// ## Returns
  /// + 阳历日期: **Solar**
  ///
  fn get_solar(&self) -> SolarRef {
    self.lock().unwrap().__solar.clone()
  }

  ///
  /// 设置阳历日期
  /// + solar: **&Solar** - 阳历日期
  ///
  fn set_solar(&self, solar: &SolarRef) {
    let mut s = self.lock().unwrap();
    s.__solar = solar.clone();
  }

  ///
  /// 是否节令
  ///
  /// ## Returns
  /// + true/false
  ///
  fn is_jie(&self) -> bool {
    self.lock().unwrap().__jie
  }

  /// """
  /// 是否气令
  ///
  /// ## Returns
  /// + true/false
  /// """
  fn is_qi(&self) -> bool {
    self.lock().unwrap().__qi
  }

  fn to_string(&self) -> String {
    self.lock().unwrap().__name.clone()
  }
}

// lazy_static!(
//   static ref __JIEQI
// )
