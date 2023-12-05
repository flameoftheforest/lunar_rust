use crate::{solar::{SolarRefHelper, self}, lunar::LunarRefHelper};



#[test]
fn test1() {
  let solar = solar::from_ymd(2020, 4, 23);
  let lunar = solar.get_lunar();
  assert_eq!("萍始生", lunar.get_wu_hou());
}

#[test]    
fn test2() {
  let solar = solar::from_ymd(2021, 1, 15);
  let lunar = solar.get_lunar();
  assert_eq!("雉始雊", lunar.get_wu_hou());
}

#[test]    
fn test3() {
  let solar = solar::from_ymd(2017, 1, 5);
  let lunar = solar.get_lunar();
  assert_eq!("雁北乡", lunar.get_wu_hou());
}

#[test]    
fn test4() {
  let solar = solar::from_ymd(2020, 4, 10);
  let lunar = solar.get_lunar();
  assert_eq!("田鼠化为鴽", lunar.get_wu_hou());
}

#[test]    
fn test5() {
  let solar = solar::from_ymd(2020, 6, 11);
  let lunar = solar.get_lunar();
  assert_eq!("鵙始鸣", lunar.get_wu_hou());
}

#[test]    
fn test6() {
  let solar = solar::from_ymd(2020, 6, 1);
  let lunar = solar.get_lunar();
  assert_eq!("麦秋至", lunar.get_wu_hou());
}

#[test]    
fn test7() {
  let solar = solar::from_ymd(2020, 12, 8);
  let lunar = solar.get_lunar();
  assert_eq!("鹖鴠不鸣", lunar.get_wu_hou());
}

#[test]    
fn test8() {
  let solar = solar::from_ymd(2020, 12, 11);
  let lunar = solar.get_lunar();
  assert_eq!("鹖鴠不鸣", lunar.get_wu_hou());
}

#[test]    
fn test9() {
  let solar = solar::from_ymd(1982, 12, 22);
  let lunar = solar.get_lunar();
  assert_eq!("蚯蚓结", lunar.get_wu_hou());
}

#[test]    
fn test10() {
  let solar = solar::from_ymd(2021, 12, 21);
  let lunar = solar.get_lunar();
  assert_eq!("冬至 初候", lunar.get_hou());
}

#[test]    
fn test11() {
  let solar = solar::from_ymd(2021, 12, 26);
  let lunar = solar.get_lunar();
  assert_eq!("冬至 二候", lunar.get_hou());
}

#[test]    
fn test12() {
  let solar = solar::from_ymd(2021, 12, 31);
  let lunar = solar.get_lunar();
  assert_eq!("冬至 三候", lunar.get_hou());
}

#[test]    
fn test13() {
  let solar = solar::from_ymd(2022, 1, 5);
  let lunar = solar.get_lunar();
  assert_eq!("小寒 初候", lunar.get_hou());
}

#[test]    
fn test15() {
  let solar = solar::from_ymd(2022, 8, 22);
  let lunar = solar.get_lunar();
  assert_eq!("寒蝉鸣", lunar.get_wu_hou());
}

#[test]    
fn test16() {
  let solar = solar::from_ymd(2022, 8, 23);
  let lunar = solar.get_lunar();
  assert_eq!("鹰乃祭鸟", lunar.get_wu_hou());
}
