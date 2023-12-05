use crate::{solar_week::{SolarWeek, SolarWeekRefHelper}, solar::{SolarRefHelper, self}, util::solar_util::{SolarUtilRefHelper, self}};


#[test]
fn test() {
  // # 一周的开始从星期一开始计
  let start = 1;
  let week = SolarWeek::from_ymd(2019, 5, 1, start);
  assert_eq!("2019-5-1", week.to_string());
  assert_eq!("2019年5月第1周", week.to_full_string());
  // # 当月共几周
  assert_eq!(5, solar_util::get().get_weeks_of_month(week.get_year(), week.get_month(), start));
  // # 当周第一天
  assert_eq!("2019-04-29", week.get_first_day().to_string());
  // # 当周第一天（本月）
  assert_eq!("2019-05-01", week.get_first_day_in_month().unwrap().to_string());
}

#[test]    
fn test1() {
  // # 一周的开始从星期日开始计
  let start = 0;
  let week = SolarWeek::from_ymd(2019, 5, 1, start);
  assert_eq!("2019-5-1", week.to_string());
  assert_eq!("2019年5月第1周", week.to_full_string());
  // # 当月共几周
  assert_eq!(5, solar_util::get().get_weeks_of_month(week.get_year(), week.get_month(), start));
  // # 当周第一天
  assert_eq!("2019-04-28", week.get_first_day().to_string());
  // # 当周第一天（本月）
  assert_eq!("2019-05-01", week.get_first_day_in_month().unwrap().to_string());
}

#[test]    
fn test2() {
  let week = SolarWeek::from_ymd(2022, 5, 1, 0);
  assert_eq!(1, week.get_index());
}

#[test]    
fn test3() {
  let week = SolarWeek::from_ymd(2022, 5, 7, 0);
  assert_eq!(1, week.get_index());
}

#[test]    
fn test4() {
  let week = SolarWeek::from_ymd(2022, 5, 8, 0);
  assert_eq!(2, week.get_index());
}

#[test]    
fn test5() {
  let week = SolarWeek::from_ymd(2022, 5, 1, 1);
  assert_eq!(1, week.get_index());
}

#[test]    
fn test6() {
  let week = SolarWeek::from_ymd(2022, 5, 2, 1);
  assert_eq!(2, week.get_index());
}

#[test]    
fn test7() {
  let week = SolarWeek::from_ymd(2022, 5, 8, 1);
  assert_eq!(2, week.get_index());
}

#[test]    
fn test8() {
  let week = SolarWeek::from_ymd(2021, 11, 1, 0);
  assert_eq!(1, week.get_index());
}

#[test]    
fn test9() {
  let week = SolarWeek::from_ymd(2021, 11, 1, 1);
  assert_eq!(1, week.get_index());
}

#[test]    
fn test10() {
  let week = SolarWeek::from_ymd(2021, 5, 2, 2);
  assert_eq!(1, week.get_index());
}

#[test]    
fn test11() {
  let week = SolarWeek::from_ymd(2021, 5, 4, 2);
  assert_eq!(2, week.get_index());
}

#[test]    
fn test12() {
  let week = SolarWeek::from_ymd(2022, 3, 6, 0);
  assert_eq!(11, week.get_index_in_year());
}

#[test]    
fn test13() {
  assert_eq!(1, solar::from_ymd(1582, 10, 1).get_week());
}

#[test]    
fn test14() {
  assert_eq!(5, solar::from_ymd(1582, 10, 15).get_week());
}

#[test]    
fn test15() {
  assert_eq!(0, solar::from_ymd(1129, 11, 17).get_week());
}

#[test]    
fn test16() {
  assert_eq!(5, solar::from_ymd(1129, 11, 1).get_week());
}

#[test]    
fn test17() {
  assert_eq!(4, solar::from_ymd(8, 11, 1).get_week());
}

#[test]    
fn test18() {
  assert_eq!(0, solar::from_ymd(1582, 9, 30).get_week());
}

#[test]    
fn test19() {
  assert_eq!(1, solar::from_ymd(1582, 1, 1).get_week());
}

#[test]    
fn test20() {
  assert_eq!(6, solar::from_ymd(1500, 2, 29).get_week());
}

#[test]    
fn test21() {
  assert_eq!(3, solar::from_ymd(9865, 7, 26).get_week());
}

#[test]    
fn test22() {
  assert_eq!(6, solar::from_ymd(1961, 9, 30).get_week());
  assert_eq!(6, solar::from_ymdhms(1961, 9, 30, 0, 0, 0).get_week());
  assert_eq!(6, solar::from_ymdhms(1961, 9, 30, 23, 59, 59).get_week());
}

#[test]    
fn test23() {
  assert_eq!(3, solar::from_ymd(2021, 9, 15).get_week());
  assert_eq!(3, solar::from_ymdhms(2021, 9, 15, 0, 0, 0).get_week());
  assert_eq!(3, solar::from_ymdhms(2021, 9, 15, 23, 59, 59).get_week());
}
