use crate::{lunar::{LunarRefHelper, self}, solar::{self, SolarRefHelper}};

#[test]
fn jie_qi_test7() {
  let lunar = lunar::from_ymd(2012, 9, 1);
  assert_eq!("2012-09-07 13:29:01", lunar.get_jie_qi_table()["白露"].to_ymdhms());
}

#[test]
fn jie_qi_test8() {
  let lunar = lunar::from_ymd(2050, 12, 1);
  assert_eq!("2050-12-07 06:41:13", lunar.get_jie_qi_table()["DA_XUE"].to_ymdhms());
}

#[test]
fn jie_qi_test1() {
  let solar = solar::from_ymd(2021, 12, 21);
  let lunar = solar.get_lunar();
  assert_eq!("冬至", lunar.get_jie_qi());
  assert_eq!("", lunar.get_jie());
  assert_eq!("冬至", lunar.get_qi());
}

#[test]
fn jie_qi_test2() {
  let lunar = lunar::from_ymd(2023, 6, 1);
  assert_eq!("2022-12-22 05:48:11", lunar.get_jie_qi_table()["冬至"].to_ymdhms())
}