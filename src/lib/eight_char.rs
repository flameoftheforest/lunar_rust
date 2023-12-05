pub mod da_yun;
pub mod liu_nian;
pub mod liu_yue;
pub mod xiao_yun;
pub mod yun;
use self::yun::YunRef;
use crate::{
  lunar::{Lunar, LunarRef, LunarRefHelper},
  util::{
    lunar_util::{
      LunarUtil, GAN, HE_GAN_5, HE_ZHI_6, JIA_ZI, NAYIN, SHI_SHEN,
      WU_XING_GAN, WU_XING_ZHI, ZHI, ZHI_HIDE_GAN,
    },
    mmacro::__static_funk,
  },
};
use serde_json::Value;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use yun::Yun;

#[derive(Clone, Debug)]
pub struct EightChar {
  __sect: i64,
  __lunar: Arc<Mutex<Lunar>>,
}

pub trait EightCharRefHelper {
  fn to_string(&self) -> String;
  fn get_sect(&self) -> i64;
  fn set_sect(&self, sect: i64);
  fn get_year(&self) -> String;
  fn get_year_gan(&self) -> String;
  fn get_year_zhi(&self) -> String;
  fn get_year_hide_gan(&self) -> Vec<String>;
  fn get_year_wu_xing(&self) -> String;
  fn get_year_na_yin(&self) -> String;
  fn get_year_shi_shen_gan(&self) -> String;
  fn get_year_shi_shen_zhi(&self) -> Vec<String>;
  fn get_day_gan_index(&self) -> i64;
  fn get_day_zhi_index(&self) -> i64;
  fn get_year_di_shi(&self) -> String;
  fn get_month(&self) -> String;
  fn get_month_gan(&self) -> String;
  fn get_month_zhi(&self) -> String;
  fn get_month_hide_gan(&self) -> Vec<String>;
  fn get_month_wu_xing(&self) -> String;
  fn get_month_na_yin(&self) -> String;
  fn get_month_shi_shen_gan(&self) -> String;
  fn get_month_shi_shen_zhi(&self) -> Vec<String>;
  fn get_month_di_shi(&self) -> String;
  fn get_day(&self) -> String;
  fn get_day_gan(&self) -> String;
  fn get_day_zhi(&self) -> String;
  fn get_day_hide_gan(&self) -> Vec<String>;
  fn get_day_wu_xing(&self) -> String;
  fn get_day_na_yin(&self) -> String;
  fn get_day_shi_shen_gan(&self) -> String;
  fn get_day_shi_shen_zhi(&self) -> Vec<String>;
  fn get_day_di_shi(&self) -> String;
  fn get_time(&self) -> String;
  fn get_time_gan(&self) -> String;
  fn get_time_zhi(&self) -> String;
  fn get_time_hide_gan(&self) -> Vec<String>;
  fn get_time_wu_xing(&self) -> String;
  fn get_time_na_yin(&self) -> String;
  fn get_time_shi_shen_gan(&self) -> String;
  fn get_time_shi_shen_zhi(&self) -> Vec<String>;
  fn get_time_di_shi(&self) -> String;
  fn get_tai_yuan(&self) -> String;
  fn get_tai_yuan_na_yin(&self) -> String;
  fn get_tai_xi(&self) -> String;
  fn get_tai_xi_na_yin(&self) -> String;
  fn get_ming_gong(&self) -> String;
  fn get_ming_gong_na_yin(&self) -> String;
  fn get_shen_gong(&self) -> String;
  fn get_shen_gong_na_yin(&self) -> String;
  fn get_lunar(&self) -> LunarRef;
  fn get_yun(self, gender: i64, sect: Option<i64>) -> YunRef;
  fn get_year_xun(&self) -> String;
  fn get_year_xun_kong(&self) -> String;
  fn get_month_xun(&self) -> String;
  fn get_month_xun_kong(&self) -> String;
  fn get_day_xun(&self) -> String;
  fn get_day_xun_kong(&self) -> String;
  fn get_time_xun(&self) -> String;
  fn get_time_xun_kong(&self) -> String;
}

pub type EightCharRef = Arc<Mutex<EightChar>>;

trait __EightCharRefHelper {
  fn __get_shi_shen_zhi(self, zhi: &str) -> Vec<String>;
  fn __get_di_shi(&self, zhi_index: i64) -> String;
  fn __get_sect(&self) -> i64;
}

impl __EightCharRefHelper for EightCharRef {
  fn __get_shi_shen_zhi(self, zhi: &str) -> Vec<String> {
    ZHI_HIDE_GAN()
      .get(zhi)
      .unwrap()
      .iter()
      .map(|gan| {
        SHI_SHEN()
          .get(&format!("{}{}", self.get_day_gan(), gan))
          .unwrap()
          .clone()
      })
      .collect::<Vec<_>>()
  }

  fn __get_di_shi(&self, zhi_index: i64) -> String {
    let mut index = __CHANG_SHENG_OFFSET()
      .get(&self.get_day_gan())
      .unwrap()
      .clone()
      + match self.get_day_gan_index() % 2 {
        0 => zhi_index,
        _ => -zhi_index,
      };

    index = index
      + match index {
        v if v >= 12 => -12,
        v if v < 0 => 12,
        _ => 0,
      };
    __CHANG_SHENG()[index as usize].to_string()
  }

  fn __get_sect(&self) -> i64 {
    self.lock().unwrap().__sect
  }
}

impl EightCharRefHelper for Arc<Mutex<EightChar>> {
  fn to_string(&self) -> String {
    format!(
      "{} {} {} {}",
      self.get_year(),
      self.get_month(),
      self.get_day(),
      self.get_time()
    )
  }

  fn get_sect(&self) -> i64 {
    return self.__get_sect();
  }

  fn set_sect(&self, sect: i64) {
    self.lock().unwrap().__sect = sect;
  }

  ///
  /// 获取年柱
  ///
  /// ## Returns
  /// 年柱: String
  ///
  fn get_year(&self) -> String {
    self.get_lunar().get_year_in_gan_zhi_exact()
  }

  ///
  /// 获取年干
  ///
  /// ## Returns
  /// 天干: String
  ///
  fn get_year_gan(&self) -> String {
    self.get_lunar().get_year_gan_exact()
  }

  ///
  /// 获取年支
  ///
  /// ## Returns
  /// 地支: String
  ///
  fn get_year_zhi(&self) -> String {
    self.get_lunar().get_year_zhi_exact()
  }

  ///
  /// 获取年柱地支藏干，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 天干: Vec<String>
  ///
  fn get_year_hide_gan(&self) -> Vec<String> {
    ZHI_HIDE_GAN().get(&self.get_year_zhi()).unwrap().clone()
  }

  ///
  /// 获取年柱五行
  ///
  /// ## Returns
  /// 五行: String
  ///
  fn get_year_wu_xing(&self) -> String {
    format!(
      "{}{}",
      WU_XING_GAN().get(&self.get_year_gan()).unwrap(),
      WU_XING_ZHI().get(&self.get_year_zhi()).unwrap()
    )
  }

  ///
  /// 获取年柱纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_year_na_yin(&self) -> String {
    NAYIN().get(&self.get_year()).unwrap().clone()
  }

  ///
  /// 获取年柱天干十神
  ///
  /// ## Returns
  /// 十神: String
  ///
  fn get_year_shi_shen_gan(&self) -> String {
    SHI_SHEN()
      .get(&format!("{}{}", self.get_day_gan(), self.get_year_gan()))
      .unwrap()
      .clone()
  }

  ///
  /// 获取年柱地支十神，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 十神: String
  ///
  fn get_year_shi_shen_zhi(&self) -> Vec<String> {
    self.clone().__get_shi_shen_zhi(&self.get_year_zhi())
  }

  fn get_day_gan_index(&self) -> i64 {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_gan_index_exact2(),
      _ => self.get_lunar().get_day_gan_index_exact(),
    }
  }

  fn get_day_zhi_index(&self) -> i64 {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_zhi_index_exact2(),
      _ => self.get_lunar().get_day_zhi_index_exact(),
    }
  }

  ///
  /// 获取年柱地势（长生十二神）
  ///
  /// ## Returns
  /// 地势: String
  ///
  fn get_year_di_shi(&self) -> String {
    self.__get_di_shi(self.get_lunar().get_year_zhi_index_exact())
  }

  ///
  /// 获取月柱
  ///
  /// ## Returns
  /// 月柱: String
  ///
  fn get_month(&self) -> String {
    self.get_lunar().get_month_in_gan_zhi_exact()
  }

  ///
  /// 获取月干
  ///
  /// ## Returns
  /// 天干: String
  ///
  fn get_month_gan(&self) -> String {
    self.get_lunar().get_month_gan_exact()
  }

  ///
  /// 获取月支
  ///
  /// ## Returns
  /// 地支: String
  ///
  fn get_month_zhi(&self) -> String {
    self.get_lunar().get_month_zhi_exact()
  }

  ///
  /// 获取月柱地支藏干，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 天干: Vec<String>
  ///
  fn get_month_hide_gan(&self) -> Vec<String> {
    ZHI_HIDE_GAN().get(&self.get_month_zhi()).unwrap().clone()
  }

  ///
  /// 获取月柱五行
  ///
  /// ## Returns
  /// 五行: String
  ///
  fn get_month_wu_xing(&self) -> String {
    format!(
      "{}{}",
      WU_XING_GAN().get(&self.get_month_gan()).unwrap(),
      WU_XING_ZHI().get(&self.get_month_zhi()).unwrap()
    )
  }

  ///
  /// 获取月柱纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_month_na_yin(&self) -> String {
    NAYIN().get(&self.get_month()).unwrap().clone()
  }

  ///
  /// 获取月柱天干十神
  ///
  /// ## Returns
  /// 十神: String
  ///
  fn get_month_shi_shen_gan(&self) -> String {
    SHI_SHEN()
      .get(&format!("{}{}", self.get_day_gan(), self.get_month_gan()))
      .unwrap()
      .clone()
  }

  ///
  /// 获取月柱地支十神，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 十神: Vec<String>
  ///
  fn get_month_shi_shen_zhi(&self) -> Vec<String> {
    self.clone().__get_shi_shen_zhi(&self.get_month_zhi())
  }

  ///
  /// 获取月柱地势（长生十二神）
  ///
  /// ## Returns
  /// 地势: String
  ///
  fn get_month_di_shi(&self) -> String {
    self.__get_di_shi(self.get_lunar().get_month_zhi_index_exact())
  }

  ///
  /// 获取日柱
  ///
  /// ## Returns
  /// 日柱: String
  ///
  fn get_day(&self) -> String {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_in_gan_zhi_exact2(),
      _ => self.get_lunar().get_day_in_gan_zhi_exact(),
    }
  }

  ///
  /// 获取日干
  ///
  /// ## Returns
  /// 天干: String
  ///
  fn get_day_gan(&self) -> String {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_gan_exact2(),
      _ => self.get_lunar().get_day_gan_exact(),
    }
  }

  ///
  /// 获取日支
  ///
  /// ## Returns
  /// 地支: String
  ///
  fn get_day_zhi(&self) -> String {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_zhi_exact2(),
      _ => self.get_lunar().get_day_zhi_exact(),
    }
  }

  ///
  /// 获取日柱地支藏干，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 天干: Vec<String>
  ///
  fn get_day_hide_gan(&self) -> Vec<String> {
    ZHI_HIDE_GAN().get(&self.get_day_zhi()).unwrap().clone()
  }

  ///
  /// 获取日柱五行
  ///
  /// ## Returns
  /// 五行
  ///
  fn get_day_wu_xing(&self) -> String {
    format!(
      "{}{}",
      WU_XING_GAN().get(&self.get_day_gan()).unwrap(),
      WU_XING_ZHI().get(&self.get_day_zhi()).unwrap()
    )
  }

  ///
  /// 获取日柱纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_day_na_yin(&self) -> String {
    NAYIN().get(&self.get_day()).unwrap().clone()
  }

  ///
  /// 获取日柱天干十神，也称日元、日干
  ///
  /// ## Returns
  /// 十神: String
  ///
  fn get_day_shi_shen_gan(&self) -> String {
    "日主".to_string()
  }

  ///
  /// 获取日柱地支十神，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 十神: Vec<String>
  ///
  fn get_day_shi_shen_zhi(&self) -> Vec<String> {
    self.clone().__get_shi_shen_zhi(&self.get_day_zhi())
  }

  ///
  /// 获取日柱地势（长生十二神）
  ///
  /// ## Returns
  /// 地势: String
  ///
  fn get_day_di_shi(&self) -> String {
    self.__get_di_shi(self.get_day_zhi_index())
  }

  ///
  /// 获取时柱
  ///
  /// ## Returns
  /// 时柱: String
  ///
  fn get_time(&self) -> String {
    self.get_lunar().get_time_in_gan_zhi()
  }

  ///
  /// 获取时干
  ///
  /// ## Returns
  /// 天干: String
  ///
  fn get_time_gan(&self) -> String {
    self.get_lunar().get_time_gan()
  }

  ///
  /// 获取时支
  ///
  /// ## Returns
  /// 地支: String
  ///
  fn get_time_zhi(&self) -> String {
    self.get_lunar().get_time_zhi()
  }

  ///
  /// 获取时柱地支藏干，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 天干: Vec<String>
  ///
  fn get_time_hide_gan(&self) -> Vec<String> {
    ZHI_HIDE_GAN().get(&self.get_time_zhi()).unwrap().clone()
  }

  ///
  /// 获取时柱五行
  ///
  /// ## Returns
  /// 五行: String
  /// """
  fn get_time_wu_xing(&self) -> String {
    format!(
      "{}{}",
      WU_XING_GAN().get(&self.get_time_gan()).unwrap(),
      WU_XING_ZHI().get(&self.get_time_zhi()).unwrap()
    )
  }

  ///
  /// 获取时柱纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_time_na_yin(&self) -> String {
    NAYIN().get(&self.get_time()).unwrap().clone()
  }

  ///
  /// 获取时柱天干十神
  ///
  /// ## Returns
  /// 十神: String
  ///
  fn get_time_shi_shen_gan(&self) -> String {
    SHI_SHEN()
      .get(&format!("{}{}", self.get_day_gan(), self.get_time_gan()))
      .unwrap()
      .clone()
  }

  ///
  /// 获取时柱地支十神，由于藏干分主气、余气、杂气，所以返回结果可能为1到3个元素
  ///
  /// ## Returns
  /// 十神: String
  ///
  fn get_time_shi_shen_zhi(&self) -> Vec<String> {
    self.clone().__get_shi_shen_zhi(&self.get_time_zhi())
  }

  ///
  /// 获取时柱地势（长生十二神）
  ///
  /// ## Returns
  /// 地势: String
  ///
  fn get_time_di_shi(&self) -> String {
    self.__get_di_shi(self.get_lunar().get_time_zhi_index())
  }

  ///
  /// 获取胎元
  ///
  /// ## Returns
  /// 胎元: String
  ///
  fn get_tai_yuan(&self) -> String {
    let mut gan_index =
      self.get_lunar().get_month_gan_index_exact() + 1;
    if gan_index >= 10 {
      gan_index = gan_index - 10;
    }
    let mut zhi_index =
      self.get_lunar().get_month_zhi_index_exact() + 3;
    if zhi_index >= 12 {
      zhi_index = zhi_index - 12;
    }
    format!(
      "{}{}",
      GAN()[gan_index as usize + 1],
      ZHI()[zhi_index as usize + 1]
    )
  }

  ///
  /// 获取胎元纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_tai_yuan_na_yin(&self) -> String {
    NAYIN().get(&self.get_tai_yuan()).unwrap().clone()
  }

  ///
  /// 获取胎息
  ///
  /// ## Returns
  /// 胎息: String
  ///
  fn get_tai_xi(&self) -> String {
    let gan_index = {
      match self.__get_sect() {
        2 => self.get_lunar().get_day_gan_index_exact2(),
        _ => self.get_lunar().get_day_gan_index_exact(),
      }
    };
    let zhi_index = {
      match self.__get_sect() {
        2 => self.get_lunar().get_day_zhi_index_exact2(),
        _ => self.get_lunar().get_day_zhi_index_exact(),
      }
    };
    format!(
      "{}{}",
      HE_GAN_5()[gan_index as usize],
      HE_ZHI_6()[zhi_index as usize]
    )
  }

  ///
  /// 获取胎息纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_tai_xi_na_yin(&self) -> String {
    NAYIN().get(&self.get_tai_xi()).unwrap().clone()
  }

  ///
  /// 获取命宫
  ///
  /// ## Returns
  /// 命宫: String
  ///
  fn get_ming_gong(&self) -> String {
    let mut month_zhi_index = 0;
    let mut time_zhi_index = 0;

    let lunar_month_zhi = self.get_lunar().get_month_zhi_exact();
    let lunar_time_zhi = self.get_lunar().get_time_zhi();
    __MONTH_ZHI().iter().enumerate().for_each(|(i, zhi)| {
      if lunar_month_zhi == *zhi {
        month_zhi_index = i;
      }
      if lunar_time_zhi == *zhi {
        time_zhi_index = i;
      }
    });

    let mut zhi_index = 26 - (month_zhi_index + time_zhi_index);
    if zhi_index > 12 {
      zhi_index = zhi_index - 12;
    }

    let mut jia_zi_index =
      LunarUtil::get_jia_zi_index(&self.get_month())
        - (month_zhi_index - zhi_index) as i64;
    if jia_zi_index >= 60 {
      jia_zi_index = jia_zi_index - 60;
    }
    if jia_zi_index < 0 {
      jia_zi_index = jia_zi_index + 60;
    }
    JIA_ZI()[jia_zi_index as usize].to_string()
  }

  ///
  /// 获取命宫纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_ming_gong_na_yin(&self) -> String {
    NAYIN().get(&self.get_ming_gong()).unwrap().clone()
  }

  ///
  /// 获取身宫
  ///
  /// ## Returns
  /// 身宫: String
  ///
  fn get_shen_gong(&self) -> String {
    let mut month_zhi_index = 0;
    let mut time_zhi_index = 0;

    let lunar_month_zhi = self.get_lunar().get_month_zhi_exact();
    let lunar_time_zhi = self.get_lunar().get_time_zhi();
    __MONTH_ZHI().iter().enumerate().for_each(|(i, zhi)| {
      if lunar_month_zhi == *zhi {
        month_zhi_index = i;
      }
      if lunar_time_zhi == *zhi {
        time_zhi_index = i;
      }
    });

    let mut zhi_index = 2 + month_zhi_index + time_zhi_index;
    if zhi_index > 12 {
      zhi_index = zhi_index - 12;
    }

    let mut jia_zi_index =
      LunarUtil::get_jia_zi_index(&self.get_month())
        - (month_zhi_index as i64 - zhi_index as i64);
    if jia_zi_index >= 60 {
      jia_zi_index = jia_zi_index - 60;
    }
    if jia_zi_index < 0 {
      jia_zi_index = jia_zi_index + 60;
    }
    JIA_ZI()[jia_zi_index as usize].to_string()
  }

  ///
  /// 获取身宫纳音
  ///
  /// ## Returns
  /// 纳音: String
  ///
  fn get_shen_gong_na_yin(&self) -> String {
    NAYIN().get(&self.get_shen_gong()).unwrap().clone()
  }

  fn get_lunar(&self) -> LunarRef {
    self.lock().unwrap().__lunar.clone()
  }

  /// """
  /// 获取运
  ///
  /// ## Parameters
  /// gender: i64 - 性别：1男，0女
  /// sect: Option<i64> - 流派：1按天数和时辰数计算，3天1年，1天4个月，1时辰10天；2按分钟数计算
  ///
  /// ## Returns
  /// 运: Yun
  ///
  fn get_yun(self, gender: i64, sect: Option<i64>) -> YunRef {
    Yun::new(self, gender, sect)
  }

  ///
  /// 获取年柱所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_year_xun(&self) -> String {
    self.get_lunar().get_year_xun_exact()
  }

  ///
  /// 获取年柱旬空(空亡)
  ///
  /// ## Returns
  /// 旬空(空亡)
  ///
  fn get_year_xun_kong(&self) -> String {
    self.get_lunar().get_year_xun_kong_exact()
  }

  ///
  /// 获取月柱所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_month_xun(&self) -> String {
    self.get_lunar().get_month_xun_exact()
  }

  ///
  /// 获取月柱旬空(空亡)
  ///
  /// ## Returns
  /// 旬空(空亡): String
  ///
  fn get_month_xun_kong(&self) -> String {
    return self.get_lunar().get_month_xun_kong_exact();
  }

  ///
  /// 获取日柱所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_day_xun(&self) -> String {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_xun_exact2(),
      _ => self.get_lunar().get_day_xun_exact(),
    }
  }

  ///
  /// 获取日柱旬空(空亡)
  ///
  /// ## Returns
  /// 旬空(空亡): String
  ///
  fn get_day_xun_kong(&self) -> String {
    match self.__get_sect() {
      2 => self.get_lunar().get_day_xun_kong_exact2(),
      _ => self.get_lunar().get_day_xun_kong_exact(),
    }
  }

  ///
  /// 获取时柱所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_time_xun(&self) -> String {
    self.get_lunar().get_time_xun()
  }

  ///
  /// 获取时柱旬空(空亡)
  ///
  /// ## Returns
  /// 旬空(空亡): String
  ///
  fn get_time_xun_kong(&self) -> String {
    self.get_lunar().get_time_xun_kong()
  }
}

impl EightChar {
  pub fn from_lunar(lunar: LunarRef) -> EightCharRef {
    Arc::new(Mutex::new(Self::__new(lunar)))
  }

  fn __new(lunar: LunarRef) -> Self {
    Self {
      __sect: 2,
      __lunar: lunar,
    }
  }
}

__static_funk!(
  __MONTH_ZHI,
  Vec<String>,
  [
    "", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
    "子", "丑"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  __CHANG_SHENG,
  Vec<String>,
  [
    "长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓",
    "绝", "胎", "养"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(__CHANG_SHENG_OFFSET, HashMap<String, i64>, {
  let v: Value = serde_json::from_str(r#"{
    "甲": 1,
    "丙": 10,
    "戊": 10,
    "庚": 7,
    "壬": 4,
    "乙": 6,
    "丁": 9,
    "己": 9,
    "辛": 0,
    "癸": 3
  }"#).unwrap();
  let hm = v.as_object()
    .unwrap()
    .iter()
    .map(|(key, value)| {
      (key.to_string(), value.as_i64().unwrap())
    })
    .collect::<HashMap<_,_>>();
  hm
});
