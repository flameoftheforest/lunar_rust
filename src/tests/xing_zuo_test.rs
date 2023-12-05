use crate::solar::{SolarRefHelper, self};

#[test]
fn test1() {
  let solar = solar::from_ymd(2020, 3, 21);
  assert_eq!("白羊", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 4, 19);
  assert_eq!("白羊", solar.get_xing_zuo());
}

#[test]
fn test2() {
  let solar = solar::from_ymd(2020, 4, 20);
  assert_eq!("金牛", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 5, 20);
  assert_eq!("金牛", solar.get_xing_zuo());
}

#[test]
fn test3() {
  let solar = solar::from_ymd(2020, 5, 21);
  assert_eq!("双子", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 6, 21);
  assert_eq!("双子", solar.get_xing_zuo());
}

#[test]
fn test4() {
  let solar = solar::from_ymd(2020, 6, 22);
  assert_eq!("巨蟹", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 7, 22);
  assert_eq!("巨蟹", solar.get_xing_zuo());
}

#[test]
fn test5() {
  let solar = solar::from_ymd(2020, 7, 23);
  assert_eq!("狮子", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 8, 22);
  assert_eq!("狮子", solar.get_xing_zuo());
}

#[test]
fn test6() {
  let solar = solar::from_ymd(2020, 8, 23);
  assert_eq!("处女", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 9, 22);
  assert_eq!("处女", solar.get_xing_zuo());
}

#[test]
fn test7() {
  let solar = solar::from_ymd(2020, 9, 23);
  assert_eq!("天秤", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 10, 23);
  assert_eq!("天秤", solar.get_xing_zuo());
}

#[test]
fn test8() {
  let solar = solar::from_ymd(2020, 10, 24);
  assert_eq!("天蝎", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 11, 22);
  assert_eq!("天蝎", solar.get_xing_zuo());
}

#[test]
fn test9() {
  let solar = solar::from_ymd(2020, 11, 23);
  assert_eq!("射手", solar.get_xing_zuo());
  let solar = solar::from_ymd(2020, 12, 21);
  assert_eq!("射手", solar.get_xing_zuo());
}

#[test]
fn test10() {
  let solar = solar::from_ymd(2020, 12, 22);
  assert_eq!("摩羯", solar.get_xing_zuo());
  let solar = solar::from_ymd(2021, 1, 19);
  assert_eq!("摩羯", solar.get_xing_zuo());
}

#[test]
fn test11() {
  let solar = solar::from_ymd(2021, 1, 20);
  assert_eq!("水瓶", solar.get_xing_zuo());
  let solar = solar::from_ymd(2021, 2, 18);
  assert_eq!("水瓶", solar.get_xing_zuo());
}

#[test]
fn test12() {
  let solar = solar::from_ymd(2021, 2, 19);
  assert_eq!("双鱼", solar.get_xing_zuo());
  let solar = solar::from_ymd(2021, 3, 20);
  assert_eq!("双鱼", solar.get_xing_zuo());
}
