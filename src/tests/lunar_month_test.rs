use crate::lunar_month::{self, LunarMonthRefHelper};

#[test]
fn lunar_month_test0() {
  let month = lunar_month::from_ym(2023, 1);
  assert_eq!(1, month.get_index());
  assert_eq!("甲寅", month.get_gan_zhi());
}

#[test]    
fn lunar_month_test1() {
  let month = lunar_month::from_ym(2023, -2);
  assert_eq!(3, month.get_index());
  assert_eq!("丙辰", month.get_gan_zhi());
}

#[test]    
fn lunar_month_test2() {
  let month = lunar_month::from_ym(2023, 3);
  assert_eq!(4, month.get_index());
  assert_eq!("丁巳", month.get_gan_zhi());
}

#[test]    
fn lunar_month_test3() {
  let month = lunar_month::from_ym(2024, 1);
  assert_eq!(1, month.get_index());
  assert_eq!("丙寅", month.get_gan_zhi());
}

#[test]    
fn lunar_month_test4() {
  let month = lunar_month::from_ym(2023, 12);
  assert_eq!(13, month.get_index());
  assert_eq!("丙寅", month.get_gan_zhi());
}

#[test]    
fn lunar_month_test5() {
  let month = lunar_month::from_ym(2022, 1);
  assert_eq!(1, month.get_index());
  assert_eq!("壬寅", month.get_gan_zhi());
}