use super::{Foto, DEAD_YEAR};
use crate::{
  foto_festival::{FotoFestivalRef, FotoFestivalRefHelper},
  lunar::{LunarRef, LunarRefHelper},
  lunar_month::{self, LunarMonthRefHelper},
  solar::SolarRefHelper,
  util::{
    foto_util::{get_XIU, DAY_ZHAI_GUAN_YIN, FESTIVAL, OTHER_FESTIVAL},
    locked_ref_trait::LockRef,
    lunar_util::{
      ANIMAL, GONG, NUMBER, SHOU, XIU_LUCK, XIU_SONG, ZHENG,
    },
  },
};
use std::sync::{Arc, Mutex, MutexGuard};

pub type FotoRef = Arc<Mutex<Foto>>;
pub trait RefHelper: LockRef {
  fn get_lunar(&self) -> LunarRef;
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn get_day(&self) -> i64;
  fn get_year_in_chinese(&self) -> String;
  fn get_month_in_chinese(&self) -> String;
  fn get_day_in_chinese(&self) -> String;
  fn get_festivals(&self) -> Vec<FotoFestivalRef>;
  fn get_other_festivals(&self) -> Vec<String>;
  fn is_month_zhai(&self) -> bool;
  fn is_day_yang_gong(&self) -> bool;
  fn is_day_zhai_shuo_wang(&self) -> bool;
  fn is_day_zhai_six(&self) -> bool;
  fn is_day_zhai_ten(&self) -> bool;
  fn is_day_zhai_guan_yin(&self) -> bool;
  fn get_xiu(&self) -> String;
  fn get_xiu_luck(&self) -> String;
  fn get_xiu_song(&self) -> String;
  fn get_zheng(&self) -> String;
  fn get_animal(&self) -> String;
  fn get_gong(&self) -> String;
  fn get_shou(&self) -> String;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

impl RefHelper for FotoRef {
  fn get_lunar(&self) -> LunarRef {
    self.as_locked_ref().__lunar.clone()
  }

  fn get_year(&self) -> i64 {
    let lunar = { self.as_locked_ref().__lunar.clone() };
    let sy = lunar.get_solar().get_year();
    let mut y = sy - DEAD_YEAR;
    if sy == lunar.get_year() {
      y = y + 1;
    }
    y
  }

  fn get_month(&self) -> i64 {
    self.as_locked_ref().__lunar.get_month()
  }

  fn get_day(&self) -> i64 {
    self.as_locked_ref().__lunar.get_day()
  }

  fn get_year_in_chinese(&self) -> String {
    let y = format!("{}", self.get_year()).chars().collect::<Vec<_>>();
    let number = &NUMBER();

    y.iter().fold(format!(""), |mut acc, c| {
      let c: u32 = (*c).into();
      acc = format!("{}{}", acc, number[c as usize - 48]);
      acc
    })
  }

  fn get_month_in_chinese(&self) -> String {
    self.as_locked_ref().__lunar.get_month_in_chinese()
  }

  fn get_day_in_chinese(&self) -> String {
    self.as_locked_ref().__lunar.get_day_in_chinese()
  }

  fn get_festivals(&self) -> Vec<FotoFestivalRef> {
    let md = format!("{}-{}", self.get_month(), self.get_day());
    let festival = FESTIVAL();
    let found = festival.get(&md);
    match found {
      Some(v) => v.clone(),
      _ => [].to_vec(),
    }
  }

  ///
  /// 获取纪念日
  ///
  /// ## Returns
  /// 非正式的节日列表，如中元节: **Vec<String>**
  ///
  fn get_other_festivals(&self) -> Vec<String> {
    let md = format!("{}-{}", self.get_month(), self.get_day());
    let other_festival = OTHER_FESTIVAL();
    let found = other_festival.get(&md);
    match found {
      Some(v) => v.clone(),
      _ => [].to_vec(),
    }
  }

  fn is_month_zhai(&self) -> bool {
    match self.get_month() {
      d if 1 == d || 5 == d || 9 == d => true,
      _ => false,
    }
  }

  fn is_day_yang_gong(&self) -> bool {
    let festivals = self.get_festivals();
    let found = festivals.iter().find(|v| &v.get_name() == "杨公忌");
    found.is_some()
  }

  fn is_day_zhai_shuo_wang(&self) -> bool {
    match self.get_day() {
      d if 1 == d || 15 == d => true,
      _ => false,
    }
  }

  fn is_day_zhai_six(&self) -> bool {
    match self.get_day() {
      d if 8 == d
        || 14 == d
        || 15 == d
        || 23 == d
        || 29 == d
        || 30 == d =>
      {
        true
      }
      28 => {
        let m = lunar_month::from_ym(
          self.get_lunar().get_year(),
          self.get_month(),
        );
        m.get_day_count() != 30
      }
      _ => false,
    }
  }

  fn is_day_zhai_ten(&self) -> bool {
    let d = self.get_day();
    1 == d
      || 8 == d
      || 14 == d
      || 15 == d
      || 18 == d
      || 23 == d
      || 24 == d
      || 28 == d
      || 29 == d
      || 30 == d
  }

  fn is_day_zhai_guan_yin(&self) -> bool {
    let k = format!("{}-{}", self.get_month(), self.get_day());
    let day_zhai_guan_yin = DAY_ZHAI_GUAN_YIN();
    let found = day_zhai_guan_yin.iter().find(|v| v.to_string() == k);
    found.is_some()
  }

  fn get_xiu(&self) -> String {
    get_XIU(self.get_month(), self.get_day())
  }

  fn get_xiu_luck(&self) -> String {
    XIU_LUCK().get(&self.get_xiu()).unwrap().clone()
  }

  fn get_xiu_song(&self) -> String {
    XIU_SONG().get(&self.get_xiu()).unwrap().clone()
  }

  fn get_zheng(&self) -> String {
    ZHENG().get(&self.get_xiu()).unwrap().clone()
  }

  fn get_animal(&self) -> String {
    ANIMAL().get(&self.get_xiu()).unwrap().clone()
  }

  fn get_gong(&self) -> String {
    GONG().get(&self.get_xiu()).unwrap().clone()
  }

  fn get_shou(&self) -> String {
    SHOU().get(&self.get_gong()).unwrap().clone()
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
    let mut s = self.to_string();
    self.get_festivals().iter().for_each(|fest| {
      s = format!("{} ({})", s, fest.to_string());
    });
    s
  }
}

impl LockRef for FotoRef {
  type Output<'a> = MutexGuard<'a, Foto>;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
