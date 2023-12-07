#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod eight_char;
pub mod foto;
pub mod foto_festival;
pub mod fu;
pub mod holiday;
pub mod jie_qi;
pub mod lunar;
pub mod lunar_month;
pub mod lunar_time;
pub mod lunar_year;
pub mod nine_star;
pub mod shu_jiu;
pub mod solar;
pub mod solar_month;
pub mod solar_season;
pub mod solar_week;
pub mod solar_year;
pub mod tao;
pub mod tao_festival;
mod util;

#[cfg(test)]
#[path = "../tests/chinese_test.rs"]
mod chinese_test;

#[cfg(test)]
#[path = "../tests/eight_char_test.rs"]
mod eight_char_test;

#[cfg(test)]
#[path = "../tests/foto_test.rs"]
mod foto_test;

#[cfg(test)]
#[path = "../tests/holiday_test.rs"]
mod holiday_test;

#[cfg(test)]
#[path = "../tests/hou_test.rs"]
mod hou_test;

#[cfg(test)]
#[path = "../tests/jie_qi_test.rs"]
mod jie_qi_test;

#[cfg(test)]
#[path = "../tests/lunar_month_test.rs"]
mod lunar_month_test;

#[cfg(test)]
#[path = "../tests/lunar_test.rs"]
mod lunar_test;

#[cfg(test)]
#[path = "../tests/nine_star_test.rs"]
mod nine_star_test;

#[cfg(test)]
#[path = "../tests/shu_jiu_test.rs"]
mod shu_jiu_test;

#[cfg(test)]
#[path = "../tests/solar_month_test.rs"]
mod solar_month_test;

#[cfg(test)]
#[path = "../tests/solar_season_test.rs"]
mod solar_season_test;

#[cfg(test)]
#[path = "../tests/solar_test.rs"]
mod solar_test;

#[cfg(test)]
#[path = "../tests/solar_week_test.rs"]
mod solar_week_test;

#[cfg(test)]
#[path = "../tests/solar_year_test.rs"]
mod solar_year_test;

#[cfg(test)]
#[path = "../tests/tao_test.rs"]
mod tao_test;

#[cfg(test)]
#[path = "../tests/week_test.rs"]
mod week_test;

#[cfg(test)]
#[path = "../tests/wu_hou_test.rs"]
mod wu_hou_test;

#[cfg(test)]
#[path = "../tests/xing_zuo_test.rs"]
mod xing_zuo_test;

#[cfg(test)]
#[path = "../tests/xun_test.rs"]
mod xun_test;

#[cfg(test)]
#[path = "../tests/year_test.rs"]
mod year_test;

#[cfg(test)]
#[path = "../tests/yun_test.rs"]
mod yun_test;
