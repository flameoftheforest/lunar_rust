mod __lunar_ref_helper_defs;
mod helper_traits;
mod lunar_ref_helper_defs;
pub use self::helper_traits::LunarRef;
pub use self::helper_traits::LunarRefHelper;
use crate::lunar_month::LunarMonthRefHelper;
use crate::{
  eight_char::EightCharRef,
  lunar_year::{LunarYear, LunarYearRefHelper},
  nine_star::{NineStar, NineStarRef},
  solar::{self, SolarRef, SolarRefHelper},
  util::{
    lunar_util::{
      LunarUtil, BASE_MONTH_ZHI_INDEX, POSITION_GAN,
      POSITION_TAI_SUI_YEAR,
    },
    mmacro::__static_funk,
  },
};
use chrono::{NaiveDateTime, NaiveTime};
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

pub fn from_ymd_hms(
  lunar_year: i64,
  lunar_month: i64,
  lunar_day: i64,
  hour: i64,
  minute: i64,
  second: i64,
) -> LunarRef {
  Lunar::__new(lunar_year, lunar_month, lunar_day, hour, minute, second)
}

pub fn from_ymd(
  lunar_year: i64,
  lunar_month: i64,
  lunar_day: i64,
) -> LunarRef {
  Lunar::__new(lunar_year, lunar_month, lunar_day, 0, 0, 0)
}

pub fn from_date(date: NaiveDateTime) -> LunarRef {
  from_solar(&solar::from_date(date))
}

pub fn from_solar(solar: &SolarRef) -> LunarRef {
  let mut year = 0;
  let mut month = 0;
  let mut day = 0;
  let ly = LunarYear::from_lunar_year(solar.get_year());
  for m in ly.get_months().iter() {
    let days =
      solar.subtract(solar::from_julian_day(m.get_first_julian_day()));
    if days < m.get_day_count() {
      year = m.get_year();
      month = m.get_month();
      day = days + 1;
      break;
    }
  }

  Lunar::__new(
    year,
    month,
    day,
    solar.get_hour(),
    solar.get_minute(),
    solar.get_second(),
  )
}

#[derive(Clone, Debug)]
pub struct Lunar {
  __year: i64,
  __month: i64,
  __day: i64,
  __hour: i64,
  __minute: i64,
  __second: i64,
  __jie_qi: HashMap<String, SolarRef>,
  __jie_qi_list: Vec<String>,
  __eight_char: Option<EightCharRef>,
  __solar: SolarRef,

  __year_gan_index: i64,
  __year_zhi_index: i64,
  __year_gan_index_by_li_chun: i64,
  __year_zhi_index_by_li_chun: i64,
  __year_gan_index_exact: i64,
  __year_zhi_index_exact: i64,

  __month_gan_index: i64,
  __month_zhi_index: i64,
  __month_gan_index_exact: i64,
  __month_zhi_index_exact: i64,

  __day_gan_index: i64,
  __day_zhi_index: i64,
  __day_gan_index_exact2: i64,
  __day_zhi_index_exact2: i64,
  __day_gan_index_exact: i64,
  __day_zhi_index_exact: i64,

  __time_zhi_index: i64,
  __time_gan_index: i64,

  __week_index: i64,
}

impl Lunar {
  fn __new(
    lunar_year: i64,
    lunar_month: i64,
    lunar_day: i64,
    hour: i64,
    minute: i64,
    second: i64,
  ) -> LunarRef {
    let key = format!(
      "{}-{}-{} {}:{}:{}",
      lunar_year, lunar_month, lunar_day, hour, minute, second
    );
    {
      let ll = __LUNAR_CACHE();
      let ll = ll.lock().unwrap();
      let l = ll.get(&key);
      if l.is_some() {
        return l.unwrap().clone();
      }
    }

    let mut y = LunarYear::from_lunar_year(lunar_year);
    let m = y.get_month(lunar_month);
    assert!(
      m.is_some(),
      "wrong lunar year {lunar_year} month {lunar_month}"
    );
    assert!(lunar_day > 0, "lunar_day must be bigger than 0");
    let days = m.clone().unwrap().get_day_count();
    assert!(lunar_day <= days, "only {days} days in lunar_year {lunar_year} lunar_month {lunar_month} ");

    let __month = lunar_month;
    let __day = lunar_day;
    let __hour = hour;
    let __minute = minute;
    let __second = second;
    let __eight_char = None;

    let m = m.unwrap();
    let noon = solar::from_julian_day(
      m.get_first_julian_day() + (lunar_day - 1) as f64,
    );
    let __solar = solar::from_ymdhms(
      noon.get_year(),
      noon.get_month(),
      noon.get_day(),
      hour,
      minute,
      second,
    );
    if noon.get_year() != lunar_year {
      y = LunarYear::from_lunar_year(noon.get_year());
    }

    let (__jie_qi, __jie_qi_list) = Self::__compute_jie_qi(y);
    let (
      __year,
      __year_gan_index,
      __year_zhi_index,
      __year_gan_index_by_li_chun,
      __year_zhi_index_by_li_chun,
      __year_gan_index_exact,
      __year_zhi_index_exact,
    ) = Self::__compute_year(&__solar, &__jie_qi, lunar_year);
    let (
      __month_gan_index,
      __month_zhi_index,
      __month_gan_index_exact,
      __month_zhi_index_exact,
    ) = Self::__compute_month(
      &__solar,
      &__jie_qi,
      __year_gan_index_by_li_chun,
      __year_gan_index_exact,
    );

    let (
      __day_gan_index,
      __day_zhi_index,
      __day_gan_index_exact2,
      __day_zhi_index_exact2,
      __day_gan_index_exact,
      __day_zhi_index_exact,
    ) = Self::__compute_day(&__solar, &__hour, &__minute);

    let (__time_zhi_index, __time_gan_index) =
      Self::__compute_time(&__hour, &__minute, &__day_gan_index_exact);

    let __week_index = Self::__compute_week(&__solar);

    let __self = Self {
      __year,
      __month,
      __day,
      __hour,
      __minute,
      __second,
      __jie_qi,
      __jie_qi_list,
      __eight_char,
      __solar,

      __year_gan_index,
      __year_zhi_index,
      __year_gan_index_by_li_chun,
      __year_zhi_index_by_li_chun,
      __year_gan_index_exact,
      __year_zhi_index_exact,

      __month_gan_index,
      __month_zhi_index,
      __month_gan_index_exact,
      __month_zhi_index_exact,

      __day_gan_index,
      __day_zhi_index,
      __day_gan_index_exact2,
      __day_zhi_index_exact2,
      __day_gan_index_exact,
      __day_zhi_index_exact,

      __time_zhi_index,
      __time_gan_index,

      __week_index,
    };

    let l = Arc::new(Mutex::new(__self));
    {
      __LUNAR_CACHE().lock().unwrap().insert(key, l.clone());
    }
    l
  }

  pub fn get_jie_qi_in_use() -> Vec<String> {
    JIE_QI_IN_USE()
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<_>>()
  }
}

impl Lunar {
  fn __compute_jie_qi(
    y: Arc<Mutex<LunarYear>>,
  ) -> (HashMap<String, SolarRef>, Vec<String>) {
    let julian_days = y.get_jie_qi_julian_days();
    let mut jie_qi: HashMap<String, SolarRef> = HashMap::new();
    let mut jie_qi_list = vec![];
    JIE_QI_IN_USE().iter().enumerate().for_each(|(i, jq)| {
      let name = jq.to_string();
      jie_qi
        .insert(name.clone(), solar::from_julian_day(julian_days[i]));
      jie_qi_list.push(name);
    });
    (jie_qi, jie_qi_list)
  }

  fn __compute_year(
    solar: &SolarRef,
    jie_qi: &HashMap<String, SolarRef>,
    lunar_year: i64,
  ) -> (i64, i64, i64, i64, i64, i64, i64) {
    // 以正月初一开始
    let offset = lunar_year - 4;
    let mut year_gan_index = offset % 10;
    let mut year_zhi_index = offset % 12;

    if year_gan_index < 0 {
      year_gan_index = year_gan_index + 10;
    }

    if year_zhi_index < 0 {
      year_zhi_index = year_zhi_index + 12;
    }

    // 以立春作为新一年的开始的干支纪年
    let mut g = year_gan_index;
    let mut z = year_zhi_index;

    // 精确的干支纪年，以立春交接时刻为准
    let mut g_exact = year_gan_index;
    let mut z_exact = year_zhi_index;

    let solar_year = solar.get_year();
    let solar_ymd = solar.to_naivedate();
    let solar_ymd_hms = solar.to_naivedatetime();

    // 获取立春的阳历时刻
    let mut li_chun = jie_qi.get("立春").unwrap();
    if li_chun.get_year() != solar_year {
      li_chun = jie_qi.get("LI_CHUN").unwrap();
    }
    let li_chun_ymd = li_chun.to_naivedate();
    let li_chun_ymd_hms = li_chun.to_naivedatetime();

    // 阳历和阴历年份相同代表正月初一及以后
    if lunar_year == solar_year {
      // 立春日期判断
      if solar_ymd < li_chun_ymd {
        g = g - 1;
        z = z - 1;
      }
      // 立春交接时刻判断
      if solar_ymd_hms < li_chun_ymd_hms {
        g_exact = g_exact - 1;
        z_exact = z_exact - 1;
      }
    } else if lunar_year < solar_year {
      if solar_ymd >= li_chun_ymd {
        g = g + 1;
        z = z + 1;
      }
      if solar_ymd_hms >= li_chun_ymd_hms {
        g_exact = g_exact + 1;
        z_exact = z_exact + 1;
      }
    }

    let __year_gan_index = year_gan_index;
    let __year_zhi_index = year_zhi_index;

    let __year_gan_index_by_li_chun = {
      (match g < 0 {
        true => g + 10,
        _ => g,
      }) % 10
    };
    let __year_zhi_index_by_li_chun = {
      (match z < 0 {
        true => z + 12,
        _ => z,
      }) % 12
    };
    let __year_gan_index_exact = {
      (match g_exact < 0 {
        true => g_exact + 10,
        _ => g_exact,
      }) % 10
    };
    let __year_zhi_index_exact = {
      (match z_exact < 0 {
        true => z_exact + 12,
        _ => z_exact,
      }) % 12
    };
    let __year = lunar_year;

    let r = (
      __year,
      __year_gan_index,
      __year_zhi_index,
      __year_gan_index_by_li_chun,
      __year_zhi_index_by_li_chun,
      __year_gan_index_exact,
      __year_zhi_index_exact,
    );
    r
  }

  fn __compute_month(
    solar: &SolarRef,
    jie_qi: &HashMap<String, SolarRef>,
    year_gan_index_by_li_chun: i64,
    year_gan_index_exact: i64,
  ) -> (i64, i64, i64, i64) {
    let ymd = solar.to_naivedate();
    let time = solar.to_naivedatetime();

    // 序号：大雪以前-3，大雪到小寒之间-2，小寒到立春之间-1，立春之后0
    let mut index = -3;
    let mut start: Option<SolarRef> = None;

    for jq in JIE_QI_IN_USE().iter().step_by(2) {
      let end = jie_qi.get(&jq.to_string()).unwrap();
      let symd = match start.is_none() {
        true => ymd,
        _ => start.unwrap().to_naivedate(),
      };

      if symd <= ymd && ymd < end.to_naivedate() {
        break;
      }
      start = Some(end.clone());
      index = index + 1;
    }

    // 干偏移值（以立春当天起算）
    let g_offset = (((year_gan_index_by_li_chun + {
      match index < 0 {
        true => 1,
        _ => 0,
      }
    }) % 5
      + 1)
      * 2)
      % 10;
    let __month_gan_index = (({
      match index < 0 {
        true => index + 10,
        _ => index,
      }
    }) + g_offset)
      % 10;
    let __month_zhi_index = (({
      match index < 0 {
        true => index + 12,
        _ => index,
      }
    }) + BASE_MONTH_ZHI_INDEX())
      % 12;

    index = -3;
    start = None;
    for jq in JIE_QI_IN_USE().iter().step_by(2) {
      let end = jie_qi.get(&jq.to_string()).unwrap();
      let stime = match start.is_none() {
        true => time,
        _ => start.unwrap().to_naivedatetime(),
      };

      if stime <= time && time < end.to_naivedatetime() {
        break;
      }
      start = Some(end.clone());
      index = index + 1;
    }
    // 干偏移值（以立春交接时刻起算）
    let g_offset = (((year_gan_index_exact + {
      match index < 0 {
        true => 1,
        _ => 0,
      }
    }) % 5
      + 1)
      * 2)
      % 10;

    let __month_gan_index_exact = (({
      match index < 0 {
        true => index + 10,
        _ => index,
      }
    }) + g_offset)
      % 10;
    let __month_zhi_index_exact = (({
      match index < 0 {
        true => index + 12,
        _ => index,
      }
    }) + BASE_MONTH_ZHI_INDEX())
      % 12;

    let r = (
      __month_gan_index,
      __month_zhi_index,
      __month_gan_index_exact,
      __month_zhi_index_exact,
    );
    r
  }

  fn __compute_day(
    solar: &SolarRef,
    hour: &i64,
    minute: &i64,
  ) -> (i64, i64, i64, i64, i64, i64) {
    let noon = solar::from_ymdhms(
      solar.get_year(),
      solar.get_month(),
      solar.get_day(),
      12,
      0,
      0,
    );
    let offset = noon.get_julian_day() as i64 - 11;
    let day_gan_index = offset % 10;
    let day_zhi_index = offset % 12;
    let __day_gan_index = day_gan_index;
    let __day_zhi_index = day_zhi_index;
    let mut day_gan_exact = day_gan_index;
    let mut day_zhi_exact = day_zhi_index;
    // 八字流派2，晚子时（夜子/子夜）日柱算当天
    let __day_gan_index_exact2 = day_gan_exact;
    let __day_zhi_index_exact2 = day_zhi_exact;

    // 八字流派1，晚子时（夜子/子夜）日柱算明天
    let hm = NaiveTime::from_hms_opt(
      hour.clone() as u32,
      minute.clone() as u32,
      0,
    )
    .unwrap();
    let min_time = NaiveTime::from_hms_opt(23, 0, 0).unwrap();
    let max_time = NaiveTime::from_hms_opt(23, 59, 0).unwrap();
    if min_time <= hm && hm <= max_time {
      day_gan_exact = day_gan_exact + 1;
      if day_gan_exact >= 10 {
        day_gan_exact = day_gan_exact - 10;
      }
      day_zhi_exact = day_zhi_exact + 1;
      if day_zhi_exact >= 12 {
        day_zhi_exact = day_zhi_exact - 12;
      }
    }
    let __day_gan_index_exact = day_gan_exact;
    let __day_zhi_index_exact = day_zhi_exact;

    let r = (
      __day_gan_index,
      __day_zhi_index,
      __day_gan_index_exact2,
      __day_zhi_index_exact2,
      __day_gan_index_exact,
      __day_zhi_index_exact,
    );
    r
  }

  fn __compute_time(
    hour: &i64,
    minute: &i64,
    day_gan_index_exact: &i64,
  ) -> (i64, i64) {
    let time_zhi_index = LunarUtil::get_time_zhi_index(&format!(
      "{}:{}",
      match *hour < 10 {
        true => format!("0{}", hour),
        _ => format!("{}", hour),
      },
      match *minute < 10 {
        true => format!("0{}", minute),
        _ => format!("{}", minute),
      }
    ));

    let __time_zhi_index = time_zhi_index;
    let __time_gan_index =
      (day_gan_index_exact % 5 * 2 + time_zhi_index) % 10;

    (__time_zhi_index, __time_gan_index)
  }

  fn __compute_week(solar: &SolarRef) -> i64 {
    solar.get_week()
  }

  fn __get_month_position_tai_sui(
    month_zhi_index: i64,
    month_gan_index: i64,
  ) -> String {
    let mut m = month_zhi_index - BASE_MONTH_ZHI_INDEX();
    if m < 0 {
      m = m + 12;
    }
    m = m % 4;
    let position = POSITION_GAN();
    let p = match m {
      0 => "艮",
      2 => "坤",
      3 => "巽",
      _ => position[month_gan_index as usize].as_str(),
    };
    p.to_string()
  }

  fn __get_day_position_tai_sui(
    day_in_gan_zhi: &str,
    year_zhi_index: i64,
  ) -> String {
    let to_find = [
      "甲子,乙丑,丙寅,丁卯,戊辰,已巳"
        .split(",")
        .collect::<Vec<_>>(),
      "丙子,丁丑,戊寅,已卯,庚辰,辛巳"
        .split(",")
        .collect::<Vec<_>>(),
      "戊子,已丑,庚寅,辛卯,壬辰,癸巳"
        .split(",")
        .collect::<Vec<_>>(),
      "庚子,辛丑,壬寅,癸卯,甲辰,乙巳"
        .split(",")
        .collect::<Vec<_>>(),
      "壬子,癸丑,甲寅,乙卯,丙辰,丁巳"
        .split(",")
        .collect::<Vec<_>>(),
    ];
    let response = ["震", "离", "中", "兑", "坎"];
    let zipped =
      to_find.iter().zip(response.iter()).collect::<Vec<_>>();
    let found = zipped.iter().find_map(|(to_find, response)| {
      match to_find
        .iter()
        .find(|vv| vv.trim() == day_in_gan_zhi)
        .is_some()
      {
        true => Some(response.to_string()),
        _ => None,
      }
    });
    match found {
      Some(f) => f,
      _ => POSITION_TAI_SUI_YEAR()[year_zhi_index as usize].to_string(),
    }
  }

  fn __convert_jie_qi(name: &str) -> String {
    let __from = [
      "DONG_ZHI", "DA_HAN", "XIAO_HAN", "LI_CHUN", "DA_XUE", "YU_SHUI",
      "JING_ZHE",
    ];
    let __to = ["冬至", "大寒", "小寒", "立春", "大雪", "雨水", "惊蛰"];
    let zipped = __from.iter().zip(__to.iter()).collect::<Vec<_>>();
    zipped
      .iter()
      .find_map(|(__from, __to)| {
        if **__from == name {
          return Some(__to.to_string());
        }
        None
      })
      .unwrap_or(name.to_string())
  }

  fn __get_month_nine_star(
    year_zhi_index: i64,
    month_zhi_index: i64,
  ) -> NineStarRef {
    let index = year_zhi_index % 3;
    let mut n = 27 - index * 3;
    if month_zhi_index < BASE_MONTH_ZHI_INDEX() {
      n = n - 3;
    }
    let offset = (n - month_zhi_index) % 9;
    NineStar::from_index(offset as usize)
  }
}

__static_funk!(
  JIE_QI,
  Vec<String>,
  [
    "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明",
    "谷雨", "立夏", "小满", "芒种", "夏至", "小暑", "大暑", "立秋",
    "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  JIE_QI_IN_USE,
  Vec<String>,
  [
    "DA_XUE", "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分",
    "清明", "谷雨", "立夏", "小满", "芒种", "夏至", "小暑", "大暑",
    "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪",
    "大雪", "DONG_ZHI", "XIAO_HAN", "DA_HAN", "LI_CHUN", "YU_SHUI",
    "JING_ZHE"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  __LUNAR_CACHE,
  Arc<Mutex<HashMap<String, LunarRef>>>,
  Arc::new(Mutex::new(HashMap::new()))
);
