use crate::solar_year::{SolarYearRefHelper, self};

#[test]
fn test() {
  let from_year = solar_year::from_year(2019);
  let year = from_year;
  assert_eq!("2019", year.to_string());
  assert_eq!("2019年", year.to_full_string());

  assert_eq!("2020", year.next(1).to_string());
  assert_eq!("2020年", year.next(1).to_full_string());

}