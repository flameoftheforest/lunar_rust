mod helper_traits;
pub use self::helper_traits::{RefHelper as SolarRefHelper, SolarRef};
use crate::{
  lunar::LunarRefHelper,
  util::lunar_util::{LunarUtil, ZHI},
};
use chrono::{Datelike, Local, NaiveDateTime, Timelike};
use once_cell::sync::Lazy;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  thread::{self},
};

// 阳历日期
pub fn from_ymdhms(
  year: i64,
  month: i64,
  day: i64,
  hour: i64,
  minute: i64,
  second: i64,
) -> SolarRef {
  Solar::__new(year, month, day, hour, minute, second, None)
}

#[allow(non_snake_case)]
pub fn J2000() -> i64 {
  __J2000
}

pub fn from_date(date: NaiveDateTime) -> SolarRef {
  Solar::__new(
    date.year() as i64,
    date.month() as i64,
    (date.day0() + 1) as i64,
    date.hour() as i64,
    date.minute() as i64,
    date.second() as i64,
    None,
  )
}

///
/// be very careful, this function is **VERY SENSITIVE**
///
pub fn from_julian_day(julian_day: f64) -> SolarRef {
  let mut d = (julian_day + 0.5).floor();
  let mut f = (julian_day + 0.5) - d;
  if d >= 2299161. {
    let c = ((d - 1867216.25) / 36524.25).floor();
    d = d + 1. + c - (c / 4.0).floor();
  }
  d = d + 1524.;
  let mut year = ((d - 122.1) / 365.25).floor();
  d = d - (365.25 * year).floor();
  let mut month = (d / 30.601).floor();
  d = d - (30.601 * month).floor();

  let mut day = d;
  if month > 13. {
    month = month - 13.;
    year = year - 4715.;
  } else {
    month = month - 1.;
    year = year - 4716.;
  }
  f = f * 24.;
  let mut hour = f.floor();

  f = f - hour;
  f = f * 60.;
  let mut minute = f.floor();

  f = f - minute;
  f = f * 60.;
  let mut second = f.round().floor();
  if second > 59. {
    second = second - 60.;
    minute = minute + 1.;
  }
  if minute > 59. {
    minute = minute - 60.;
    hour = hour + 1.;
  }
  if hour > 23. {
    hour = hour - 24.;
    day = day + 1.;
  }

  let s = Solar::__new(
    year as i64,
    month as i64,
    day as i64,
    hour as i64,
    minute as i64,
    second as i64,
    Some(julian_day),
  );
  s
}

pub fn from_ymd(year: i64, month: i64, day: i64) -> SolarRef {
  Solar::__new(year, month, day, 0, 0, 0, None)
}

pub fn from_bazi(
  year_gan_zhi: &str,
  month_gan_zhi: &str,
  day_gan_zhi: &str,
  time_gan_zhi: &str,
  sect: Option<i64>,
  base_year: Option<i64>,
) -> Vec<SolarRef> {
  // let mut measurement = TimeElapsed::start_print("from_bazi 01");

  let sect = (match sect.unwrap_or(2) {
    1 => 1,
    _ => 2,
  }) as i64;
  let base_year = base_year.unwrap_or(1900);
  let mut years: Vec<i64> = vec![];
  let today = from_date({
    let date = Local::now().date_naive();
    let time = Local::now().time();
    date.and_time(time)
  });

  // measurement.end_print_delta(true);

  // ---------------------------
  let mut offset_year = LunarUtil::get_jia_zi_index(
    &today.get_lunar().get_year_in_gan_zhi_exact(),
  ) - LunarUtil::get_jia_zi_index(year_gan_zhi);

  // measurement.end_print_delta(true);

  if offset_year < 0 {
    offset_year = offset_year + 60;
  }
  let mut start_year = today.get_year() - offset_year - 1;
  let min_year = base_year - 2;
  while start_year >= min_year {
    years.push(start_year);
    start_year = start_year - 60;
  }

  // measurement.end_print_delta(true);

  let mut hours: Vec<i64> = vec![];
  let time_zhi = LunarUtil::split_ganzhi(time_gan_zhi).1;
  for (i, zhi) in ZHI().iter().enumerate() {
    if *zhi == time_zhi {
      hours.push((i - 1) as i64 * 2);
      break;
    }
  }
  if "子" == &time_zhi {
    hours.push(23);
  }

  // measurement.end_print_delta(false);
  #[derive(Debug, Clone)]
  struct Zipped {
    solar: SolarRef,
    year_gan_zhi: String,
    month_gan_zhi: String,
    day_gan_zhi: String,
    time_gan_zhi: String,
    sect: i64,
  }

  let handle_zipped = |item: Zipped| {
    let mut collected = vec![];
    let lunar = item.solar.get_lunar();
    let dgz = match item.sect == 2 {
      true => lunar.get_day_in_gan_zhi_exact2(),
      _ => lunar.get_day_in_gan_zhi_exact(),
    };
    if lunar.get_year_in_gan_zhi_exact() == item.year_gan_zhi
      && lunar.get_month_in_gan_zhi_exact() == item.month_gan_zhi
      && dgz == item.day_gan_zhi
      && lunar.get_time_in_gan_zhi() == item.time_gan_zhi
    {
      collected.push(item.solar.clone());
    }
    collected
  };

  let mut zippeds = vec![];
  for hour in hours {
    for y in years.clone() {
      let max_year = y + 3;
      let mut year = y;
      let mut month = 11 as i64;
      if year < base_year {
        year = base_year;
        month = 1;
      }
      let mut solar = Solar::__new(year, month, 1, hour, 0, 0, None);

      while solar.get_year() <= max_year {
        zippeds.push(Zipped {
          solar: solar.clone(),
          year_gan_zhi: year_gan_zhi.to_string(),
          month_gan_zhi: month_gan_zhi.to_string(),
          day_gan_zhi: day_gan_zhi.to_string(),
          time_gan_zhi: time_gan_zhi.to_string(),
          sect: sect.clone(),
        });
        solar = solar.next(1, None);
      }
    }
  }

  let zipped_chunks =
    zippeds.chunks(10).map(|z| z.to_vec()).collect::<Vec<_>>();
  let mut join_handles = vec![];
  for (_i, zipped_chunk) in zipped_chunks.into_iter().enumerate() {
    let zipped_chunk = zipped_chunk.clone();
    let join_handle = thread::spawn(move || {
      let mut collected = vec![];
      for zipped in zipped_chunk {
        collected.push(handle_zipped(zipped.clone()));
      }
      collected.into_iter().flatten().collect::<Vec<_>>()
    });
    join_handles.push(join_handle);
  }
  let _total_handles = join_handles.len();
  let mut solar_vecs = vec![];
  for (_i, join_handle) in join_handles.into_iter().enumerate() {
    solar_vecs.push(join_handle.join().unwrap());
  }
  solar_vecs.into_iter().flatten().collect::<Vec<_>>()
}

#[derive(Clone, Debug)]
pub struct Solar {
  __year: i64,
  __month: i64,
  __day: i64,
  __hour: i64,
  __minute: i64,
  __second: i64,
  __julian_day: Option<f64>,
}

impl Solar {
  fn __cache() -> Arc<Mutex<HashMap<String, SolarRef>>> {
    static __SOLAR_CACHE: Lazy<Arc<Mutex<HashMap<String, SolarRef>>>> =
      Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
    __SOLAR_CACHE.clone()
  }

  fn __new(
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
    julian_day: Option<f64>,
  ) -> SolarRef {
    let key = format!(
      "{}-{}-{} {}:{}:{}",
      year, month, day, hour, minute, second
    );

    {
      let s = Self::__cache();
      let s = s.lock().unwrap();
      let s = s.get(&key);
      if s.is_some() {
        return s.unwrap().clone();
      }
    };

    let s = Arc::new(Mutex::new(Self {
      __year: year,
      __month: month,
      __day: day,
      __hour: hour,
      __minute: minute,
      __second: second,
      __julian_day: julian_day,
    }));

    {
      Self::__cache().lock().unwrap().insert(key, s.clone());
    }

    if julian_day.is_none() {
      s.get_julian_day();
    }

    s
  }
}

const __J2000: i64 = 2451545;
