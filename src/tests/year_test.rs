use crate::lunar_year::{LunarYear, LunarYearRefHelper};



#[test]
fn test1() {
  let year = LunarYear::from_lunar_year(2017);
  assert_eq!("二龙治水", year.get_zhi_shui());
  assert_eq!("二人分饼", year.get_fen_bing());
}

#[test]    
fn test2() {
  let year = LunarYear::from_lunar_year(2018);
  assert_eq!("二龙治水", year.get_zhi_shui());
  assert_eq!("八人分饼", year.get_fen_bing());
}

#[test]    
fn test3() {
  let year = LunarYear::from_lunar_year(5);
  assert_eq!("三龙治水", year.get_zhi_shui());
  assert_eq!("一人分饼", year.get_fen_bing());
}

#[test]    
fn test4() {
  let year = LunarYear::from_lunar_year(2021);
  assert_eq!("十一牛耕田", year.get_geng_tian());
}

#[test]    
fn test5() {
  let year = LunarYear::from_lunar_year(2018);
  assert_eq!("三日得金", year.get_de_jin());
}

#[test]    
fn test6() {
  let year = LunarYear::from_lunar_year(1864);
  assert_eq!("上元", year.get_yuan());
}

#[test]    
fn test7() {
  let year = LunarYear::from_lunar_year(1923);
  assert_eq!("上元", year.get_yuan());
}

#[test]    
fn test8() {
  let year = LunarYear::from_lunar_year(1924);
  assert_eq!("中元", year.get_yuan());
}

#[test]    
fn test9() {
  let year = LunarYear::from_lunar_year(1983);
  assert_eq!("中元", year.get_yuan());
}

#[test]    
fn test10() {
  let year = LunarYear::from_lunar_year(1984);
  assert_eq!("下元", year.get_yuan());
}

#[test]    
fn test11() {
  let year = LunarYear::from_lunar_year(2043);
  assert_eq!("下元", year.get_yuan());
}

#[test]    
fn test12() {
  let year = LunarYear::from_lunar_year(1864);
  assert_eq!("一运", year.get_yun());
}

#[test]    
fn test13() {
  let year = LunarYear::from_lunar_year(1883);
  assert_eq!("一运", year.get_yun());
}

#[test]    
fn test14() {
  let year = LunarYear::from_lunar_year(1884);
  assert_eq!("二运", year.get_yun());
}

#[test]    
fn test15() {
  let year = LunarYear::from_lunar_year(1903);
  assert_eq!("二运", year.get_yun());
}

#[test]    
fn test16() {
  let year = LunarYear::from_lunar_year(1904);
  assert_eq!("三运", year.get_yun());
}

#[test]    
fn test17() {
  let year = LunarYear::from_lunar_year(1923);
  assert_eq!("三运", year.get_yun());
}

#[test]    
fn test18() {
  let year = LunarYear::from_lunar_year(2004);
  assert_eq!("八运", year.get_yun());
}

#[test]    
fn test19() {
  let year = LunarYear::from_lunar_year(2023);
  assert_eq!(384, year.get_day_count());
}

#[test]    
fn test20() {
  let year = LunarYear::from_lunar_year(1517);
  assert_eq!(384, year.get_day_count());
}

#[test]    
fn test21() {
  let year = LunarYear::from_lunar_year(1518);
  assert_eq!(355, year.get_day_count());
}

#[test]    
fn test22() {
  let year = LunarYear::from_lunar_year(2021);
  assert_eq!(354, year.get_day_count());
}
