use crate::{solar::{SolarRefHelper, self}, lunar::{LunarRefHelper, self}, nine_star::{NineStarRefHelper}};


#[test]
fn nine_star_test1() {
  let lunar = solar::from_ymd(1985, 2, 19).get_lunar();
  assert_eq!("六", lunar.get_year_nine_star(None).get_number());
}

#[test]    
fn nine_star_test23() {
  let lunar = lunar::from_ymd(2022, 1, 1);
  assert_eq!("六白金开阳", lunar.get_year_nine_star(None).to_string());
}

#[test]    
fn nine_star_test24() {
  let lunar = lunar::from_ymd(2033, 1, 1);
  assert_eq!("四绿木天权", lunar.get_year_nine_star(None).to_string());
}
