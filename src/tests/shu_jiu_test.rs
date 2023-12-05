use crate::{solar::{SolarRefHelper, self}, lunar::LunarRefHelper, shu_jiu::ShuJiuRefHelper};


#[test]
fn shu_jiu_test1() {
  let solar = solar::from_ymd(2020, 12, 21);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu().unwrap();
  assert_eq!("一九", shu_jiu.to_string());
  assert_eq!("一九第1天", shu_jiu.to_full_string())
}

#[test]    
fn shu_jiu_test2() {
  let solar = solar::from_ymd(2020, 12, 22);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu().unwrap();
  assert_eq!("一九", shu_jiu.to_string());
  assert_eq!("一九第2天", shu_jiu.to_full_string())
}

#[test]    
fn shu_jiu_test3() {
  let solar = solar::from_ymd(2020, 1, 7);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu().unwrap();
  assert_eq!("二九", shu_jiu.to_string());
  assert_eq!("二九第8天", shu_jiu.to_full_string())
}

#[test]    
fn shu_jiu_test4() {
  let solar = solar::from_ymd(2021, 1, 6);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu().unwrap();
  assert_eq!("二九", shu_jiu.to_string());
  assert_eq!("二九第8天", shu_jiu.to_full_string())
}

#[test]    
fn shu_jiu_test5() {
  let solar = solar::from_ymd(2021, 1, 8);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu().unwrap();
  assert_eq!("三九", shu_jiu.to_string());
  assert_eq!("三九第1天", shu_jiu.to_full_string())
}

#[test]    
fn shu_jiu_test6() {
  let solar = solar::from_ymd(2021, 3, 5);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu().unwrap();
  assert_eq!("九九", shu_jiu.to_string());
  assert_eq!("九九第3天", shu_jiu.to_full_string())
}

#[test]    
fn shu_jiu_test7() {
  let solar = solar::from_ymd(2021, 7, 5);
  let lunar = solar.get_lunar();
  let shu_jiu = lunar.get_shu_jiu();
  assert!(shu_jiu.is_some() == false);
}
