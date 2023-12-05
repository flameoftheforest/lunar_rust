use super::{
  helper_traits::__LunarRefHelper, Lunar, LunarRef, LunarRefHelper,
  JIE_QI,
};
use crate::{
  eight_char::{EightChar, EightCharRef, EightCharRefHelper},
  foto::{self, FotoRef},
  fu::{Fu, FuRef},
  jie_qi::{JieQi, JieQiRef, JieQiRefHelper},
  lunar::JIE_QI_IN_USE,
  lunar_time::{LunarTime, LunarTimeRef},
  nine_star::{NineStar, NineStarRef},
  shu_jiu::{ShuJiu, ShuJiuRef},
  solar::{self, SolarRef, SolarRefHelper},
  tao::{self, TaoRef},
  util::{
    lunar_util::{
      LunarUtil, ANIMAL, CHONG, CHONG_GAN, CHONG_GAN_TIE, DAY,
      FESTIVAL, GAN, GONG, HOU, LIU_YAO, LU, MONTH, NAYIN, NUMBER,
      OTHER_FESTIVAL, PENG_ZU_GAN, PENG_ZU_ZHI, POSITION_CAI,
      POSITION_DESC, POSITION_FU, POSITION_FU_2, POSITION_TAI_DAY,
      POSITION_TAI_MONTH, POSITION_TAI_SUI_YEAR, POSITION_XI,
      POSITION_YANG_GUI, POSITION_YIN_GUI, SEASON, SHA, SHENGXIAO,
      SHOU, TIAN_SHEN, TIAN_SHEN_TYPE, TIAN_SHEN_TYPE_LUCK, WU_HOU,
      XIU, XIU_LUCK, XIU_SONG, YUE_XIANG, ZHENG, ZHI,
      ZHI_TIAN_SHEN_OFFSET, ZHI_XING,
    },
    solar_util::{self, SolarUtilRefHelper},
  },
};
use std::collections::HashMap;

impl LunarRefHelper for LunarRef {
  fn get_year(&self) -> i64 {
    self.lock().unwrap().__year
  }

  fn get_month(&self) -> i64 {
    self.lock().unwrap().__month
  }

  fn get_day(&self) -> i64 {
    self.lock().unwrap().__day
  }

  fn get_hour(&self) -> i64 {
    self.lock().unwrap().__hour
  }

  fn get_minute(&self) -> i64 {
    self.lock().unwrap().__minute
  }

  fn get_second(&self) -> i64 {
    self.lock().unwrap().__second
  }

  fn get_solar(&self) -> SolarRef {
    self.lock().unwrap().__solar.clone()
  }

  fn get_year_gan(&self) -> String {
    GAN()[self.lock().unwrap().__year_gan_index as usize + 1]
      .to_string()
  }

  fn get_year_gan_by_li_chun(&self) -> String {
    GAN()[self.lock().unwrap().__year_gan_index_by_li_chun as usize + 1]
      .to_string()
  }

  fn get_year_gan_exact(&self) -> String {
    GAN()[self.lock().unwrap().__year_gan_index_exact as usize + 1]
      .to_string()
  }

  fn get_year_zhi(&self) -> String {
    ZHI()[self.lock().unwrap().__year_zhi_index as usize + 1]
      .to_string()
  }

  fn get_year_zhi_by_li_chun(&self) -> String {
    ZHI()[self.lock().unwrap().__year_zhi_index_by_li_chun as usize + 1]
      .to_string()
  }

  fn get_year_zhi_exact(&self) -> String {
    ZHI()[self.lock().unwrap().__year_zhi_index_exact as usize + 1]
      .to_string()
  }

  fn get_year_in_gan_zhi(&self) -> String {
    format!("{}{}", self.get_year_gan(), self.get_year_zhi())
  }

  fn get_year_in_gan_zhi_by_li_chun(&self) -> String {
    format!(
      "{}{}",
      self.get_year_gan_by_li_chun(),
      self.get_year_zhi_by_li_chun()
    )
  }

  fn get_year_in_gan_zhi_exact(&self) -> String {
    format!(
      "{}{}",
      self.get_year_gan_exact(),
      self.get_year_zhi_exact()
    )
  }

  fn get_month_gan(&self) -> String {
    GAN()[self.lock().unwrap().__month_gan_index as usize + 1]
      .to_string()
  }

  fn get_month_gan_exact(&self) -> String {
    GAN()[self.lock().unwrap().__month_gan_index_exact as usize + 1]
      .to_string()
  }

  fn get_month_zhi(&self) -> String {
    ZHI()[self.lock().unwrap().__month_zhi_index as usize + 1]
      .to_string()
  }

  fn get_month_zhi_exact(&self) -> String {
    ZHI()[self.lock().unwrap().__month_zhi_index_exact as usize + 1]
      .to_string()
  }

  fn get_month_in_gan_zhi(&self) -> String {
    format!("{}{}", self.get_month_gan(), self.get_month_zhi())
  }

  fn get_month_in_gan_zhi_exact(&self) -> String {
    format!(
      "{}{}",
      self.get_month_gan_exact(),
      self.get_month_zhi_exact()
    )
  }

  fn get_day_gan(&self) -> String {
    GAN()[self.lock().unwrap().__day_gan_index as usize + 1].to_string()
  }

  fn get_day_gan_exact(&self) -> String {
    GAN()[self.lock().unwrap().__day_gan_index_exact as usize + 1]
      .to_string()
  }

  fn get_day_gan_exact2(&self) -> String {
    GAN()[self.lock().unwrap().__day_gan_index_exact2 as usize + 1]
      .to_string()
  }

  fn get_day_zhi(&self) -> String {
    ZHI()[self.lock().unwrap().__day_zhi_index as usize + 1].to_string()
  }

  fn get_day_zhi_exact(&self) -> String {
    ZHI()[self.lock().unwrap().__day_zhi_index_exact as usize + 1]
      .to_string()
  }

  fn get_day_zhi_exact2(&self) -> String {
    ZHI()[self.lock().unwrap().__day_zhi_index_exact2 as usize + 1]
      .to_string()
  }

  fn get_day_in_gan_zhi(&self) -> String {
    format!("{}{}", self.get_day_gan(), self.get_day_zhi())
  }

  fn get_day_in_gan_zhi_exact(&self) -> String {
    format!("{}{}", self.get_day_gan_exact(), self.get_day_zhi_exact())
  }

  fn get_day_in_gan_zhi_exact2(&self) -> String {
    format!(
      "{}{}",
      self.get_day_gan_exact2(),
      self.get_day_zhi_exact2()
    )
  }

  fn get_time_gan(&self) -> String {
    GAN()[self.lock().unwrap().__time_gan_index as usize + 1]
      .to_string()
  }

  fn get_time_zhi(&self) -> String {
    ZHI()[self.lock().unwrap().__time_zhi_index as usize + 1]
      .to_string()
  }

  fn get_time_in_gan_zhi(&self) -> String {
    format!("{}{}", self.get_time_gan(), self.get_time_zhi())
  }

  fn get_year_sheng_xiao(&self) -> String {
    SHENGXIAO()[self.lock().unwrap().__year_zhi_index as usize + 1]
      .to_string()
  }

  fn get_year_sheng_xiao_by_li_chun(&self) -> String {
    SHENGXIAO()
      [self.lock().unwrap().__year_zhi_index_by_li_chun as usize + 1]
      .to_string()
  }

  fn get_year_sheng_xiao_exact(&self) -> String {
    SHENGXIAO()
      [self.lock().unwrap().__year_zhi_index_exact as usize + 1]
      .to_string()
  }

  fn get_month_sheng_xiao(&self) -> String {
    SHENGXIAO()[self.lock().unwrap().__month_zhi_index as usize + 1]
      .to_string()
  }

  fn get_month_sheng_xiao_exact(&self) -> String {
    SHENGXIAO()
      [self.lock().unwrap().__month_zhi_index_exact as usize + 1]
      .to_string()
  }

  fn get_day_sheng_xiao(&self) -> String {
    SHENGXIAO()[self.lock().unwrap().__day_zhi_index as usize + 1]
      .to_string()
  }

  fn get_time_sheng_xiao(&self) -> String {
    SHENGXIAO()[self.lock().unwrap().__time_zhi_index as usize + 1]
      .to_string()
  }

  fn get_year_in_chinese(&self) -> String {
    let y = format!("{}", self.lock().unwrap().__year);
    let mut s = "".to_string();
    for c in y.chars().collect::<Vec<_>>().iter() {
      s = format!(
        "{}{}",
        s,
        NUMBER()[c.to_string().parse::<usize>().unwrap()]
      );
    }
    s
  }

  fn get_month_in_chinese(&self) -> String {
    let month = self.lock().unwrap().__month;
    format!(
      "{}{}",
      {
        match month < 0 {
          true => "闰",
          _ => "",
        }
      },
      MONTH()[month.abs() as usize]
    )
  }

  fn get_day_in_chinese(&self) -> String {
    DAY()[self.lock().unwrap().__day as usize].to_string()
  }

  fn get_peng_zu_gan(&self) -> String {
    PENG_ZU_GAN()[self.lock().unwrap().__day_gan_index as usize + 1]
      .to_string()
  }

  fn get_peng_zu_zhi(&self) -> String {
    PENG_ZU_ZHI()[self.lock().unwrap().__day_zhi_index as usize + 1]
      .to_string()
  }

  fn get_position_xi(&self) -> String {
    self.get_day_position_xi()
  }

  fn get_position_xi_desc(&self) -> String {
    self.get_day_position_xi_desc()
  }

  fn get_position_yang_gui(&self) -> String {
    self.get_day_position_yang_gui()
  }

  fn get_position_yang_gui_desc(&self) -> String {
    self.get_day_position_yang_gui_desc()
  }

  fn get_position_yin_gui(&self) -> String {
    self.get_day_position_yin_gui()
  }

  fn get_position_yin_gui_desc(&self) -> String {
    self.get_day_position_yin_gui_desc()
  }

  fn get_position_fu(&self) -> String {
    self.get_day_position_fu(None)
  }

  fn get_position_fu_desc(&self) -> String {
    self.get_day_position_fu_desc(None)
  }

  fn get_position_cai(&self) -> String {
    self.get_day_position_cai()
  }

  fn get_position_cai_desc(&self) -> String {
    self.get_day_position_cai_desc()
  }

  fn get_day_position_xi(&self) -> String {
    POSITION_XI()[self.lock().unwrap().__day_gan_index as usize + 1]
      .to_string()
  }

  fn get_day_position_xi_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_day_position_xi())
      .unwrap()
      .clone()
  }

  fn get_day_position_yang_gui(&self) -> String {
    POSITION_YANG_GUI()
      [self.lock().unwrap().__day_gan_index as usize + 1]
      .to_string()
  }

  fn get_day_position_yang_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_day_position_yang_gui())
      .unwrap()
      .clone()
  }

  fn get_day_position_yin_gui(&self) -> String {
    POSITION_YIN_GUI()
      [self.lock().unwrap().__day_gan_index as usize + 1]
      .to_string()
  }

  fn get_day_position_yin_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_day_position_yin_gui())
      .unwrap()
      .clone()
  }

  fn get_day_position_fu(&self, sect: Option<i64>) -> String {
    let sect = sect.unwrap_or(2);
    let position_fu = match sect == 1 {
      true => POSITION_FU.clone(),
      _ => POSITION_FU_2.clone(),
    };
    position_fu()[self.lock().unwrap().__day_gan_index as usize + 1]
      .to_string()
  }

  fn get_day_position_fu_desc(&self, sect: Option<i64>) -> String {
    POSITION_DESC()
      .get(&self.get_day_position_fu(sect))
      .unwrap()
      .clone()
  }

  fn get_day_position_cai(&self) -> String {
    POSITION_CAI()[self.lock().unwrap().__day_gan_index as usize + 1]
      .to_string()
  }

  fn get_day_position_cai_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_day_position_cai())
      .unwrap()
      .clone()
  }

  fn get_year_position_tai_sui(&self, sect: Option<i64>) -> String {
    let sect = sect.unwrap_or(2);
    let year_zhi_index = match sect {
      1 => self.lock().unwrap().__year_zhi_index,
      3 => self.lock().unwrap().__year_zhi_index_exact,
      _ => self.lock().unwrap().__year_zhi_index_by_li_chun,
    };
    POSITION_TAI_SUI_YEAR()[year_zhi_index as usize].to_string()
  }

  fn get_year_position_tai_sui_desc(
    &self,
    sect: Option<i64>,
  ) -> String {
    POSITION_DESC()
      .get(&self.get_year_position_tai_sui(sect))
      .unwrap()
      .clone()
  }

  fn get_month_position_tai_sui(&self, sect: Option<i64>) -> String {
    let sect = sect.unwrap_or(2);
    let s = self.lock().unwrap();
    let (month_zhi_index, month_gan_index) = match sect {
      3 => (s.__month_zhi_index_exact, s.__month_gan_index_exact),
      _ => (s.__month_zhi_index, s.__month_gan_index),
    };
    Lunar::__get_month_position_tai_sui(
      month_zhi_index,
      month_gan_index,
    )
  }

  fn get_month_position_tai_sui_desc(
    &self,
    sect: Option<i64>,
  ) -> String {
    POSITION_DESC()
      .get(&self.get_month_position_tai_sui(sect))
      .unwrap()
      .clone()
  }

  fn get_day_position_tai_sui(&self, sect: Option<i64>) -> String {
    let (
      __year_zhi_index,
      __year_zhi_index_exact,
      __year_zhi_index_by_li_chun,
    ) = {
      let s = self.lock().unwrap();
      (
        s.__year_zhi_index,
        s.__year_zhi_index_exact,
        s.__year_zhi_index_by_li_chun,
      )
    };
    let pair = match sect {
      Some(1) => (self.get_day_in_gan_zhi(), __year_zhi_index),
      Some(3) => (self.get_day_in_gan_zhi(), __year_zhi_index_exact),
      _ => (
        self.get_day_in_gan_zhi_exact2(),
        __year_zhi_index_by_li_chun,
      ),
    };
    Lunar::__get_day_position_tai_sui(&pair.0, pair.1)
  }

  fn get_day_position_tai_sui_desc(&self, sect: Option<i64>) -> String {
    POSITION_DESC()
      .get(&self.get_day_position_tai_sui(sect))
      .unwrap()
      .clone()
  }

  fn get_time_position_xi(&self) -> String {
    POSITION_XI()[self.lock().unwrap().__time_gan_index as usize + 1]
      .to_string()
  }

  fn get_time_position_xi_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_time_position_xi())
      .unwrap()
      .clone()
  }

  fn get_time_position_yang_gui(&self) -> String {
    POSITION_YANG_GUI()
      [self.lock().unwrap().__time_gan_index as usize + 1]
      .to_string()
  }

  fn get_time_position_yang_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_time_position_yang_gui())
      .unwrap()
      .clone()
  }

  fn get_time_position_yin_gui(&self) -> String {
    POSITION_YIN_GUI()
      [self.lock().unwrap().__time_gan_index as usize + 1]
      .to_string()
  }

  fn get_time_position_yin_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_time_position_yin_gui())
      .unwrap()
      .clone()
  }

  fn get_time_position_fu(&self, sect: Option<i64>) -> String {
    let position_fu = match sect {
      Some(1) => POSITION_FU.clone(),
      _ => POSITION_FU_2.clone(),
    };
    position_fu()[self.lock().unwrap().__time_gan_index as usize + 1]
      .to_string()
  }

  fn get_time_position_fu_desc(&self, sect: Option<i64>) -> String {
    POSITION_DESC()
      .get(&self.get_time_position_fu(sect))
      .unwrap()
      .clone()
  }

  fn get_time_position_cai(&self) -> String {
    POSITION_CAI()[self.lock().unwrap().__time_gan_index as usize + 1]
      .to_string()
  }

  fn get_time_position_cai_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_time_position_cai())
      .unwrap()
      .clone()
  }

  fn get_chong(&self) -> String {
    self.get_day_chong()
  }

  fn get_day_chong(&self) -> String {
    CHONG()[self.lock().unwrap().__day_zhi_index as usize].to_string()
  }

  fn get_time_chong(&self) -> String {
    CHONG()[self.lock().unwrap().__time_zhi_index as usize].to_string()
  }

  fn get_chong_gan(&self) -> String {
    self.get_day_chong_gan()
  }

  fn get_day_chong_gan(&self) -> String {
    CHONG_GAN()[self.lock().unwrap().__day_gan_index as usize]
      .to_string()
  }

  fn get_time_chong_gan(&self) -> String {
    CHONG_GAN()[self.lock().unwrap().__time_gan_index as usize]
      .to_string()
  }

  fn get_chong_gan_tie(&self) -> String {
    self.get_day_chong_gan_tie()
  }

  fn get_day_chong_gan_tie(&self) -> String {
    CHONG_GAN_TIE()[self.lock().unwrap().__day_gan_index as usize]
      .to_string()
  }

  fn get_time_chong_gan_tie(&self) -> String {
    CHONG_GAN_TIE()[self.lock().unwrap().__time_gan_index as usize]
      .to_string()
  }

  fn get_chong_sheng_xiao(&self) -> String {
    self.get_day_chong_sheng_xiao()
  }

  fn get_day_chong_sheng_xiao(&self) -> String {
    let chong = self.get_day_chong();
    for (i, zhi) in ZHI().iter().enumerate() {
      if *zhi == chong {
        return SHENGXIAO()[i].to_string();
      }
    }
    format!("")
  }

  fn get_time_chong_sheng_xiao(&self) -> String {
    let chong = self.get_time_chong();
    for (i, zhi) in ZHI().iter().enumerate() {
      if *zhi == chong {
        return SHENGXIAO()[i].to_string();
      }
    }
    format!("")
  }

  fn get_chong_desc(&self) -> String {
    self.get_day_chong_desc()
  }

  fn get_day_chong_desc(&self) -> String {
    format!(
      "({}{}){}",
      self.get_day_chong_gan(),
      self.get_day_chong(),
      self.get_day_chong_sheng_xiao()
    )
  }

  fn get_time_chong_desc(&self) -> String {
    format!(
      "({}{}){}",
      self.get_time_chong_gan(),
      self.get_time_chong(),
      self.get_time_chong_sheng_xiao()
    )
  }

  fn get_sha(&self) -> String {
    self.get_day_sha()
  }

  fn get_day_sha(&self) -> String {
    SHA().get(&self.get_day_zhi()).unwrap().clone()
  }

  fn get_time_sha(&self) -> String {
    SHA().get(&self.get_time_zhi()).unwrap().clone()
  }

  fn get_year_na_yin(&self) -> String {
    NAYIN().get(&self.get_year_in_gan_zhi()).unwrap().clone()
  }

  fn get_month_na_yin(&self) -> String {
    NAYIN().get(&self.get_month_in_gan_zhi()).unwrap().clone()
  }

  fn get_day_na_yin(&self) -> String {
    NAYIN().get(&self.get_day_in_gan_zhi()).unwrap().clone()
  }

  fn get_time_na_yin(&self) -> String {
    NAYIN().get(&self.get_time_in_gan_zhi()).unwrap().clone()
  }

  fn get_season(&self) -> String {
    SEASON()[self.lock().unwrap().__month as usize].to_string()
  }

  fn get_jie(&self) -> String {
    // odd items of JIE_QI_IN_USE
    for (_, key) in JIE_QI_IN_USE().iter().enumerate().step_by(2) {
      let (__jie_qi, __solar) = {
        let s = self.lock().unwrap();
        (s.__jie_qi.clone(), s.__solar.clone())
      };
      let d = __jie_qi.get(key).unwrap();
      if d.get_year() == __solar.get_year()
        && d.get_month() == __solar.get_month()
        && d.get_day() == __solar.get_day()
      {
        return Lunar::__convert_jie_qi(key);
      }
    }
    format!("")
  }

  fn get_qi(&self) -> String {
    // even items of JIE_QI_IN_USE
    let (__jie_qi, __solar) = {
      let s = self.lock().unwrap();
      (s.__jie_qi.clone(), s.__solar.clone())
    };
    for key in JIE_QI_IN_USE().iter().skip(1).step_by(2) {
      let d = __jie_qi.get(key).unwrap();
      if d.get_year() == __solar.get_year()
        && d.get_month() == __solar.get_month()
        && d.get_day() == __solar.get_day()
      {
        return Lunar::__convert_jie_qi(key);
      }
    }
    format!("")
  }

  fn get_week(&self) -> i64 {
    self.lock().unwrap().__week_index
  }

  fn get_week_in_chinese(&self) -> String {
    solar_util::get().WEEK()[self.get_week() as usize].to_string()
  }

  fn get_xiu(&self) -> String {
    XIU()
      .get(&format!("{}{}", self.get_day_zhi(), self.get_week()))
      .unwrap()
      .clone()
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

  fn get_festivals(&self) -> Vec<String> {
    let (__year, __month, __day) = {
      let s = self.lock().unwrap();
      (s.__year, s.__month, s.__day)
    };

    let mut fs = vec![];
    let found = FESTIVAL().iter().find_map(|_fs| {
      let (f_month, f_day) = &_fs.0;
      if *f_month == __month && *f_day == __day {
        return Some(_fs.1.clone());
      }
      None
    });
    let _ = found.map_or((), |f| fs.push(f));
    if __month.abs() == 12
      && __day >= 29
      && __year != self.next(1).get_year()
    {
      fs.push(format!("除夕"));
    }
    fs
  }

  fn get_other_festivals(&self) -> Vec<String> {
    let (__month, __day, __solar, __jie_qi) = {
      let s = self.lock().unwrap();
      (s.__month, s.__day, s.__solar.clone(), s.__jie_qi.clone())
    };

    let mut fs: Vec<String> = vec![];
    let found = OTHER_FESTIVAL().iter().find_map(|(k, v)| {
      let (f_month, f_day) = k;
      if *f_month == __month && *f_day == __day {
        return Some(v.clone());
      }
      None
    });
    let _ = found.map_or((), |f| {
      let mut f = f.clone();
      fs.append(&mut f);
    });
    let solar_ymd = __solar.to_ymd();
    if __jie_qi.get("清明").unwrap().next(-1, None).to_ymd()
      == solar_ymd
    {
      fs.push(format!("寒食节"));
    }

    let jq = __jie_qi.get("立春").unwrap().clone();
    let mut offset = 4 - jq.get_lunar().get_day_gan_index();
    if offset < 0 {
      offset = offset + 10;
    }
    if solar_ymd == jq.next(offset + 40, None).to_ymd() {
      fs.push(format!("春社"));
    }

    let jq = __jie_qi.get("立秋").unwrap().clone();
    let mut offset = 4 - jq.get_lunar().get_day_gan_index();
    if offset < 0 {
      offset = offset + 10;
    }
    if solar_ymd == jq.next(offset + 40, None).to_ymd() {
      fs.push(format!("秋社"));
    }

    fs
  }

  fn get_eight_char(&self) -> EightCharRef {
    let mut s = self.lock().unwrap();
    if s.__eight_char.is_none() {
      (*s).__eight_char = Some(EightChar::from_lunar(self.clone()));
    }
    s.__eight_char.as_ref().unwrap().clone()
  }

  fn get_ba_zi(&self) -> Vec<String> {
    let ba_zi = self.get_eight_char();
    [
      ba_zi.get_year(),
      ba_zi.get_month(),
      ba_zi.get_day(),
      ba_zi.get_time(),
    ]
    .to_vec()
  }

  fn get_ba_zi_wu_xing(&self) -> Vec<String> {
    let ba_zi = self.get_eight_char();
    [
      ba_zi.get_year_wu_xing(),
      ba_zi.get_month_wu_xing(),
      ba_zi.get_day_wu_xing(),
      ba_zi.get_time_wu_xing(),
    ]
    .to_vec()
  }

  fn get_ba_zi_na_yin(&self) -> Vec<String> {
    let ba_zi = self.get_eight_char();
    [
      ba_zi.get_year_na_yin(),
      ba_zi.get_month_na_yin(),
      ba_zi.get_day_na_yin(),
      ba_zi.get_time_na_yin(),
    ]
    .to_vec()
  }

  fn get_ba_zi_shi_shen_gan(&self) -> Vec<String> {
    let ba_zi = self.get_eight_char();
    [
      ba_zi.get_year_shi_shen_gan(),
      ba_zi.get_month_shi_shen_gan(),
      ba_zi.get_day_shi_shen_gan(),
      ba_zi.get_time_shi_shen_gan(),
    ]
    .to_vec()
  }

  fn get_ba_zi_shi_shen_zhi(&self) -> Vec<Vec<String>> {
    let ba_zi = self.get_eight_char();
    [
      ba_zi.get_year_shi_shen_zhi(),
      ba_zi.get_month_shi_shen_zhi(),
      ba_zi.get_day_shi_shen_zhi(),
      ba_zi.get_time_shi_shen_zhi(),
    ]
    .to_vec()
  }

  fn get_ba_zi_shi_shen_year_zhi(&self) -> Vec<String> {
    self.get_eight_char().get_year_shi_shen_zhi()
  }

  fn get_ba_zi_shi_shen_month_zhi(&self) -> Vec<String> {
    self.get_eight_char().get_month_shi_shen_zhi()
  }

  fn get_ba_zi_shi_shen_day_zhi(&self) -> Vec<String> {
    self.get_eight_char().get_day_shi_shen_zhi()
  }

  fn get_ba_zi_shi_shen_time_zhi(&self) -> Vec<String> {
    self.get_eight_char().get_time_shi_shen_zhi()
  }

  fn get_zhi_xing(&self) -> String {
    let s = self.lock().unwrap();
    let mut offset = s.__day_zhi_index - s.__month_zhi_index;
    if offset < 0 {
      offset += 12
    }
    ZHI_XING()[offset as usize + 1].to_string()
  }

  fn get_day_tian_shen(&self) -> String {
    let __day_zhi_index = { self.lock().unwrap().__day_zhi_index };
    TIAN_SHEN()[(__day_zhi_index
      + ZHI_TIAN_SHEN_OFFSET().get(&self.get_month_zhi()).unwrap())
      as usize
      % 12
      + 1]
      .to_string()
  }

  fn get_time_tian_shen(&self) -> String {
    let __time_zhi_index = { self.lock().unwrap().__time_zhi_index };
    TIAN_SHEN()[(__time_zhi_index
      + ZHI_TIAN_SHEN_OFFSET()
        .get(&self.get_day_zhi_exact())
        .unwrap()) as usize
      % 12
      + 1]
      .to_string()
  }

  fn get_day_tian_shen_type(&self) -> String {
    TIAN_SHEN_TYPE()
      .get(&self.get_day_tian_shen())
      .unwrap()
      .clone()
  }

  fn get_time_tian_shen_type(&self) -> String {
    TIAN_SHEN_TYPE()
      .get(&self.get_time_tian_shen())
      .unwrap()
      .clone()
  }

  fn get_day_tian_shen_luck(&self) -> String {
    TIAN_SHEN_TYPE_LUCK()
      .get(&self.get_day_tian_shen_type())
      .unwrap()
      .clone()
  }

  fn get_time_tian_shen_luck(&self) -> String {
    TIAN_SHEN_TYPE_LUCK()
      .get(&self.get_time_tian_shen_type())
      .unwrap()
      .clone()
  }

  fn get_day_position_tai(&self) -> String {
    POSITION_TAI_DAY()
      [LunarUtil::get_jia_zi_index(&self.get_day_in_gan_zhi()) as usize]
      .to_string()
  }

  fn get_month_position_tai(&self) -> String {
    let m = self.lock().unwrap().__month;
    if m < 0 {
      return format!("");
    }
    POSITION_TAI_MONTH()[m as usize - 1].to_string()
  }

  ///
  /// 获取每日宜
  ///
  /// ## Returns
  /// 宜: Vec<String>
  ///
  fn get_day_yi(&self, sect: Option<i64>) -> Vec<String> {
    let month_gan_zhi = match sect {
      Some(2) => self.get_month_in_gan_zhi_exact(),
      _ => self.get_month_in_gan_zhi(),
    };
    LunarUtil::get_day_yi(&month_gan_zhi, &self.get_day_in_gan_zhi())
  }

  ///
  /// 获取每日忌
  ///
  /// ## Returns
  /// 忌: Vec<String>
  ///
  fn get_day_ji(&self, sect: Option<i64>) -> Vec<String> {
    let month_gan_zhi = match sect {
      Some(2) => self.get_month_in_gan_zhi_exact(),
      _ => self.get_month_in_gan_zhi(),
    };
    LunarUtil::get_day_ji(&month_gan_zhi, &self.get_day_in_gan_zhi())
  }

  ///
  /// 获取时宜
  ///
  /// ## Returns
  /// 宜: Vec<String>
  ///
  fn get_time_yi(&self) -> Vec<String> {
    LunarUtil::get_time_yi(
      &self.get_day_in_gan_zhi_exact(),
      &self.get_time_in_gan_zhi(),
    )
  }

  ///
  /// 获取时忌
  ///
  /// ## Returns
  /// 忌: Vec<String>
  ///
  fn get_time_ji(&self) -> Vec<String> {
    LunarUtil::get_time_ji(
      &self.get_day_in_gan_zhi_exact(),
      &self.get_time_in_gan_zhi(),
    )
  }

  ///
  /// 获取日吉神（宜趋）
  ///
  /// ## Returns
  /// 日吉神: Vec<String>
  ///
  fn get_day_ji_shen(&self) -> Vec<String> {
    LunarUtil::get_day_ji_shen(
      self.get_month(),
      &self.get_day_in_gan_zhi(),
    )
  }

  ///
  /// 获取日凶煞（宜忌）
  ///
  /// ## Returns
  /// 日凶煞: Vec<String>
  ///
  fn get_day_xiong_sha(&self) -> Vec<String> {
    LunarUtil::get_day_xiong_sha(
      self.get_month(),
      &self.get_day_in_gan_zhi(),
    )
  }

  ///
  /// 获取月相
  ///
  /// ## Returns
  /// 月相: String
  ///
  fn get_yue_xiang(&self) -> String {
    YUE_XIANG()[self.lock().unwrap().__day as usize].to_string()
  }

  fn get_year_nine_star(&self, sect: Option<i64>) -> NineStarRef {
    let year_in_gan_zhi = match sect {
      Some(1) => self.get_year_in_gan_zhi(),
      Some(3) => self.get_year_in_gan_zhi_exact(),
      _ => self.get_year_in_gan_zhi_by_li_chun(),
    };
    self.__get_year_nine_star(&year_in_gan_zhi)
  }

  fn get_month_nine_star(&self, sect: Option<i64>) -> NineStarRef {
    let (
      __year_zhi_index,
      __month_zhi_index,
      __year_zhi_index_exact,
      __month_zhi_index_exact,
      __year_zhi_index_by_li_chun,
    ) = {
      let s = self.lock().unwrap();
      (
        s.__year_zhi_index,
        s.__month_zhi_index,
        s.__year_zhi_index_exact,
        s.__month_zhi_index_exact,
        s.__year_zhi_index_by_li_chun,
      )
    };
    let (year_zhi_index, month_zhi_index) = match sect {
      Some(1) => (__year_zhi_index, __month_zhi_index),
      Some(3) => (__year_zhi_index_exact, __month_zhi_index_exact),
      _ => (__year_zhi_index_by_li_chun, __month_zhi_index),
    };
    Lunar::__get_month_nine_star(year_zhi_index, month_zhi_index)
  }

  fn get_day_nine_star(&self) -> NineStarRef {
    let solar_date = self.get_solar().to_naivedate();
    let jie_qi = { self.lock().unwrap().__jie_qi.clone() };
    let dong_zhi = jie_qi.get("冬至").unwrap().clone();
    let dong_zhi2 = jie_qi.get("DONG_ZHI").unwrap().clone();
    let xia_zhi = jie_qi.get("夏至").unwrap().clone();
    let dong_zhi_index = LunarUtil::get_jia_zi_index(
      &dong_zhi.get_lunar().get_day_in_gan_zhi(),
    );
    let dong_zhi_index2 = LunarUtil::get_jia_zi_index(
      &dong_zhi2.get_lunar().get_day_in_gan_zhi(),
    );
    let xia_zhi_index = LunarUtil::get_jia_zi_index(
      &xia_zhi.get_lunar().get_day_in_gan_zhi(),
    );

    let solar_shun_bai = match dong_zhi_index > 29 {
      true => dong_zhi.next(60 - dong_zhi_index, None),
      _ => dong_zhi.next(-dong_zhi_index, None),
    };
    let solar_shun_bai_date = solar_shun_bai.to_naivedate();

    let solar_shun_bai2 = match dong_zhi_index2 > 29 {
      true => dong_zhi2.next(60 - dong_zhi_index2, None),
      _ => dong_zhi2.next(-dong_zhi_index2, None),
    };
    let solar_shun_bai_date2 = solar_shun_bai2.to_naivedate();

    let solar_ni_zi = match xia_zhi_index > 29 {
      true => xia_zhi.next(60 - xia_zhi_index, None),
      _ => xia_zhi.next(-xia_zhi_index, None),
    };
    let solar_ni_zi_date = solar_ni_zi.to_naivedate();

    let offset = match solar_date {
      d if solar_shun_bai_date <= d && d < solar_ni_zi_date => {
        self.get_solar().subtract(solar_shun_bai) % 9
      }
      d if solar_ni_zi_date <= d && d < solar_shun_bai_date2 => {
        8 - (self.get_solar().subtract(solar_ni_zi) % 9)
      }
      d if d >= solar_shun_bai_date2 => {
        self.get_solar().subtract(solar_shun_bai2) % 9
      }
      d if d < solar_shun_bai_date => {
        (8 + solar_shun_bai.subtract(self.get_solar())) % 9
      }
      _ => 0,
    };
    NineStar::from_index(offset as usize)
  }

  fn get_time_nine_star(&self) -> NineStarRef {
    let solar_date = self.get_solar().to_naivedate();
    let (jie_qi, time_zhi_index) = {
      let s = self.lock().unwrap();
      (s.__jie_qi.clone(), s.__time_zhi_index)
    };

    let asc = if jie_qi.get("冬至").unwrap().to_naivedate()
      <= solar_date
      && solar_date < jie_qi.get("夏至").unwrap().to_naivedate()
    {
      true
    } else if solar_date
      >= jie_qi.get("DONG_ZHI").unwrap().to_naivedate()
    {
      true
    } else {
      false
    };

    let day_zhi = self.get_day_zhi().chars().collect::<Vec<_>>();
    let start = if "子午卯酉"
      .chars()
      .collect::<Vec<_>>()
      .iter()
      .find(|c| day_zhi.iter().find(|z| **z == **c).is_some())
      .is_some()
    {
      match asc {
        true => 0,
        _ => 8,
      }
    } else if "辰戌丑未"
      .chars()
      .collect::<Vec<_>>()
      .iter()
      .find(|c| day_zhi.iter().find(|z| **z == **c).is_some())
      .is_some()
    {
      match asc {
        true => 3,
        _ => 5,
      }
    } else {
      match asc {
        true => 6,
        _ => 2,
      }
    };

    let index = match asc {
      true => start + time_zhi_index,
      _ => start + 9 - time_zhi_index,
    };

    return NineStar::from_index(index as usize % 9);
  }

  fn get_jie_qi_table(&self) -> HashMap<String, SolarRef> {
    self.lock().unwrap().__jie_qi.clone()
  }

  fn get_jie_qi_list(&self) -> Vec<String> {
    self.lock().unwrap().__jie_qi_list.clone()
  }

  fn get_time_gan_index(&self) -> i64 {
    self.lock().unwrap().__time_gan_index
  }

  fn get_time_zhi_index(&self) -> i64 {
    self.lock().unwrap().__time_zhi_index
  }

  fn get_day_gan_index(&self) -> i64 {
    self.lock().unwrap().__day_gan_index
  }

  fn get_day_zhi_index(&self) -> i64 {
    self.lock().unwrap().__day_zhi_index
  }

  fn get_day_gan_index_exact(&self) -> i64 {
    self.lock().unwrap().__day_gan_index_exact
  }

  fn get_day_gan_index_exact2(&self) -> i64 {
    self.lock().unwrap().__day_gan_index_exact2
  }

  fn get_day_zhi_index_exact(&self) -> i64 {
    self.lock().unwrap().__day_zhi_index_exact
  }

  fn get_day_zhi_index_exact2(&self) -> i64 {
    self.lock().unwrap().__day_zhi_index_exact2
  }

  fn get_month_gan_index(&self) -> i64 {
    self.lock().unwrap().__month_gan_index
  }

  fn get_month_zhi_index(&self) -> i64 {
    self.lock().unwrap().__month_zhi_index
  }

  fn get_month_gan_index_exact(&self) -> i64 {
    self.lock().unwrap().__month_gan_index_exact
  }

  fn get_month_zhi_index_exact(&self) -> i64 {
    self.lock().unwrap().__month_zhi_index_exact
  }

  fn get_year_gan_index(&self) -> i64 {
    self.lock().unwrap().__year_gan_index
  }

  fn get_year_zhi_index(&self) -> i64 {
    self.lock().unwrap().__year_zhi_index
  }

  fn get_year_gan_index_by_li_chun(&self) -> i64 {
    self.lock().unwrap().__year_gan_index_by_li_chun
  }

  fn get_year_zhi_index_by_li_chun(&self) -> i64 {
    self.lock().unwrap().__year_zhi_index_by_li_chun
  }

  fn get_year_gan_index_exact(&self) -> i64 {
    self.lock().unwrap().__year_gan_index_exact
  }

  fn get_year_zhi_index_exact(&self) -> i64 {
    self.lock().unwrap().__year_zhi_index_exact
  }

  ///
  /// 获取下一节（顺推的第一个节）
  ///
  /// ## Parameters
  /// whole_day: Option<bool> - 是否按天计 **default(false)**
  ///
  /// ## Returns
  /// 节气: String
  ///
  fn get_next_jie(&self, whole_day: Option<bool>) -> Option<JieQiRef> {
    let whole_day = whole_day.unwrap_or(false);
    let mut conditions = vec![];

    for i in 0..(JIE_QI_IN_USE().len() / 2) {
      conditions.push(JIE_QI_IN_USE()[i * 2].to_string())
    }

    self.__get_near_jie_qi(true, &conditions, whole_day)
  }

  ///
  /// 获取上一节（逆推的第一个节）
  ///
  /// ## Arguments
  /// whole_day: Option<bool> - 是否按天计 **default(false)**
  ///
  /// ## Returns
  /// 节气: String
  ///
  fn get_prev_jie(&self, whole_day: Option<bool>) -> Option<JieQiRef> {
    let whole_day = whole_day.map_or(false, |f| f);
    let mut conditions = vec![];

    JIE_QI_IN_USE()
      .iter()
      .enumerate()
      .step_by(2)
      .for_each(|(_i, jq)| conditions.push(jq.to_string()));

    self.__get_near_jie_qi(false, &conditions, whole_day)
  }

  ///
  /// 获取下一气令（顺推的第一个气令）
  ///
  /// ## Arguments
  /// whole_day: **Option\<bool\>** - 是否按天计 **default=false**
  ///
  /// ## Returns
  /// 节气: **String**
  ///
  fn get_next_qi(&self, whole_day: Option<bool>) -> Option<JieQiRef> {
    let whole_day = whole_day.map_or(false, |f| f);
    let mut conditions = vec![];

    JIE_QI_IN_USE()
      .iter()
      .enumerate()
      .skip(1)
      .step_by(2)
      .for_each(|(_i, jq)| conditions.push(jq.to_string()));
    self.__get_near_jie_qi(true, &conditions, whole_day)
  }

  ///
  /// 获取上一气令（逆推的第一个气令）
  ///
  /// ## Arguments
  /// whole_day: **Option\<bool\>** - 是否按天计 **default(false)**
  ///
  /// ## Returns
  /// 节气: String
  ///
  fn get_prev_qi(&self, whole_day: Option<bool>) -> Option<JieQiRef> {
    let whole_day = whole_day.map_or(false, |f| f);
    let mut conditions = vec![];

    JIE_QI_IN_USE()
      .iter()
      .enumerate()
      .skip(1)
      .step_by(2)
      .for_each(|(_i, jq)| conditions.push(jq.to_string()));
    self.__get_near_jie_qi(false, &conditions, whole_day)
  }

  ///
  /// 获取下一节气（顺推的第一个节气）
  ///
  /// ## Arguments
  /// whole_day: **Option\<bool\>** - 是否按天计 **default(false)**
  ///
  /// ## Returns
  /// 节气: String
  ///
  fn get_next_jie_qi(
    &self,
    whole_day: Option<bool>,
  ) -> Option<JieQiRef> {
    let whole_day = whole_day.map_or(false, |f| f);
    self.__get_near_jie_qi(true, &vec![], whole_day)
  }

  ///
  /// 获取上一节气（逆推的第一个节气）
  ///
  /// ## Arguments
  /// whole_day: **Option\<bool\>** - 是否按天计 **default(false)**
  ///
  /// ## Returns
  /// 节气: String
  ///
  fn get_prev_jie_qi(
    &self,
    whole_day: Option<bool>,
  ) -> Option<JieQiRef> {
    let whole_day = whole_day.unwrap_or(false);
    self.__get_near_jie_qi(false, &vec![], whole_day)
  }

  ///
  /// 获取节气名称，如果无节气，返回空字符串
  ///
  /// ## Returns
  /// 节气名称: **String**
  ///
  fn get_jie_qi(&self) -> String {
    let (__jie_qi, year, month, day) = {
      let s = self.lock().unwrap();
      (
        s.__jie_qi.clone(),
        s.__solar.get_year(),
        s.__solar.get_month(),
        s.__solar.get_day(),
      )
    };

    for (name, solar) in __jie_qi.iter() {
      if solar.get_year() == year
        && solar.get_month() == month
        && solar.get_day() == day
      {
        return Lunar::__convert_jie_qi(name.as_str());
      }
    }
    format!("")
  }

  ///
  /// 获取当天节气对象，如果无节气，返回None
  ///
  /// ## Returns
  /// 节气对象: **Option<JieQiRef>**
  ///
  fn get_current_jie_qi(&self) -> Option<JieQiRef> {
    let (__jie_qi, year, month, day) = {
      let s = self.lock().unwrap();
      (
        s.__jie_qi.clone(),
        s.__solar.get_year(),
        s.__solar.get_month(),
        s.__solar.get_day(),
      )
    };

    for (name, solar) in __jie_qi.iter() {
      if solar.get_year() == year
        && solar.get_month() == month
        && solar.get_day() == day
      {
        return Some(JieQi::new(
          &Lunar::__convert_jie_qi(name.as_str()),
          solar,
        ));
      }
    }
    None
  }

  ///
  /// 获取当天节令对象，如果无节令，返回None
  ///
  /// ## Return
  /// 节气对象: **Option<JieQiRef>**
  ///
  fn get_current_jie(&self) -> Option<JieQiRef> {
    let (__jie_qi, year, month, day) = {
      let s = self.lock().unwrap();
      (
        s.__jie_qi.clone(),
        s.__solar.get_year(),
        s.__solar.get_month(),
        s.__solar.get_day(),
      )
    };

    for key in JIE_QI_IN_USE().iter().step_by(2) {
      let d = __jie_qi.get(&key.to_string()).unwrap();
      if d.get_year() == year
        && d.get_month() == month
        && d.get_day() == day
      {
        return Some(JieQi::new(&Lunar::__convert_jie_qi(key), d));
      }
    }
    None
  }

  ///
  /// 获取当天气令对象，如果无气令，返回None
  ///
  /// ## Returns
  /// 节气对象: **Option<JieQiRef>**
  ///
  fn get_current_qi(&self) -> Option<JieQiRef> {
    let (__jie_qi, year, month, day) = {
      let s = self.lock().unwrap();
      (
        s.__jie_qi.clone(),
        s.__solar.get_year(),
        s.__solar.get_month(),
        s.__solar.get_day(),
      )
    };

    for key in JIE_QI_IN_USE().iter().skip(1).step_by(2) {
      let d = __jie_qi.get(&key.to_string()).unwrap();
      if d.get_year() == year
        && d.get_month() == month
        && d.get_day() == day
      {
        return Some(JieQi::new(&Lunar::__convert_jie_qi(key), d));
      }
    }
    None
  }

  ///
  /// 获取往后推几天的农历日期，如果要往前推，则天数用负数
  ///
  /// ## Arguments
  /// days: **i64** - 天数
  ///
  /// ## Returns
  /// 农历日期: **LunarRef**
  ///
  fn next(&self, days: i64) -> LunarRef {
    self.lock().unwrap().__solar.next(days, None).get_lunar()
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
    s = format!(
      "{} {}({})年",
      s,
      self.get_year_in_gan_zhi(),
      self.get_year_sheng_xiao()
    );
    s = format!(
      "{} {}({})月",
      s,
      self.get_month_in_gan_zhi(),
      self.get_month_sheng_xiao()
    );
    s = format!(
      "{} {}({})日",
      s,
      self.get_day_in_gan_zhi(),
      self.get_day_sheng_xiao()
    );
    s = format!(
      "{} {}({})时",
      s,
      self.get_time_zhi(),
      self.get_time_sheng_xiao()
    );
    s = format!(
      "{} 纳音[{} {} {} {}]",
      s,
      self.get_year_na_yin(),
      self.get_month_na_yin(),
      self.get_day_na_yin(),
      self.get_time_na_yin()
    );
    s = format!("{} 星期{}", s, self.get_week_in_chinese());

    let fest = self
      .get_festivals()
      .iter()
      .map(|festival| format!("({})", festival))
      .collect::<Vec<_>>()
      .join(" ");
    if fest.len() > 0 {
      s = format!("{} {}", s, fest);
    }

    let other_fest = self
      .get_other_festivals()
      .iter()
      .map(|festival| format!("({})", festival))
      .collect::<Vec<_>>()
      .join(" ");
    if other_fest.len() > 0 {
      s = format!("{} {}", s, other_fest);
    }

    let jq = self.get_jie_qi();
    if jq.len() > 0 {
      s = format!("{} [{}]", s, jq);
    }

    s = format!("{} {}方{}", s, self.get_gong(), self.get_shou());
    s = format!(
      "{} 星宿[{}{}{}]({})",
      s,
      self.get_xiu(),
      self.get_zheng(),
      self.get_animal(),
      self.get_xiu_luck()
    );
    s = format!(
      "{} 彭祖百忌[{} {}]",
      s,
      self.get_peng_zu_gan(),
      self.get_peng_zu_zhi()
    );
    s = format!(
      "{} 喜神方位[{}]({})",
      s,
      self.get_day_position_xi(),
      self.get_day_position_xi_desc()
    );
    s = format!(
      "{} 阳贵神方位[{}]({})",
      s,
      self.get_day_position_yang_gui(),
      self.get_day_position_yang_gui_desc()
    );
    s = format!(
      "{} 阴贵神方位[{}]({})",
      s,
      self.get_day_position_yin_gui(),
      self.get_day_position_yin_gui_desc()
    );
    s = format!(
      "{} 福神方位[{}]({})",
      s,
      self.get_day_position_fu(None),
      self.get_day_position_fu_desc(None)
    );
    s = format!(
      "{} 财神方位[{}]({})",
      s,
      self.get_day_position_cai(),
      self.get_day_position_cai_desc()
    );
    s = format!("{} 冲[{}]", s, self.get_chong_desc());
    s = format!("{} 煞[{}]", s, self.get_sha());
    s
  }

  ///
  /// 获取年所在旬（以正月初一作为新年的开始）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_year_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_year_in_gan_zhi())
  }

  ///
  /// 获取年所在旬（以立春当天作为新年的开始）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_year_xun_by_li_chun(&self) -> String {
    LunarUtil::get_xun(&self.get_year_in_gan_zhi_by_li_chun())
  }

  ///
  /// 获取年所在旬（以立春交接时刻作为新年的开始）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_year_xun_exact(&self) -> String {
    LunarUtil::get_xun(&self.get_year_in_gan_zhi_exact())
  }

  ///
  /// 获取值年空亡（以正月初一作为新年的开始）
  ///
  /// ## Returns
  /// 空亡(旬空): **String**
  ///
  fn get_year_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_year_in_gan_zhi())
  }

  ///
  /// 获取值年空亡（以立春当天作为新年的开始）
  ///
  /// ## Returns
  /// 空亡(旬空): **String**
  ///
  fn get_year_xun_kong_by_li_chun(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_year_in_gan_zhi_by_li_chun())
  }

  ///
  /// 获取值年空亡（以立春交接时刻作为新年的开始）
  ///
  /// ## Returns
  /// 空亡(旬空): **String**
  ///
  fn get_year_xun_kong_exact(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_year_in_gan_zhi_exact())
  }

  ///
  /// 获取月所在旬（以节交接当天起算）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_month_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_month_in_gan_zhi())
  }

  ///
  /// 获取月所在旬（以节交接时刻起算）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_month_xun_exact(&self) -> String {
    LunarUtil::get_xun(&self.get_month_in_gan_zhi_exact())
  }

  ///
  /// 获取值月空亡（以节交接当天起算）
  ///
  /// ## Returns
  /// 空亡(旬空): **String**
  ///
  fn get_month_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_month_in_gan_zhi())
  }

  ///
  /// 获取值月空亡（以节交接时刻起算）
  ///
  /// ## Returns
  /// 空亡(旬空): **String**
  ///
  fn get_month_xun_kong_exact(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_month_in_gan_zhi_exact())
  }

  ///
  /// 获取日所在旬（以节交接当天起算）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_day_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_day_in_gan_zhi())
  }

  ///
  /// 获取日所在旬（晚子时日柱算明天）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_day_xun_exact(&self) -> String {
    LunarUtil::get_xun(&self.get_day_in_gan_zhi_exact())
  }

  ///
  /// 获取日所在旬（晚子时日柱算当天）
  ///
  /// ## Returns
  /// 旬: **String**
  ///
  fn get_day_xun_exact2(&self) -> String {
    LunarUtil::get_xun(&self.get_day_in_gan_zhi_exact2())
  }

  ///
  /// 获取值日空亡
  ///
  /// ## Returns
  /// 空亡(旬空): String
  ///
  fn get_day_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_day_in_gan_zhi())
  }

  ///
  /// 获取值日空亡（晚子时日柱算明天）
  ///
  /// ## Returns
  /// 空亡(旬空): String
  ///
  fn get_day_xun_kong_exact(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_day_in_gan_zhi_exact())
  }

  ///
  /// 获取值日空亡（晚子时日柱算当天）
  ///
  /// ## Returns
  /// 空亡(旬空): String
  ///
  fn get_day_xun_kong_exact2(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_day_in_gan_zhi_exact2())
  }

  ///
  /// 获取时辰所在旬
  ///
  /// ## Returns
  /// 旬: String
  ///
  fn get_time_xun(&self) -> String {
    LunarUtil::get_xun(&self.get_time_in_gan_zhi())
  }

  ///
  /// 获取值时空亡
  ///
  /// ## Returns
  /// 空亡(旬空): String
  ///
  fn get_time_xun_kong(&self) -> String {
    LunarUtil::get_xun_kong(&self.get_time_in_gan_zhi())
  }

  ///
  /// 获取数九
  ///
  /// ## Returns
  /// 数九，如果不是数九天，返回None
  ///
  fn get_shu_jiu(&self) -> Option<ShuJiuRef> {
    let (__jie_qi, year, month, day) = {
      let s = self.lock().unwrap();
      (
        s.__jie_qi.clone(),
        s.__solar.get_year(),
        s.__solar.get_month(),
        s.__solar.get_day(),
      )
    };
    let current = solar::from_ymd(year, month, day);
    let mut start = __jie_qi.get("DONG_ZHI").unwrap().clone();
    start = solar::from_ymd(
      start.get_year(),
      start.get_month(),
      start.get_day(),
    );
    if current.is_before(start.clone()) {
      start = __jie_qi.get("冬至").unwrap().clone();
      start = solar::from_ymd(
        start.get_year(),
        start.get_month(),
        start.get_day(),
      );
    }
    let end = solar::from_ymd(
      start.get_year(),
      start.get_month(),
      start.get_day(),
    )
    .next(81, None);
    if current.is_before(start.clone()) || !current.is_before(end) {
      return None;
    }
    let days = current.subtract(start);
    return Some(ShuJiu::new(
      &format!("{}九", NUMBER()[(days as f64 / 9.) as usize + 1]),
      days % 9 + 1,
    ));
  }

  ///
  /// 获取三伏
  ///
  /// ## Returns
  /// 三伏，如果不是伏天，返回None: **Option\<FuRef\>**
  ///
  fn get_fu(&self) -> Option<FuRef> {
    let current = { &self.lock().unwrap().__solar };
    let xia_zhi =
      { self.lock().unwrap().__jie_qi.get("夏至").unwrap().clone() };
    let li_qiu =
      { self.lock().unwrap().__jie_qi.get("立秋").unwrap().clone() };
    let mut start = solar::from_ymd(
      xia_zhi.get_year(),
      xia_zhi.get_month(),
      xia_zhi.get_day(),
    );
    let mut add = 6 - xia_zhi.get_lunar().get_day_gan_index();
    if add < 0 {
      add = add + 10;
    }
    add = add + 20;
    start = start.next(add, None);
    if current.is_before(start.clone()) {
      return None;
    }
    let mut days = current.subtract(start.clone());
    if days < 10 {
      return Some(Fu::new("初伏", days + 1));
    }
    start = start.next(10, None);
    days = current.subtract(start.clone());
    if days < 10 {
      return Some(Fu::new("中伏", days + 1));
    }
    start = start.clone().next(10, None);
    days = current.subtract(start.clone());
    let li_qiu_solar = solar::from_ymd(
      li_qiu.get_year(),
      li_qiu.get_month(),
      li_qiu.get_day(),
    );
    if li_qiu_solar.is_after(start.clone()) {
      if days < 10 {
        return Some(Fu::new("中伏", days + 11));
      }
      start = start.next(10, None);
      days = current.subtract(start);
    }
    if days < 10 {
      return Some(Fu::new("末伏", days + 1));
    }
    None
  }

  ///
  /// 获取六曜
  ///
  /// ## Returns
  /// 六曜: **String**
  ///
  fn get_liu_yao(&self) -> String {
    LIU_YAO()[({ self.lock().unwrap().__month } + {
      self.lock().unwrap().__day
    } - 2) as usize
      % 6]
      .to_string()
  }

  ///
  /// 获取物候
  ///
  /// ## Returns
  /// 物候: **String**
  ///
  fn get_wu_hou(&self) -> String {
    let jie_qi = self.get_prev_jie_qi(Some(true)).unwrap();
    let mut offset = 0;
    for (i, jq) in JIE_QI().iter().enumerate() {
      if jie_qi.get_name() == jq.to_string() {
        offset = i;
        break;
      }
    }
    let mut index = { self.lock().unwrap().__solar.clone() }
      .subtract(jie_qi.get_solar())
      / 5;
    if index > 2 {
      index = 2;
    }
    WU_HOU()[(offset * 3 + index as usize) % WU_HOU().len()].to_string()
  }

  fn get_hou(&self) -> String {
    let jie_qi = self.get_prev_jie_qi(Some(true)).unwrap();
    let hou = &HOU();
    let size = hou.len() as i64 - 1;
    let mut offset = ({ self.lock().unwrap().__solar.clone() }
      .subtract(jie_qi.get_solar()) as f64
      / 5.) as i64;
    if offset > size {
      offset = size;
    }
    format!("{} {}", jie_qi.get_name(), hou[offset as usize])
  }

  ///
  /// 获取日禄
  ///
  /// ## Returns
  /// 日禄: **String**
  ///
  fn get_day_lu(&self) -> String {
    let lu = LU();
    let gan = lu.get(&self.get_day_gan()).unwrap();
    let self_day_zhi = self.get_day_zhi();
    let zhi = lu.iter().find(|(key, _)| self_day_zhi == **key);
    let mut lu = format!("{}命互禄", gan.join(", "));
    if zhi.is_some() {
      lu = format!("{} {}命进禄", lu, zhi.unwrap().1.join(", "));
    }
    lu
  }

  ///
  /// 获取时辰
  ///
  /// ## Returns
  /// 时辰: **String**
  ///
  fn get_time(&self) -> LunarTimeRef {
    let (year, month, day, hour, minute, second) = {
      let s = self.lock().unwrap();
      (
        s.__year, s.__month, s.__day, s.__hour, s.__minute, s.__second,
      )
    };
    LunarTime::from_ymd_hms(year, month, day, hour, minute, second)
  }

  ///
  /// 获取当天的时辰列表
  ///
  /// ## Returns
  /// 时辰列表: **Vec\<LunarTimeRef\>**
  ///
  fn get_times(&self) -> Vec<LunarTimeRef> {
    let (year, month, day) = {
      let s = self.lock().unwrap();
      (s.__year, s.__month, s.__day)
    };
    let mut times =
      [LunarTime::from_ymd_hms(year, month, day, 0, 0, 0)].to_vec();
    for i in 0..12 {
      times.push(LunarTime::from_ymd_hms(
        year,
        month,
        day,
        (i + 1) * 2 - 1,
        0,
        0,
      ))
    }
    times
  }

  ///
  /// 获取佛历
  ///
  /// ## Returns
  /// 佛历: **FotoRef**
  ///
  fn get_foto(&self) -> FotoRef {
    foto::from_lunar(self.clone())
  }

  ///
  /// 获取道历
  ///
  /// ## Returns
  /// 道历: TaoRef
  ///
  fn get_tao(&self) -> TaoRef {
    tao::from_lunar(self.clone())
  }
}
