use chrono::NaiveDate;
use crate::{solar::{SolarRefHelper, self}, lunar::{LunarRefHelper, self}, eight_char::{EightCharRefHelper, yun::YunRefHelper}};

#[test]
fn eight_char_gan_zhi() {
  let solar = solar::from_ymdhms(2005, 12, 23, 8, 37, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  assert_eq!("乙酉", &eight_char.get_year());
  assert_eq!("戊子", &eight_char.get_month());
  assert_eq!("辛巳", &eight_char.get_day());
  assert_eq!("壬辰", &eight_char.get_time());
}

#[test]
fn eight_char_shen_gong() {
  let lunar = solar::from_ymdhms(1995, 12, 18, 10, 28, 0).get_lunar();
  assert_eq!("壬午", &lunar.get_eight_char().get_shen_gong());
}

#[test]
fn eight_char_shen_gong_1() {
  let lunar = solar::from_ymdhms(1994, 12, 6, 2, 0, 0).get_lunar();
  assert_eq!("丁丑", &lunar.get_eight_char().get_shen_gong());
}

#[test]
fn eight_char_shen_gong2() {
  let lunar = solar::from_ymdhms(1990, 12, 11, 6, 0, 0).get_lunar();
  assert_eq!("庚辰", &lunar.get_eight_char().get_shen_gong());
}

#[test]
fn eight_char_shen_gong3() {
  let lunar = solar::from_ymdhms(1993, 5, 23, 4, 0, 0).get_lunar();
  assert_eq!("庚申", &lunar.get_eight_char().get_shen_gong());
}

#[test]
fn eight_char_test4() {
  let lunar = lunar::from_ymd(1985, 12, 27);
  assert_eq!("1995-11-05", &lunar.get_eight_char().get_yun(1, None).get_start_solar().to_ymd());
}

#[test]
fn eight_char_test5() {
  let lunar = lunar::from_ymd(1985, 1, 27);
  assert_eq!("1989-03-28", &lunar.get_eight_char().get_yun(1, None).get_start_solar().to_ymd());
}

#[test]
fn eight_char_test6() {
  let lunar = lunar::from_ymd(1986, 12, 27);
  assert_eq!("1990-04-15", &lunar.get_eight_char().get_yun(1, None).get_start_solar().to_ymd());
}

#[test]
fn eight_char_test7() {
  let solar = solar::from_ymdhms(2022, 8, 28, 1, 50, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  assert_eq!("壬寅", &eight_char.get_year());
  assert_eq!("戊申", &eight_char.get_month());
  assert_eq!("癸丑", &eight_char.get_day());
  assert_eq!("癸丑", &eight_char.get_time());
}

#[test]
fn eight_char_test8() {
  let lunar = lunar::from_ymd_hms(2022, 8, 2, 1, 50, 0);
  let eight_char: std::sync::Arc<std::sync::Mutex<crate::eight_char::EightChar>> = lunar.get_eight_char();
  assert_eq!("壬寅", &eight_char.get_year());
  assert_eq!("戊申", &eight_char.get_month());
  assert_eq!("癸丑", &eight_char.get_day());
  assert_eq!("癸丑", &eight_char.get_time());
}

#[test]
fn eight_char_test9() {
  // datetime.strptime('2022-08-28 01:50:00', '%Y-%m-%d %H:%M:%S')
  let lunar = lunar::from_date(NaiveDate::from_ymd_opt(2022, 8, 28).unwrap().and_hms_opt(1, 50, 0).unwrap());
  let eight_char = lunar.get_eight_char();
  assert_eq!("壬寅", &eight_char.get_year());
  assert_eq!("戊申", &eight_char.get_month());
  assert_eq!("癸丑", &eight_char.get_day());
  assert_eq!("癸丑", &eight_char.get_time());
}

#[test]
fn eight_char_test10() {
  let lunar = solar::from_ymdhms(1988, 2, 15, 23, 30, 0).get_lunar();
  let eight_char = lunar.get_eight_char();
  assert_eq!("戊辰", &eight_char.get_year());
  assert_eq!("甲寅", &eight_char.get_month());
  assert_eq!("庚子", &eight_char.get_day());
  assert_eq!("戊子", &eight_char.get_time());
}

#[test]
fn eight_char_test11() {
  let lunar = lunar::from_ymd_hms(1987, 12, 28, 23, 30, 0);
  let eight_char = lunar.get_eight_char();
  assert_eq!("戊辰", &eight_char.get_year());
  assert_eq!("甲寅", &eight_char.get_month());
  assert_eq!("庚子", &eight_char.get_day());
  assert_eq!("戊子", &eight_char.get_time());
}

#[test]
fn eight_char_test12() {
  let solar_list = solar::from_bazi("己卯", "辛未", "甲戌", "癸酉", None, None);
  assert!(1 < solar_list.len());
}

#[test]
fn eight_char_test13() {
  let lunar = lunar::from_ymd_hms(1991, 4, 5, 3, 37, 0);
  let eight_char = lunar.get_eight_char();
  assert_eq!("辛未", &eight_char.get_year());
  assert_eq!("癸巳", &eight_char.get_month());
  assert_eq!("戊子", &eight_char.get_day());
  assert_eq!("甲寅", &eight_char.get_time());
}

#[test]
fn eight_char_test14() {
  let solar_list = solar::from_bazi("己卯", "辛未", "甲戌", "壬申", None, None);
  let actual = solar_list.iter().map(|s| s.to_naivedatetime()).collect::<Vec<_>>();
  let expected = vec![
    NaiveDate::from_ymd_opt(1999, 7, 21).unwrap().and_hms_opt(16, 0, 0).unwrap(),
    NaiveDate::from_ymd_opt(1939, 8, 5).unwrap().and_hms_opt(16, 0, 0).unwrap(),
  ];
  assert_eq!(expected, actual);
}

#[test]
fn eight_char_test15() {
  let solar_list = solar::from_bazi("庚子", "戊子", "己卯", "庚午", None, None);
  let actual = solar_list.iter().map(|s| s.to_naivedatetime()).collect::<Vec<_>>();
  let expected = vec![
    NaiveDate::from_ymd_opt(1960, 12, 17).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    NaiveDate::from_ymd_opt(1901, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
  ];
  assert_eq!(expected, actual);
}

#[test]
fn eight_char_test16() {
  let solar_list = solar::from_bazi("癸卯", "甲寅", "癸丑", "甲子", Some(2), Some(1843));
  let actual = solar_list.iter().map(|s| s.to_naivedatetime()).collect::<Vec<_>>();
  let expected = vec![
    NaiveDate::from_ymd_opt(2023, 2, 24).unwrap().and_hms_opt(23, 0, 0).unwrap(),
    NaiveDate::from_ymd_opt(1843, 2, 8).unwrap().and_hms_opt(23, 0, 0).unwrap(),
  ];
  assert_eq!(expected, actual);
}

#[test]
fn eight_char_test17() {
  let solar_list = solar::from_bazi("己亥", "丁丑", "壬寅", "戊申", None, None);
  let actual = solar_list.iter().map(|s| s.to_naivedatetime()).collect::<Vec<_>>();
  let expected = vec![
    NaiveDate::from_ymd_opt(1960, 1, 15).unwrap().and_hms_opt(16, 0, 0).unwrap(),
    NaiveDate::from_ymd_opt(1900, 1, 29).unwrap().and_hms_opt(16, 0, 0).unwrap(),
  ];
  assert_eq!(expected, actual);  
}

#[test]
fn eight_char_test18() {
  let solar_list = solar::from_bazi("己亥", "丙子", "癸酉", "庚申", None, None);
  let actual = solar_list.iter().map(|s| s.to_naivedatetime()).collect::<Vec<_>>();
  let expected = vec![
    NaiveDate::from_ymd_opt(1959, 12, 17).unwrap().and_hms_opt(16, 0, 0).unwrap(),
  ];
  assert_eq!(expected, actual);  
}
