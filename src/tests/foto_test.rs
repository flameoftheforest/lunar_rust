use crate::{foto::{FotoRefHelper, self}, lunar::{
  self}};


#[test]
fn foto_test() {
  let foto = foto::from_lunar(lunar::from_ymd(2021, 10, 14));
  assert_eq!("二五六五年十月十四 (三元降) (四天王巡行)", foto.to_full_string());
}

#[test]    
fn foto_test1() {
  let foto = foto::from_lunar(lunar::from_ymd(2020, 4, 13));
  assert_eq!("氐", &foto.get_xiu());
  assert_eq!("土", &foto.get_zheng());
  assert_eq!("貉", &foto.get_animal());
  assert_eq!("东", &foto.get_gong());
  assert_eq!("青龙", &foto.get_shou());
}

#[test]    
fn foto_test2() {
  let foto = foto::from_lunar(lunar::from_ymd(2021, 3, 16));
  let expected = ["准提菩萨圣诞".to_string()].to_vec();
  assert_eq!(expected, foto.get_other_festivals());
}