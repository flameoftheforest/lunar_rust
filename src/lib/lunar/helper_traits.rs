use super::Lunar;
use crate::{
  eight_char::EightCharRef, foto::FotoRef, fu::FuRef, jie_qi::JieQiRef,
  lunar_time::LunarTimeRef, nine_star::NineStarRef, shu_jiu::ShuJiuRef,
  solar::SolarRef, tao::TaoRef,
};
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
pub type LunarRef = Arc<Mutex<Lunar>>;

pub trait LunarRefHelper {
  fn get_year(&self) -> i64;
  fn get_month(&self) -> i64;
  fn get_day(&self) -> i64;
  fn get_hour(&self) -> i64;
  fn get_minute(&self) -> i64;
  fn get_second(&self) -> i64;
  fn get_solar(&self) -> SolarRef;
  fn get_year_gan(&self) -> String;
  fn get_year_gan_by_li_chun(&self) -> String;
  fn get_year_gan_exact(&self) -> String;
  fn get_year_zhi(&self) -> String;
  fn get_year_zhi_by_li_chun(&self) -> String;
  fn get_year_zhi_exact(&self) -> String;
  fn get_year_in_gan_zhi(&self) -> String;
  fn get_year_in_gan_zhi_by_li_chun(&self) -> String;
  fn get_year_in_gan_zhi_exact(&self) -> String;
  fn get_month_gan(&self) -> String;
  fn get_month_gan_exact(&self) -> String;
  fn get_month_zhi(&self) -> String;
  fn get_month_zhi_exact(&self) -> String;
  fn get_month_in_gan_zhi(&self) -> String;
  fn get_month_in_gan_zhi_exact(&self) -> String;
  fn get_day_gan(&self) -> String;
  fn get_day_gan_exact(&self) -> String;
  fn get_day_gan_exact2(&self) -> String;
  fn get_day_zhi(&self) -> String;
  fn get_day_zhi_exact(&self) -> String;
  fn get_day_zhi_exact2(&self) -> String;
  fn get_day_in_gan_zhi(&self) -> String;
  fn get_day_in_gan_zhi_exact(&self) -> String;
  fn get_day_in_gan_zhi_exact2(&self) -> String;
  fn get_time_gan(&self) -> String;
  fn get_time_zhi(&self) -> String;
  fn get_time_in_gan_zhi(&self) -> String;
  fn get_year_sheng_xiao(&self) -> String;
  fn get_year_sheng_xiao_by_li_chun(&self) -> String;
  fn get_year_sheng_xiao_exact(&self) -> String;
  fn get_month_sheng_xiao(&self) -> String;
  fn get_month_sheng_xiao_exact(&self) -> String;
  fn get_day_sheng_xiao(&self) -> String;
  fn get_time_sheng_xiao(&self) -> String;
  fn get_year_in_chinese(&self) -> String;
  fn get_month_in_chinese(&self) -> String;
  fn get_day_in_chinese(&self) -> String;
  fn get_peng_zu_gan(&self) -> String;
  fn get_peng_zu_zhi(&self) -> String;
  fn get_position_xi(&self) -> String;
  fn get_position_xi_desc(&self) -> String;
  fn get_position_yang_gui(&self) -> String;
  fn get_position_yang_gui_desc(&self) -> String;
  fn get_position_yin_gui(&self) -> String;
  fn get_position_yin_gui_desc(&self) -> String;
  fn get_position_fu(&self) -> String;
  fn get_position_fu_desc(&self) -> String;
  fn get_position_cai(&self) -> String;
  fn get_position_cai_desc(&self) -> String;
  fn get_day_position_xi(&self) -> String;
  fn get_day_position_xi_desc(&self) -> String;
  fn get_day_position_yang_gui(&self) -> String;
  fn get_day_position_yang_gui_desc(&self) -> String;
  fn get_day_position_yin_gui(&self) -> String;
  fn get_day_position_yin_gui_desc(&self) -> String;
  fn get_day_position_fu(&self, sect: Option<i64>) -> String;
  fn get_day_position_fu_desc(&self, sect: Option<i64>) -> String;
  fn get_day_position_cai(&self) -> String;
  fn get_day_position_cai_desc(&self) -> String;
  fn get_year_position_tai_sui(&self, sect: Option<i64>) -> String;
  fn get_year_position_tai_sui_desc(&self, sect: Option<i64>)
    -> String;
  fn get_month_position_tai_sui(&self, sect: Option<i64>) -> String;
  fn get_month_position_tai_sui_desc(
    &self,
    sect: Option<i64>,
  ) -> String;
  fn get_day_position_tai_sui(&self, sect: Option<i64>) -> String;
  fn get_day_position_tai_sui_desc(&self, sect: Option<i64>) -> String;
  fn get_time_position_xi(&self) -> String;
  fn get_time_position_xi_desc(&self) -> String;
  fn get_time_position_yang_gui(&self) -> String;
  fn get_time_position_yang_gui_desc(&self) -> String;
  fn get_time_position_yin_gui(&self) -> String;
  fn get_time_position_yin_gui_desc(&self) -> String;
  fn get_time_position_fu(&self, sect: Option<i64>) -> String;
  fn get_time_position_fu_desc(&self, sect: Option<i64>) -> String;
  fn get_time_position_cai(&self) -> String;
  fn get_time_position_cai_desc(&self) -> String;
  fn get_chong(&self) -> String;
  fn get_day_chong(&self) -> String;
  fn get_time_chong(&self) -> String;
  fn get_chong_gan(&self) -> String;
  fn get_day_chong_gan(&self) -> String;
  fn get_time_chong_gan(&self) -> String;
  fn get_chong_gan_tie(&self) -> String;
  fn get_day_chong_gan_tie(&self) -> String;
  fn get_time_chong_gan_tie(&self) -> String;
  fn get_chong_sheng_xiao(&self) -> String;
  fn get_day_chong_sheng_xiao(&self) -> String;
  fn get_time_chong_sheng_xiao(&self) -> String;
  fn get_chong_desc(&self) -> String;
  fn get_day_chong_desc(&self) -> String;
  fn get_time_chong_desc(&self) -> String;
  fn get_sha(&self) -> String;
  fn get_day_sha(&self) -> String;
  fn get_time_sha(&self) -> String;
  fn get_year_na_yin(&self) -> String;
  fn get_month_na_yin(&self) -> String;
  fn get_day_na_yin(&self) -> String;
  fn get_time_na_yin(&self) -> String;
  fn get_season(&self) -> String;
  fn get_jie(&self) -> String;
  fn get_qi(&self) -> String;
  fn get_week(&self) -> i64;
  fn get_week_in_chinese(&self) -> String;
  fn get_xiu(&self) -> String;
  fn get_xiu_luck(&self) -> String;
  fn get_xiu_song(&self) -> String;
  fn get_zheng(&self) -> String;
  fn get_animal(&self) -> String;
  fn get_gong(&self) -> String;
  fn get_shou(&self) -> String;
  fn get_festivals(&self) -> Vec<String>;
  fn get_other_festivals(&self) -> Vec<String>;
  fn get_eight_char(&self) -> EightCharRef;
  fn get_ba_zi(&self) -> Vec<String>;
  fn get_ba_zi_wu_xing(&self) -> Vec<String>;
  fn get_ba_zi_na_yin(&self) -> Vec<String>;
  fn get_ba_zi_shi_shen_gan(&self) -> Vec<String>;
  fn get_ba_zi_shi_shen_zhi(&self) -> Vec<Vec<String>>;
  fn get_ba_zi_shi_shen_year_zhi(&self) -> Vec<String>;
  fn get_ba_zi_shi_shen_month_zhi(&self) -> Vec<String>;
  fn get_ba_zi_shi_shen_day_zhi(&self) -> Vec<String>;
  fn get_ba_zi_shi_shen_time_zhi(&self) -> Vec<String>;
  fn get_zhi_xing(&self) -> String;
  fn get_day_tian_shen(&self) -> String;
  fn get_time_tian_shen(&self) -> String;
  fn get_day_tian_shen_type(&self) -> String;
  fn get_time_tian_shen_type(&self) -> String;
  fn get_day_tian_shen_luck(&self) -> String;
  fn get_time_tian_shen_luck(&self) -> String;
  fn get_day_position_tai(&self) -> String;
  fn get_month_position_tai(&self) -> String;
  fn get_day_yi(&self, sect: Option<i64>) -> Vec<String>;
  fn get_day_ji(&self, sect: Option<i64>) -> Vec<String>;
  fn get_time_yi(&self) -> Vec<String>;
  fn get_time_ji(&self) -> Vec<String>;
  fn get_day_ji_shen(&self) -> Vec<String>;
  fn get_day_xiong_sha(&self) -> Vec<String>;
  fn get_yue_xiang(&self) -> String;
  fn get_year_nine_star(&self, sect: Option<i64>) -> NineStarRef;
  fn get_month_nine_star(&self, sect: Option<i64>) -> NineStarRef;
  fn get_day_nine_star(&self) -> NineStarRef;
  fn get_time_nine_star(&self) -> NineStarRef;
  fn get_jie_qi_table(&self) -> HashMap<String, SolarRef>;
  fn get_jie_qi_list(&self) -> Vec<String>;
  fn get_time_gan_index(&self) -> i64;
  fn get_time_zhi_index(&self) -> i64;
  fn get_day_gan_index(&self) -> i64;
  fn get_day_zhi_index(&self) -> i64;
  fn get_day_gan_index_exact(&self) -> i64;
  fn get_day_gan_index_exact2(&self) -> i64;
  fn get_day_zhi_index_exact(&self) -> i64;
  fn get_day_zhi_index_exact2(&self) -> i64;
  fn get_month_gan_index(&self) -> i64;
  fn get_month_zhi_index(&self) -> i64;
  fn get_month_gan_index_exact(&self) -> i64;
  fn get_month_zhi_index_exact(&self) -> i64;
  fn get_year_gan_index(&self) -> i64;
  fn get_year_zhi_index(&self) -> i64;
  fn get_year_gan_index_by_li_chun(&self) -> i64;
  fn get_year_zhi_index_by_li_chun(&self) -> i64;
  fn get_year_gan_index_exact(&self) -> i64;
  fn get_year_zhi_index_exact(&self) -> i64;
  fn get_next_jie(&self, whole_day: Option<bool>) -> Option<JieQiRef>;
  fn get_prev_jie(&self, whole_day: Option<bool>) -> Option<JieQiRef>;
  fn get_next_qi(&self, whole_day: Option<bool>) -> Option<JieQiRef>;
  fn get_prev_qi(&self, whole_day: Option<bool>) -> Option<JieQiRef>;
  fn get_next_jie_qi(
    &self,
    whole_day: Option<bool>,
  ) -> Option<JieQiRef>;
  fn get_prev_jie_qi(
    &self,
    whole_day: Option<bool>,
  ) -> Option<JieQiRef>;
  fn get_jie_qi(&self) -> String;
  fn get_current_jie_qi(&self) -> Option<JieQiRef>;
  fn get_current_jie(&self) -> Option<JieQiRef>;
  fn get_current_qi(&self) -> Option<JieQiRef>;
  fn next(&self, days: i64) -> LunarRef;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
  fn get_year_xun(&self) -> String;
  fn get_year_xun_by_li_chun(&self) -> String;
  fn get_year_xun_exact(&self) -> String;
  fn get_year_xun_kong(&self) -> String;
  fn get_year_xun_kong_by_li_chun(&self) -> String;
  fn get_year_xun_kong_exact(&self) -> String;
  fn get_month_xun(&self) -> String;
  fn get_month_xun_exact(&self) -> String;
  fn get_month_xun_kong(&self) -> String;
  fn get_month_xun_kong_exact(&self) -> String;
  fn get_day_xun(&self) -> String;
  fn get_day_xun_exact(&self) -> String;
  fn get_day_xun_exact2(&self) -> String;
  fn get_day_xun_kong(&self) -> String;
  fn get_day_xun_kong_exact(&self) -> String;
  fn get_day_xun_kong_exact2(&self) -> String;
  fn get_time_xun(&self) -> String;
  fn get_time_xun_kong(&self) -> String;
  fn get_shu_jiu(&self) -> Option<ShuJiuRef>;
  fn get_fu(&self) -> Option<FuRef>;
  fn get_liu_yao(&self) -> String;
  fn get_wu_hou(&self) -> String;
  fn get_hou(&self) -> String;
  fn get_day_lu(&self) -> String;
  fn get_time(&self) -> LunarTimeRef;
  fn get_times(&self) -> Vec<LunarTimeRef>;
  fn get_foto(&self) -> FotoRef;
  fn get_tao(&self) -> TaoRef;
}

pub trait __LunarRefHelper {
  fn __get_year_nine_star(&self, year_in_gan_zhi: &str) -> NineStarRef;
  fn __get_near_jie_qi(
    &self,
    forward: bool,
    conditions: &Vec<String>,
    whole_day: bool,
  ) -> Option<JieQiRef>;
  fn __get_jie_qi_map(&self) -> HashMap<String, SolarRef>;
}
