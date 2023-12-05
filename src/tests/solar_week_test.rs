use crate::solar_month::{SolarMonth, SolarMonthRefHelper};

#[test]
fn test() {
  let month = SolarMonth::from_ym(2022, 12);
  assert_eq!(5, month.get_weeks(0).len());
}