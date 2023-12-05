use crate::{tao::{TaoRefHelper, self}, lunar::{LunarRefHelper, self}, tao_festival::TaoFestivalRefHelper};

#[test]
fn test() {
  let tao = tao::from_lunar(lunar::from_ymd_hms(2021, 10, 17, 18, 0, 0));
  assert_eq!("四七一八年十月十七", tao.to_string());
  assert_eq!("道歷四七一八年，天运辛丑年，己亥月，癸酉日。十月十七日，酉時。", tao.to_full_string());
}

#[test]
fn test1() {
  let tao = tao::from_ymd(4718, 10, 18);
  assert_eq!("地母娘娘圣诞", tao.get_festivals()[0].to_string());

  let tao = lunar::from_ymd(2021, 10, 18).get_tao();
  assert_eq!("四时会", tao.get_festivals()[1].to_string());
}