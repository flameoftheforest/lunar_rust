use crate::{solar::{SolarRefHelper, self}, lunar::{LunarRefHelper, self}, eight_char::{EightCharRefHelper, yun::YunRefHelper}};

#[test]
fn test() {
  let solar = solar::from_ymdhms(1981, 1, 29, 23, 37, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  let yun = eight_char.get_yun(0, None);
  assert_eq!(8, yun.get_start_year(), "起运年数");
  assert_eq!(0, yun.get_start_month(), "起运月数");
  assert_eq!(20, yun.get_start_day(), "起运天数");
  assert_eq!("1989-02-18", yun.get_start_solar().to_ymd(), "起运阳历");
}

#[test]    
fn test2() {
  let lunar = lunar::from_ymd_hms(2019, 12, 12, 11, 22, 0);
  let eight_char = lunar.get_eight_char();
  let yun = eight_char.get_yun(1, None);
  assert_eq!(0, yun.get_start_year(), "起运年数");
  assert_eq!(1, yun.get_start_month(), "起运月数");
  assert_eq!(0, yun.get_start_day(), "起运天数");
  assert_eq!("2020-02-06", yun.get_start_solar().to_ymd(), "起运阳历");
}

#[test]    
fn test3() {
  let solar = solar::from_ymdhms(2020, 1, 6, 11, 22, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  let yun = eight_char.get_yun(1, None);
  assert_eq!(0, yun.get_start_year(), "起运年数");
  assert_eq!(1, yun.get_start_month(), "起运月数");
  assert_eq!(0, yun.get_start_day(), "起运天数");
  assert_eq!("2020-02-06", yun.get_start_solar().to_ymd(), "起运阳历");
}

#[test]    
fn test4() {
  let solar = solar::from_ymdhms(2022, 3, 9, 20, 51, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  let yun = eight_char.get_yun(1, None);
  assert_eq!("2030-12-19", yun.get_start_solar().to_ymd(), "起运阳历");
}

#[test]    
fn test5() {
  let solar = solar::from_ymdhms(2022, 3, 9, 20, 51, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  let yun = eight_char.get_yun(1, Some(2));
  assert_eq!(8, yun.get_start_year(), "起运年数");
  assert_eq!(9, yun.get_start_month(), "起运月数");
  assert_eq!(2, yun.get_start_day(), "起运天数");
  assert_eq!("2030-12-12", yun.get_start_solar().to_ymd(), "起运阳历");
}

#[test]    
fn test6() {
  let solar = solar::from_ymdhms(2018, 6, 11, 9, 30, 0);
  let lunar = solar.get_lunar();
  let eight_char = lunar.get_eight_char();
  let yun = eight_char.get_yun(0, Some(2));
  assert_eq!("2020-03-21", yun.get_start_solar().to_ymd(), "起运阳历");
}
