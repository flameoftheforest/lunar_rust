use crate::{
  lunar::{LunarRef, LunarRefHelper},
  tao_festival::{self, TaoFestivalRef},
  util::{self, locked_ref_trait::LockRef, lunar_util},
};
use std::sync::{Arc, Mutex, MutexGuard};

use super::{Tao, BIRTH_YEAR};

pub type TaoRef = Arc<Mutex<Tao>>;

pub trait RefHelper: LockRef {
  fn get_lunar(&self) -> LunarRef;
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn get_day(&self) -> i64;
  fn get_year_in_chinese(&self) -> String;
  fn get_month_in_chinese(&self) -> String;
  fn get_day_in_chinese(&self) -> String;
  fn get_festivals(&self) -> Vec<TaoFestivalRef>;
  fn is_day_san_hui(&self) -> bool;
  fn is_day_san_yuan(&self) -> bool;
  fn is_day_ba_jie(&self) -> bool;
  fn is_day_wu_la(&self) -> bool;
  fn is_day_ba_hui(&self) -> bool;
  fn is_day_ming_wu(&self) -> bool;
  fn is_day_an_wu(&self) -> bool;
  fn is_day_wu(&self) -> bool;
  fn is_day_tian_she(&self) -> bool;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

trait __RefHelper: LockRef {
  fn __is_day_in(&self, days: &Vec<String>) -> bool;
}

impl RefHelper for TaoRef {
  fn get_lunar(&self) -> LunarRef {
    self.as_locked_ref().__lunar.clone()
  }

  fn get_year(&self) -> i64 {
    self.get_lunar().get_year() - BIRTH_YEAR
  }

  fn get_month(&self) -> i64 {
    self.get_lunar().get_month()
  }

  fn get_day(&self) -> i64 {
    self.get_lunar().get_day()
  }

  fn get_year_in_chinese(&self) -> String {
    let y = self.get_year().to_string().chars().collect::<Vec<_>>();
    let mut s = vec![];
    y.iter().for_each(|yy| {
      s.push(
        lunar_util::NUMBER()
          [yy.to_string().parse::<i64>().unwrap() as usize]
          .clone(),
      );
    });
    s.join("")
  }

  fn get_month_in_chinese(&self) -> String {
    self.get_lunar().get_month_in_chinese()
  }

  fn get_day_in_chinese(&self) -> String {
    self.get_lunar().get_day_in_chinese()
  }

  fn get_festivals(&self) -> Vec<TaoFestivalRef> {
    let md = format!("{}-{}", self.get_month(), self.get_day());
    let festival = util::tao_util::FESTIVAL();
    let found = festival.get(&md);
    let mut fesitvals = match found {
      Some(f) => f.clone(),
      _ => vec![],
    };
    let jq = self.get_lunar().get_jie();
    match jq.as_str() {
      "冬至" => fesitvals.push(tao_festival::new_1("元始天尊圣诞")),
      "夏至" => fesitvals.push(tao_festival::new_1("灵宝天尊圣诞")),
      _ => (),
    };
    // 八节日
    match util::tao_util::BA_JIE().get(&jq) {
      Some(f) => fesitvals.push(tao_festival::new_1(f.as_str())),
      _ => (),
    };
    // 八会日
    let gz = self.get_lunar().get_day_in_gan_zhi();
    match util::tao_util::BA_HUI().get(&gz) {
      Some(f) => fesitvals.push(tao_festival::new_1(f.as_str())),
      _ => (),
    };
    fesitvals
  }

  fn is_day_san_hui(&self) -> bool {
    self.__is_day_in(&util::tao_util::SAN_HUI())
  }

  fn is_day_san_yuan(&self) -> bool {
    self.__is_day_in(&util::tao_util::SAN_YUAN())
  }

  fn is_day_ba_jie(&self) -> bool {
    util::tao_util::BA_JIE()
      .get(&self.get_lunar().get_jie_qi())
      .is_some()
  }

  fn is_day_wu_la(&self) -> bool {
    self.__is_day_in(&util::tao_util::WU_LA())
  }

  fn is_day_ba_hui(&self) -> bool {
    util::tao_util::BA_HUI()
      .get(&self.get_lunar().get_day_in_gan_zhi())
      .is_some()
  }

  fn is_day_ming_wu(&self) -> bool {
    "戊" == &self.get_lunar().get_day_gan()
  }

  fn is_day_an_wu(&self) -> bool {
    util::tao_util::AN_WU()[self.get_month() as usize - 1]
      == self.get_lunar().get_day_zhi()
  }

  fn is_day_wu(&self) -> bool {
    self.is_day_ming_wu() || self.is_day_an_wu()
  }

  fn is_day_tian_she(&self) -> bool {
    let mz = self.get_lunar().get_month_zhi();
    let dgz = self.get_lunar().get_day_in_gan_zhi();
    match mz.as_str() {
      d if "寅卯辰".contains(d) => &dgz == "戊寅",
      d if "巳午未".contains(d) => &dgz == "甲午",
      d if "申酉戌".contains(d) => &dgz == "戊申",
      d if "亥子丑".contains(d) => &dgz == "甲子",
      _ => false,
    }
  }

  fn to_string(&self) -> String {
    format!(
      "{}年{}月{}",
      self.get_year_in_chinese(),
      self.get_month_in_chinese(),
      self.get_day_in_chinese()
    )
  }

  fn to_full_string(&self) -> String {
    format!(
      "道歷{}年，天运{}年，{}月，{}日。{}月{}日，{}時。",
      self.get_year_in_chinese(),
      self.get_lunar().get_year_in_gan_zhi(),
      self.get_lunar().get_month_in_gan_zhi(),
      self.get_lunar().get_day_in_gan_zhi(),
      self.get_month_in_chinese(),
      self.get_day_in_chinese(),
      self.get_lunar().get_time_zhi()
    )
  }
}

impl __RefHelper for TaoRef {
  fn __is_day_in(&self, days: &Vec<String>) -> bool {
    let md = format!("{}-{}", self.get_month(), self.get_day());
    days.iter().find(|d| **d == md).is_some()
  }
}

impl LockRef for TaoRef {
  type Output<'a> = MutexGuard<'a, Tao>;

  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
