use crate::{solar::{SolarRefHelper, self}, lunar::LunarRefHelper, eight_char::EightCharRefHelper};


#[test]
fn test_xun1() {
  let solar = solar::from_ymdhms(2020, 11, 19, 0, 0, 0);
  let lunar = solar.get_lunar();
  assert_eq!("甲午", lunar.get_year_xun());
}

#[test]
fn test_xun_kong1() {
  let solar = solar::from_ymdhms(2020, 11, 19, 0, 0, 0);
  let lunar = solar.get_lunar();
  assert_eq!("辰巳", lunar.get_year_xun_kong());
  assert_eq!("午未", lunar.get_month_xun_kong());
  assert_eq!("戌亥", lunar.get_day_xun_kong());
}

#[test]
fn test_xun_and_kong() {
  let solar = solar::from_ymd(2022, 5, 24);
  let lunar = solar.get_lunar();
  assert_eq!("甲戌", lunar.get_day_xun());
  assert_eq!("申酉", lunar.get_day_xun_kong());
}

#[test]
fn test_ba_zi_day_xun_kong() {
  let solar = solar::from_ymdhms(1990, 12, 23, 8, 37, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  assert_eq!("子丑", eight_char.get_day_xun_kong());
}