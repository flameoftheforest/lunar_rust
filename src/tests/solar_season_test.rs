use crate::solar_season::{SolarSeason, SolarSeasonRefHelper};

fn solar_season_test() {
  let season = SolarSeason::from_ym(2019, 5);
  assert_eq!("2019.2", season.to_string());
  assert_eq!("2019年2季度", season.to_full_string());
  assert_eq!("2019.3", season.next(1).to_string());
  assert_eq!("2019年3季度", season.next(1).to_full_string());
}