use super::{
  helper_traits::__LunarRefHelper, Lunar, LunarRef, LunarRefHelper,
  JIE_QI_IN_USE,
};
use crate::{
  jie_qi::{JieQi, JieQiRef},
  nine_star::{NineStar, NineStarRef},
  solar::{SolarRef, SolarRefHelper},
  util::lunar_util::LunarUtil,
};
use chrono::NaiveDateTime;
use std::collections::{HashMap, HashSet};

impl __LunarRefHelper for LunarRef {
  fn __get_year_nine_star(&self, year_in_gan_zhi: &str) -> NineStarRef {
    let index_exact = LunarUtil::get_jia_zi_index(year_in_gan_zhi) + 1;
    let index =
      LunarUtil::get_jia_zi_index(&self.get_year_in_gan_zhi()) + 1;
    let mut year_offset = index_exact - index;
    year_offset = year_offset
      + match year_offset {
        y if y > 1 => -60,
        y if y < -1 => 60,
        _ => 0,
      };

    let yuan = ((self.lock().unwrap().__year + year_offset + 2696)
      as f64
      / 60.) as i64
      % 3;
    let mut offset = (62 + yuan * 3 - index_exact) % 9;
    if 0 == offset {
      offset = 9;
    }
    NineStar::from_index(offset as usize - 1)
  }

  ///
  /// 获取最近的节气，如果未找到匹配的，返回null
  ///
  /// ## Arguments
  /// forward: **bool** - 是否顺推，true为顺推，false为逆推
  /// conditions: **&Vec<String>** - 过滤条件，如果设置过滤条件，仅返回匹配该名称的
  /// whole_day: **bool** - 是否按天计
  ///
  /// ## Returns
  /// 节气: Option<JieQi>
  ///
  fn __get_near_jie_qi(
    &self,
    forward: bool,
    conditions: &Vec<String>,
    whole_day: bool,
  ) -> Option<JieQiRef> {
    let datetime_match_whole_day =
      |solar_ref: &SolarRef| -> NaiveDateTime {
        match whole_day {
          true => {
            solar_ref.to_naivedate().and_hms_opt(0, 0, 0).unwrap()
          }
          _ => solar_ref.to_naivedatetime(),
        }
      };

    let mut name = None;
    let mut near = None;
    let mut filters = HashSet::new();

    conditions.iter().for_each(|c| {
      filters.insert(c.clone());
    });
    let is_filter = filters.len() > 0;
    let today = datetime_match_whole_day(&self.get_solar());

    for jq_key in JIE_QI_IN_USE().iter() {
      let jq = Lunar::__convert_jie_qi(jq_key.as_str());
      if is_filter && !filters.contains(&jq_key.to_string()) {
        continue;
      }
      let solar = self
        .lock()
        .unwrap()
        .__jie_qi
        .get(&jq_key.to_string())
        .unwrap()
        .clone();
      let day = datetime_match_whole_day(&solar);

      if forward {
        if day < today {
          continue;
        }
        if near.is_none() {
          name = Some(jq.clone());
          near = Some(solar.clone());
        } else {
          let near_day =
            datetime_match_whole_day(&near.clone().unwrap());
          if day < near_day {
            name = Some(jq.clone());
            near = Some(solar.clone());
          }
        }
      } else {
        if day > today {
          continue;
        }
        if near.is_none() {
          name = Some(jq.clone());
          near = Some(solar.clone());
        } else {
          let near_day =
            datetime_match_whole_day(&near.clone().unwrap());
          if day > near_day {
            name = Some(jq.clone());
            near = Some(solar.clone());
          }
        }
      }
    }

    if near.is_none() {
      return None;
    }

    Some(JieQi::new(&name.unwrap().clone(), &near.unwrap().clone()))
  }

  fn __get_jie_qi_map(&self) -> HashMap<String, SolarRef> {
    self.lock().unwrap().__jie_qi.clone()
  }
}
