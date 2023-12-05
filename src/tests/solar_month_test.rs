use crate::solar_month::{SolarMonth, SolarMonthRefHelper};

#[test]
fn solar_month_test() {
  let month = SolarMonth::from_ym(2019, 5);
  assert_eq!("2019-5", month.to_string());
  assert_eq!("2019年5月", month.to_full_string());
  assert_eq!("2019-6", month.clone().next(1).to_string());
  assert_eq!("2019年6月", month.next(1).to_full_string());
}