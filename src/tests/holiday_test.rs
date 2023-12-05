use crate::{util::holiday_util::{self, HolidayUtilRefHelper}, holiday::HolidayRefHelper};

#[test]
fn holiday() {
  let holiday = holiday_util::get().get_holiday(2010, Some(1), Some(1)).unwrap();
  assert_eq!("元旦节", holiday.get_name());

  let mut h = holiday_util::get();
  h.fix([].to_vec(), "20100101~000000000000000000000000000");
  let holiday = holiday_util::get().get_holiday(2010, Some(1), Some(1));
  assert!(holiday.is_none());
}