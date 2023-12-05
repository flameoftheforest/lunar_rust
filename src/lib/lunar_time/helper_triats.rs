use crate::{
  lunar::LunarRefHelper,
  nine_star::{NineStar, NineStarRef},
  solar::SolarRefHelper,
  util::{
    locked_ref_trait::LockRef,
    lunar_util::{
      LunarUtil, CHONG, CHONG_GAN, CHONG_GAN_TIE, GAN, NAYIN,
      POSITION_CAI, POSITION_DESC, POSITION_FU, POSITION_FU_2,
      POSITION_XI, POSITION_YANG_GUI, POSITION_YIN_GUI, SHA, SHENGXIAO,
      TIAN_SHEN, TIAN_SHEN_TYPE, TIAN_SHEN_TYPE_LUCK, ZHI,
      ZHI_TIAN_SHEN_OFFSET,
    },
  },
};
use std::sync::{Arc, Mutex, MutexGuard};

use super::LunarTime;

pub type LunarTimeRef = Arc<Mutex<LunarTime>>;

pub trait RefHelper: LockRef {
  fn get_gan(&self) -> String;
  fn get_zhi(&self) -> String;
  fn get_gan_zhi(&self) -> String;
  fn get_sheng_xiao(&self) -> String;
  fn get_position_xi(&self) -> String;
  fn get_position_xi_desc(&self) -> String;
  fn get_position_yang_gui(&self) -> String;
  fn get_position_yang_gui_desc(&self) -> String;
  fn get_position_yin_gui(&self) -> String;
  fn get_position_yin_gui_desc(&self) -> String;
  fn get_position_fu(&self, sect: Option<i64>) -> String;
  fn get_position_fu_desc(&self, sect: Option<i64>) -> String;
  fn get_position_cai(&self) -> String;
  fn get_position_cai_desc(&self) -> String;
  fn get_chong(&self) -> String;
  fn get_chong_gan(&self) -> String;
  fn get_chong_gan_tie(&self) -> String;
  fn get_chong_sheng_xiao(&self) -> String;
  fn get_chong_desc(&self) -> String;
  fn get_sha(&self) -> String;
  fn get_na_yin(&self) -> String;
  fn get_tian_shen(&self) -> String;
  fn get_tian_shen_type(&self) -> String;
  fn get_tian_shen_luck(&self) -> String;
  fn get_yi(&self) -> Vec<String>;
  fn get_ji(&self) -> Vec<String>;
  fn get_nine_star(&self) -> NineStarRef;
  fn get_gan_index(&self) -> i64;
  fn get_zhi_index(&self) -> i64;
  fn to_string(&self) -> String;
  fn get_xun(&self) -> String;
  fn get_xun_kong(&self) -> String;
  fn get_min_hm(&self) -> String;
  fn get_max_hm(&self) -> String;
}

impl RefHelper for LunarTimeRef {
  fn get_gan(&self) -> String {
    GAN()[self.as_locked_ref().__gan_index as usize + 1].to_string()
  }

  fn get_zhi(&self) -> String {
    ZHI()[self.as_locked_ref().__zhi_index as usize + 1].to_string()
  }

  fn get_gan_zhi(&self) -> String {
    format!("{}{}", self.get_gan(), self.get_zhi())
  }

  fn get_sheng_xiao(&self) -> String {
    SHENGXIAO()[self.as_locked_ref().__zhi_index as usize + 1]
      .to_string()
  }

  fn get_position_xi(&self) -> String {
    POSITION_XI()[self.as_locked_ref().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_xi_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_xi())
      .unwrap()
      .clone()
  }

  fn get_position_yang_gui(&self) -> String {
    POSITION_YANG_GUI()[self.as_locked_ref().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_yang_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_yang_gui())
      .unwrap()
      .clone()
  }

  fn get_position_yin_gui(&self) -> String {
    POSITION_YIN_GUI()[self.as_locked_ref().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_yin_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_yin_gui())
      .unwrap()
      .clone()
  }

  fn get_position_fu(&self, sect: Option<i64>) -> String {
    let sect = sect.unwrap_or(2);
    let position_fu = match sect {
      1 => POSITION_FU.clone(),
      _ => POSITION_FU_2.clone(),
    };
    position_fu()[self.as_locked_ref().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_fu_desc(&self, sect: Option<i64>) -> String {
    POSITION_DESC()
      .get(&self.get_position_fu(sect))
      .unwrap()
      .clone()
  }

  fn get_position_cai(&self) -> String {
    POSITION_CAI()[self.as_locked_ref().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_cai_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_cai())
      .unwrap()
      .clone()
  }

  fn get_chong(&self) -> String {
    CHONG()[self.as_locked_ref().__zhi_index as usize].to_string()
  }

  fn get_chong_gan(&self) -> String {
    CHONG_GAN()[self.as_locked_ref().__gan_index as usize].to_string()
  }

  fn get_chong_gan_tie(&self) -> String {
    CHONG_GAN_TIE()[self.as_locked_ref().__gan_index as usize]
      .to_string()
  }

  fn get_chong_sheng_xiao(&self) -> String {
    let chong = self.get_chong();
    let zhi = ZHI();
    let found =
      zhi.iter().enumerate().find(|z| z.1.to_string() == chong);
    match found {
      Some((i, _)) => SHENGXIAO()[i].to_string(),
      _ => format!(""),
    }
  }

  fn get_chong_desc(&self) -> String {
    format!(
      "({}{}){}",
      self.get_chong_gan(),
      self.get_chong(),
      self.get_chong_sheng_xiao()
    )
  }

  fn get_sha(&self) -> String {
    SHA().get(&self.get_zhi()).unwrap().clone()
  }

  fn get_na_yin(&self) -> String {
    NAYIN().get(&self.get_gan_zhi()).unwrap().clone()
  }

  fn get_tian_shen(&self) -> String {
    let (zhi_index, lunar) = {
      let s = self.as_locked_ref();
      (s.__zhi_index, s.__lunar.clone())
    };
    TIAN_SHEN()[zhi_index as usize
      + ZHI_TIAN_SHEN_OFFSET()
        .get(&lunar.get_day_zhi_exact())
        .unwrap()
        .clone() as usize]
      .to_string()
  }

  fn get_tian_shen_type(&self) -> String {
    TIAN_SHEN_TYPE().get(&self.get_tian_shen()).unwrap().clone()
  }

  fn get_tian_shen_luck(&self) -> String {
    TIAN_SHEN_TYPE_LUCK()
      .get(&self.get_tian_shen_type())
      .unwrap()
      .clone()
  }

  ///
  /// 获取时宜
  ///
  /// ## Returns
  /// 宜: Vec<String>
  ///
  fn get_yi(&self) -> Vec<String> {
    LunarUtil::get_time_yi(
      &{ self.as_locked_ref().__lunar.clone() }
        .get_day_in_gan_zhi_exact(),
      &self.get_gan_zhi(),
    )
  }

  ///
  /// 获取时忌
  ///
  /// ## Returns
  /// 忌: Vec<String>
  ///
  fn get_ji(&self) -> Vec<String> {
    LunarUtil::get_time_ji(
      &{ self.as_locked_ref().__lunar.clone() }
        .get_day_in_gan_zhi_exact(),
      &self.get_gan_zhi(),
    )
  }

  fn get_nine_star(&self) -> NineStarRef {
    let (solar_date, jie_qi, lunar, zhi_index) = {
      let s = self.as_locked_ref();
      (
        s.__lunar.get_solar().to_naivedate(),
        &s.__lunar.get_jie_qi_table(),
        s.__lunar.clone(),
        s.__zhi_index,
      )
    };

    let asc = match solar_date {
      d if jie_qi.get("冬至").unwrap().to_naivedate() <= d
        && d < jie_qi.get("夏至").unwrap().to_naivedate() =>
      {
        true
      }
      _ => false,
    };

    let mut start = match asc {
      true => 7,
      _ => 3,
    };
    let day_zhi = lunar.get_day_zhi();
    if "子午卯酉".contains(&day_zhi) {
      start = match asc {
        true => 1,
        _ => 9,
      }
    } else if "辰戌丑未".contains(&day_zhi) {
      start = match asc {
        true => 4,
        _ => 6,
      }
    }
    let mut index = match asc {
      true => start + zhi_index - 1,
      _ => start - zhi_index - 1,
    };
    index = index
      + match index {
        d if d > 8 => -9,
        d if d < 0 => 9,
        _ => 0,
      };
    NineStar::from_index(index as usize)
  }

  fn get_gan_index(&self) -> i64 {
    self.as_locked_ref().__gan_index
  }

  fn get_zhi_index(&self) -> i64 {
    self.as_locked_ref().__zhi_index
  }

  fn to_string(&self) -> String {
    self.get_gan_zhi()
  }

  ///
  /// 获取时辰所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_gan_zhi())
  }

  ///
  /// 获取值时空亡
  ///
  /// ## Returns
  /// 空亡(旬空): String
  ///
  fn get_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_gan_zhi())
  }

  fn get_min_hm(&self) -> String {
    let hour = self.as_locked_ref().__lunar.get_hour();
    match hour {
      d if d < 1 => format!("00:00"),
      d if d > 22 => format!("23:00"),
      _ => {
        let hour = match hour % 2 {
          0 => hour - 1,
          _ => hour,
        };
        match hour < 10 {
          true => format!("0{}:00", hour),
          _ => format!("{}:00", hour),
        }
      }
    }
  }

  fn get_max_hm(&self) -> String {
    let hour = self.as_locked_ref().__lunar.get_hour();
    match hour {
      d if d < 1 => format!("00:59"),
      d if d > 22 => format!("23:59"),
      _ => {
        let hour = match hour % 2 {
          0 => hour,
          _ => hour + 1,
        };
        match hour < 10 {
          true => format!("0{}:59", hour),
          _ => format!("{}:59", hour),
        }
      }
    }
  }
}

impl LockRef for LunarTimeRef {
  type Output<'a> = MutexGuard<'a, LunarTime>;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
