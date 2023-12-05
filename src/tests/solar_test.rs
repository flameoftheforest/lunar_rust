use crate::{solar::{SolarRefHelper, self}, lunar::LunarRefHelper, util::solar_util::{SolarUtilRefHelper, self}};


#[test]
fn test() {
  let solar = solar::from_ymd(2019, 5, 1);
  assert_eq!("2019-05-01", solar.to_string());
  assert_eq!("2019-05-01 00:00:00 星期三 (劳动节) 金牛座", solar.to_full_string());
  assert_eq!("二〇一九年三月廿七", solar.get_lunar().to_string());
  assert_eq!("二〇一九年三月廿七 己亥(猪)年 戊辰(龙)月 戊戌(狗)日 子(鼠)时 纳音[平地木 大林木 平地木 桑柘木] 星期三 西方白虎 星宿[参水猿](吉) 彭祖百忌[戊不受田田主不祥 戌不吃犬作怪上床] 喜神方位[巽](东南) 阳贵神方位[艮](东北) 阴贵神方位[坤](西南) 福神方位[艮](东北) 财神方位[坎](正北) 冲[(壬辰)龙] 煞[北]", solar.get_lunar().to_full_string());

}


#[test]
fn test1() {
  let solar = solar::from_ymdhms(2020, 5, 24, 13, 0, 0);
  assert_eq!("二〇二〇年闰四月初二", solar.get_lunar().to_string());
}


#[test]
fn test2() {
  let solar = solar::from_ymd(11, 1, 1);
  assert_eq!("一〇年腊月初八", solar.get_lunar().to_string());
}


#[test]
fn test3() {
  let solar = solar::from_ymd(11, 3, 1);
  assert_eq!("一一年二月初八", solar.get_lunar().to_string());
}


#[test]
fn test4() {
  let solar = solar::from_ymd(26, 4, 13);
  assert_eq!("二六年三月初八", solar.get_lunar().to_string());
}


#[test]
fn test6() {
  let solar = solar::from_ymd(1, 1, 1);
  assert_eq!("0001-01-01", solar.to_string());
}


#[test]
fn test5() {
  let date = solar::from_ymd(2020, 1, 23);
  assert_eq!("2020-01-24", date.next(1, None).to_string());
  // # 仅工作日，跨越春节假期
  assert_eq!("2020-02-03", date.next(1, Some(true)).to_string());
  
  let date = solar::from_ymd(2020, 2, 3);
  assert_eq!("2020-01-31", date.next(-3, None).to_string());
  // # 仅工作日，跨越春节假期
  assert_eq!("2020-01-21", date.next(-3, Some(true)).to_string());
  
  let date = solar::from_ymd(2020, 2, 9);
  assert_eq!("2020-02-15", date.next(6, None).to_string());
  // # 仅工作日，跨越周末
  assert_eq!("2020-02-17", date.next(6, Some(true)).to_string());
  
  let date = solar::from_ymd(2020, 1, 17);
  assert_eq!("2020-01-18", date.next(1, None).to_string());
  // # 仅工作日，周日调休按上班算
  assert_eq!("2020-01-19", date.next(1, Some(true)).to_string());
}

#[test]
fn test10() {
  assert_eq!(true, solar_util::get().is_leap_year(1500))
}


#[test]
fn test11() {
  let solar = solar::from_ymd(2022, 3, 28);
  assert_eq!("全国中小学生安全教育日", solar.get_festivals()[0])
}


#[test]
fn test12() {
  assert_eq!("壬午", solar::from_ymd(1991, 5, 12).get_lunar().get_day_in_gan_zhi());
}


#[test]
fn test13() {
  assert_eq!("1582-09-30", solar::from_ymd(1582, 10, 15).next(-5, None).to_ymd());
}


#[test]
fn test14() {
  assert_eq!("1582-10-04", solar::from_ymd(1582, 10, 15).next(-1, None).to_ymd());
}


#[test]
fn test15() {
  assert_eq!("1582-09-29", solar::from_ymd(1582, 10, 15).next(-6, None).to_ymd());
}


#[test]
fn test16() {
  assert_eq!(2, solar_util::get().get_days_between(100, 2, 28, 100, 3, 1));
}


#[test]
fn test17() {
  assert_eq!(59, solar_util::get().get_days_in_year(100, 2, 28));
}


#[test]
fn test18() {
  assert_eq!(61, solar_util::get().get_days_in_year(100, 3, 1));
}


#[test]
fn test19() {
  assert_eq!("2023-09-30", solar::from_ymd(2023, 8, 31).next_month(1).to_ymd());
}


#[test]
fn test20() {
  assert_eq!("2023-10-31", solar::from_ymd(2023, 8, 31).next_month(2).to_ymd());
}


#[test]
fn test21() {
  assert_eq!("2024-02-29", solar::from_ymd(2023, 8, 31).next_month(6).to_ymd());
}


#[test]
fn test22() {
  assert_eq!("2025-08-31", solar::from_ymd(2023, 8, 31).next_year(2).to_ymd());
}

