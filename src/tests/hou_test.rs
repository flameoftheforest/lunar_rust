use crate::{solar::{self, SolarRefHelper}, lunar::LunarRefHelper};

#[test]
fn hou_test1() {
  let solar = solar::from_ymd(2021, 12, 22);
  assert_eq!("冬至 初候", &solar.get_lunar().get_hou());
}

#[test]
fn hou_test2() {
  let solar = solar::from_ymd(2021, 12, 26);
  assert_eq!("冬至 二候", &solar.get_lunar().get_hou());
}

#[test]
fn hou_test3() {
  let solar = solar::from_ymd(2021, 12, 31);
  assert_eq!("冬至 三候", &solar.get_lunar().get_hou());

}

#[test]    
fn hou_test4() {
  let solar = solar::from_ymd(2022, 1, 5);
  assert_eq!("小寒 初候", &solar.get_lunar().get_hou());

}
