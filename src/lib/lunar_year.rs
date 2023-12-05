use crate::{
  lunar::{Lunar, LunarRefHelper},
  lunar_month::{self, LunarMonth, LunarMonthRef, LunarMonthRefHelper},
  nine_star::NineStar,
  solar::{self, SolarRefHelper},
  util::{
    lunar_util::{
      LunarUtil, GAN, NUMBER, POSITION_CAI, POSITION_DESC, POSITION_FU,
      POSITION_FU_2, POSITION_TAI_SUI_YEAR, POSITION_XI,
      POSITION_YANG_GUI, POSITION_YIN_GUI, ZHI,
    },
    mmacro::__static_funk,
    shou_xing_util::ShouXingUtil,
  },
};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct LunarYear {
  __year: i64,
  __gan_index: i64,
  __zhi_index: i64,
  __months: Vec<LunarMonthRef>,
  __jie_qi_julian_days: Vec<f64>,
}

pub trait LunarYearRefHelper {
  fn get_year(&self) -> i64;
  fn get_gan_index(&self) -> i64;
  fn get_zhi_index(&self) -> i64;
  fn get_gan(&self) -> String;
  fn get_zhi(&self) -> String;
  fn get_gan_zhi(&self) -> String;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
  fn get_day_count(&self) -> i64;
  fn get_months_in_year(&self) -> Vec<LunarMonthRef>;
  fn get_months(&self) -> Vec<LunarMonthRef>;
  fn get_jie_qi_julian_days(&self) -> Vec<f64>;
  fn get_leap_months(&self) -> i64;
  fn get_month(&self, lunar_month: i64) -> Option<LunarMonthRef>;
  fn get_tuo_liang(&self) -> String;
  fn get_cai_zi(&self) -> String;
  fn get_geng_tian(&self) -> String;
  fn get_hua_shou(&self) -> String;
  fn get_zhi_shui(&self) -> String;
  fn get_tuo_gu(&self) -> String;
  fn get_qiang_mi(&self) -> String;
  fn get_kan_can(&self) -> String;
  fn get_gong_zhu(&self) -> String;
  fn get_jia_tian(&self) -> String;
  fn get_fen_bing(&self) -> String;
  fn get_de_jin(&self) -> String;
  fn get_ren_bing(&self) -> String;
  fn get_ren_chu(&self) -> String;
  fn get_yuan(&self) -> String;
  fn get_yun(&self) -> String;
  fn get_nine_star(&self) -> Arc<Mutex<NineStar>>;
  fn get_position_xi(&self) -> String;
  fn get_position_xi_desc(&self) -> String;
  fn get_position_yang_gui(&self) -> String;
  fn get_position_yang_gui_desc(&self) -> String;
  fn get_position_yin_gui(&self) -> String;
  fn get_position_yin_gui_desc(&self) -> String;
  fn get_position_fu(self, sect: Option<i64>) -> String;
  fn get_position_fu_desc(self, sect: Option<i64>) -> String;
  fn get_position_cai(self) -> String;
  fn get_position_cai_desc(self) -> String;
  fn get_position_tai_sui(self) -> String;
  fn get_position_tai_sui_desc(self) -> String;
  fn next(&self, n: i64) -> Arc<Mutex<LunarYear>>;
}

trait LunarYearRefPrivateHelper {
  fn __get_zao_by_gan(&self, index: i64, name: &str) -> String;
  fn __get_zao_by_zhi(&self, index: i64, name: &str) -> String;
}

impl LunarYearRefHelper for Arc<Mutex<LunarYear>> {
  fn get_year(&self) -> i64 {
    self.lock().unwrap().__year
  }

  fn get_gan_index(&self) -> i64 {
    self.lock().unwrap().__gan_index
  }

  fn get_zhi_index(&self) -> i64 {
    self.lock().unwrap().__zhi_index
  }

  fn get_gan(&self) -> String {
    GAN()[self.get_gan_index() as usize + 1].to_string()
  }

  fn get_zhi(&self) -> String {
    ZHI()[self.get_zhi_index() as usize + 1].to_string()
  }

  fn get_gan_zhi(&self) -> String {
    format!("{}{}", self.get_gan(), self.get_zhi())
  }

  fn to_string(&self) -> String {
    format!("{}", self.lock().unwrap().__year)
  }

  fn to_full_string(&self) -> String {
    format!("{}年", self.lock().unwrap().__year)
  }

  fn get_day_count(&self) -> i64 {
    let mut n = 0;
    self.get_months().iter().for_each(|m| {
      if m.get_year() == self.get_year() {
        n = n + m.get_day_count()
      }
    });
    n
  }

  fn get_months_in_year(&self) -> Vec<LunarMonthRef> {
    let mut months = vec![];
    let current_year = self.lock().unwrap().__year;
    self.lock().unwrap().__months.iter().for_each(|m| {
      if m.get_year() == current_year {
        months.push(m.clone());
      }
    });
    months
  }

  fn get_months(&self) -> Vec<LunarMonthRef> {
    self.lock().unwrap().__months.clone()
  }

  fn get_jie_qi_julian_days(&self) -> Vec<f64> {
    self.lock().unwrap().__jie_qi_julian_days.clone()
  }

  ///
  /// 获取闰月
  ///
  /// ## Returns
  /// + 闰月数字，1代表闰1月，0代表无闰月: **i64**
  ///
  fn get_leap_months(&self) -> i64 {
    let current_year = self.lock().unwrap().__year;
    for m in self.lock().unwrap().__months.iter() {
      if m.get_year() == current_year && m.is_leap() {
        return m.get_month();
      }
    }
    0
  }

  ///
  /// 获取农历月
  ///
  /// ## Arguments
  /// + lunar_month: **i64** - 闰月数字，1代表闰1月，0代表无闰月
  ///
  /// ## Returns
  /// + 农历月: Option<LunarMonth>
  ///
  fn get_month(&self, lunar_month: i64) -> Option<LunarMonthRef> {
    let current_year = self.lock().unwrap().__year;
    for m in self.lock().unwrap().__months.iter() {
      if m.get_year() == current_year && m.get_month() == lunar_month {
        return Some(m.clone());
      }
    }
    None
  }

  fn get_tuo_liang(&self) -> String {
    self.__get_zao_by_zhi(0, "几鼠偷粮")
  }

  fn get_cai_zi(&self) -> String {
    self.__get_zao_by_zhi(0, "草子几分")
  }

  ///
  /// 获取耕田（正月第一个丑日是初几，就是几牛耕田）
  ///
  /// ## Returns
  /// + 耕田，如：六牛耕田: **String**
  ///
  fn get_geng_tian(&self) -> String {
    self.__get_zao_by_zhi(1, "几牛耕田")
  }

  fn get_hua_shou(&self) -> String {
    self.__get_zao_by_zhi(3, "花收几分")
  }

  ///
  /// 获取治水（正月第一个辰日是初几，就是几龙治水）
  ///
  /// ## Returns
  /// + 治水，如：二龙治水: **String**
  ///
  fn get_zhi_shui(&self) -> String {
    self.__get_zao_by_zhi(4, "几龙治水")
  }

  fn get_tuo_gu(&self) -> String {
    self.__get_zao_by_zhi(6, "几马驮谷")
  }

  fn get_qiang_mi(&self) -> String {
    self.__get_zao_by_zhi(9, "几鸡抢米")
  }

  fn get_kan_can(&self) -> String {
    self.__get_zao_by_zhi(9, "几姑看蚕")
  }

  fn get_gong_zhu(&self) -> String {
    self.__get_zao_by_zhi(11, "几屠共猪")
  }

  fn get_jia_tian(&self) -> String {
    self.__get_zao_by_gan(0, "甲田几分")
  }

  ///
  /// 获取分饼（正月第一个丙日是初几，就是几人分饼）
  ///
  /// ## Returns
  /// + 分饼，如：六人分饼: **String**
  ///
  fn get_fen_bing(&self) -> String {
    self.__get_zao_by_gan(2, "几人分饼")
  }

  ///
  /// 获取得金（正月第一个辛日是初几，就是几日得金）
  /// + 得金，如：一日得金: **String**
  ///
  fn get_de_jin(&self) -> String {
    self.__get_zao_by_gan(7, "几日得金")
  }

  fn get_ren_bing(&self) -> String {
    let s = self.__get_zao_by_zhi(2, "几人几丙");
    self.__get_zao_by_gan(2, &s)
  }

  fn get_ren_chu(&self) -> String {
    let s = self.__get_zao_by_zhi(2, "几人几锄");
    self.__get_zao_by_gan(3, &s)
  }

  fn get_yuan(&self) -> String {
    format!(
      "{}{}",
      YUAN()[(({ self.lock().unwrap().__year } + 2696) as f64 / 60.)
        as usize
        % 3],
      "元"
    )
  }

  fn get_yun(&self) -> String {
    format!(
      "{}{}",
      YUN()[(({ self.lock().unwrap().__year } + 2696) as f64 / 20.)
        as usize
        % 9],
      "运"
    )
  }

  fn get_nine_star(&self) -> Arc<Mutex<NineStar>> {
    let index = LunarUtil::get_jia_zi_index(&self.get_gan_zhi()) + 1;
    let yuan = (({ self.lock().unwrap().__year } + 2696) as f64 / 60.)
      as i64
      % 3;
    let mut offset = (62 + yuan * 3 - index) % 9;
    if 0 == offset {
      offset = 9;
    }
    NineStar::from_index(offset as usize - 1)
  }

  fn get_position_xi(&self) -> String {
    POSITION_XI()[self.lock().unwrap().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_xi_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_xi())
      .unwrap()
      .to_string()
  }

  fn get_position_yang_gui(&self) -> String {
    POSITION_YANG_GUI()[self.lock().unwrap().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_yang_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_yang_gui())
      .unwrap()
      .to_string()
  }

  fn get_position_yin_gui(&self) -> String {
    POSITION_YIN_GUI()[self.lock().unwrap().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_yin_gui_desc(&self) -> String {
    POSITION_DESC()
      .get(&self.get_position_yin_gui())
      .unwrap()
      .to_string()
  }

  fn get_position_fu(self, sect: Option<i64>) -> String {
    let position_fu = match sect.unwrap_or(2) == 1 {
      true => POSITION_FU.clone(),
      _ => POSITION_FU_2.clone(),
    };
    position_fu()[self.lock().unwrap().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_fu_desc(self, sect: Option<i64>) -> String {
    POSITION_DESC()
      .get(&self.get_position_fu(sect))
      .unwrap()
      .clone()
  }

  fn get_position_cai(self) -> String {
    POSITION_CAI()[self.lock().unwrap().__gan_index as usize + 1]
      .to_string()
  }

  fn get_position_cai_desc(self) -> String {
    POSITION_DESC()
      .get(&self.get_position_cai())
      .unwrap()
      .clone()
  }

  fn get_position_tai_sui(self) -> String {
    POSITION_TAI_SUI_YEAR()[self.lock().unwrap().__zhi_index as usize]
      .to_string()
  }

  fn get_position_tai_sui_desc(self) -> String {
    POSITION_DESC()
      .get(&self.get_position_tai_sui())
      .unwrap()
      .clone()
  }

  ///
  /// 获取往后推几年的阴历年，如果要往前推，则年数用负数
  ///
  /// ## Arguments
  /// + n: **i64** - 年数
  ///
  /// ## Returns
  /// + 阴历年: **LunarYear**
  ///
  fn next(&self, n: i64) -> Arc<Mutex<LunarYear>> {
    LunarYear::from_lunar_year(self.lock().unwrap().__year + n)
  }
}

impl LunarYearRefPrivateHelper for Arc<Mutex<LunarYear>> {
  fn __get_zao_by_gan(&self, index: i64, name: &str) -> String {
    let mut offset = index
      - solar::from_julian_day(
        self.get_month(1).unwrap().get_first_julian_day(),
      )
      .get_lunar()
      .get_day_gan_index();
    if offset < 0 {
      offset = offset + 10;
    }
    name
      .to_string()
      .replace("几", NUMBER()[offset as usize + 1].as_str())
  }

  fn __get_zao_by_zhi(&self, index: i64, name: &str) -> String {
    let mut offset = index
      - solar::from_julian_day(
        self.get_month(1).unwrap().get_first_julian_day(),
      )
      .get_lunar()
      .get_day_zhi_index();
    if offset < 0 {
      offset = offset + 12;
    }

    name
      .to_string()
      .replace("几", NUMBER()[offset as usize + 1].as_str())
  }
}

impl LunarYear {
  pub fn from_lunar_year(lunar_year: i64) -> Arc<Mutex<Self>> {
    match Self::__from_cache(lunar_year) {
      Some(f) => f,
      None => {
        let f = Arc::new(Mutex::new(Self::__compute(lunar_year)));
        Self::__add_to_cache(&f);
        f
      }
    }
  }

  fn __add_to_cache(_obj: &Arc<Mutex<Self>>) {
    __CACHED_LUNARYEARS().lock().unwrap().push(_obj.clone());
  }

  fn __from_cache(lunar_year: i64) -> Option<Arc<Mutex<Self>>> {
    let cached_lunaryears = __CACHED_LUNARYEARS();
    let cached_lunaryears = cached_lunaryears.lock().unwrap();
    let found = cached_lunaryears.iter().find(|lunar_year_arc| {
      lunar_year == lunar_year_arc.lock().unwrap().__year
    });
    match found {
      Some(f) => Some(f.clone()),
      _ => None,
    }
  }

  fn __compute(lunar_year: i64) -> Self {
    let offset = lunar_year - 4;
    let mut year_gan_index = offset % 10;
    let mut year_zhi_index = offset % 12;
    if year_gan_index < 0 {
      year_gan_index = year_gan_index + 10;
    }
    if year_zhi_index < 0 {
      year_zhi_index = year_zhi_index + 12;
    }

    // 节气
    let mut jq = vec![];
    // 合朔，即每月初一
    let mut hs = vec![];
    // 每月天数，长度15
    let mut day_counts = vec![];
    // 月份
    let mut months = vec![];
    //
    let mut jie_qi_julian_days = vec![];

    let current_year = lunar_year;
    let mut jd =
      ((current_year - 2000) as f64 * 365.2422 + 180.).floor();
    // 355是2000.12冬至，得到较靠近jd的冬至估计值
    let mut w =
      ((jd - 355. + 183.) / 365.2422).floor() * 365.2422 + 355.;
    if ShouXingUtil::calc_qi(w) > jd as f64 {
      w = w - 365.2422;
    }
    // 25个节气时刻(北京时间)，从冬至开始到下一个冬至以后
    for i in 0..26 {
      let qi = ShouXingUtil::calc_qi(w + 15.2184 * i as f64);
      jq.push(qi);
    }
    // 从上年的大雪到下年的立春 精确的节气
    let jie_qi_in_use = Lunar::get_jie_qi_in_use();
    for i in 0..jie_qi_in_use.len() {
      if i == 0 {
        jd = ShouXingUtil::qi_accurate2(jq[0] - 15.2184);
      } else if i <= 26 {
        jd = ShouXingUtil::qi_accurate2(jq[i - 1]);
      } else {
        jd = ShouXingUtil::qi_accurate2(
          jq[25] + 15.2184 * (i - 26) as f64,
        );
      }
      jie_qi_julian_days.push(jd + solar::J2000() as f64);
    }

    // 冬至前的初一，今年"首朔"的日月黄经差w
    w = ShouXingUtil::calc_shuo(jq[0]);
    if w > jq[0] {
      w = w - 29.53;
    }
    // 递推每月初一
    for i in 0..16 {
      hs.push(ShouXingUtil::calc_shuo(w + 29.5306 * i as f64))
    }
    // 每月
    for i in 0..15 {
      day_counts.push((hs[i + 1] - hs[i]) as i64);
      months.push(i as i64);
    }

    let prev_year = current_year - 1;
    let leap_index = if __LEAP_11()
      .iter()
      .find(|y| **y == current_year)
      .is_some()
    {
      // leap_index = 13;
      13
    } else if __LEAP_12().iter().find(|y| **y == current_year).is_some()
    {
      // leap_index = 14;
      14
    } else if hs[13] <= jq[24] {
      let mut i = 1;
      while hs[i + 1] > jq[2 * i] && i < 13 {
        i = i + 1;
      }
      // leap_index = i;
      i
    } else {
      16
    };
    for j in leap_index..15 {
      months[j] = months[j] - 1;
    }
    let ymc = [11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec();

    let mut fm = -1;
    let mut index = -1;
    let mut y = prev_year;
    let mut out_months = vec![];
    for i in 0..15 {
      let dm = (hs[i] + solar::J2000() as f64) as i64;
      let v2 = months[i];
      let mut mc = ymc[(v2 % 12) as usize];
      if 1724360 <= dm && dm < 1729794 {
        mc = ymc[((v2 + 1) % 12) as usize];
      } else if 1807724 <= dm && dm < 1808699 {
        mc = ymc[((v2 + 1) % 12) as usize];
      } else if dm == 1729794 || dm == 1808699 {
        mc = 12;
      }

      if fm == -1 {
        fm = mc;
        index = mc;
      }
      if mc < fm {
        y = y + 1;
        index = 1;
      }
      fm = mc;
      if i == leap_index {
        mc = -mc;
      } else if dm == 1729794 || dm == 1808699 {
        mc = -11;
      }
      out_months.push(lunar_month::new(
        y,
        mc,
        day_counts[i],
        dm as f64,
        index,
      ));
      index += 1;
    }

    Self {
      __year: lunar_year,
      __gan_index: year_gan_index,
      __zhi_index: year_zhi_index,
      __months: out_months,
      __jie_qi_julian_days: jie_qi_julian_days,
    }
  }
}

__static_funk!(
  __CACHED_LUNARYEARS,
  Arc<Mutex<Vec<Arc<Mutex<LunarYear>>>>>,
  Arc::new(Mutex::new(vec![]))
);
__static_funk!(
  YUAN,
  Vec<String>,
  ["下", "上", "中"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  YUN,
  Vec<String>,
  ["七", "八", "九", "一", "二", "三", "四", "五", "六"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  __LEAP_11,
  Vec<i64>,
  vec![
    75, 94, 170, 265, 322, 398, 469, 553, 583, 610, 678, 735, 754, 773,
    849, 887, 936, 1050, 1069, 1126, 1145, 1164, 1183, 1259, 1278,
    1308, 1373, 1403, 1441, 1460, 1498, 1555, 1593, 1612, 1631, 1642,
    2033, 2128, 2147, 2242, 2614, 2728, 2910, 3062, 3244, 3339, 3616,
    3711, 3730, 3825, 4007, 4159, 4197, 4322, 4341, 4379, 4417, 4531,
    4599, 4694, 4713, 4789, 4808, 4971, 5085, 5104, 5161, 5180, 5199,
    5294, 5305, 5476, 5677, 5696, 5772, 5791, 5848, 5886, 6049, 6068,
    6144, 6163, 6258, 6402, 6440, 6497, 6516, 6630, 6641, 6660, 6679,
    6736, 6774, 6850, 6869, 6899, 6918, 6994, 7013, 7032, 7051, 7070,
    7089, 7108, 7127, 7146, 7222, 7271, 7290, 7309, 7366, 7385, 7404,
    7442, 7461, 7480, 7491, 7499, 7594, 7624, 7643, 7662, 7681, 7719,
    7738, 7814, 7863, 7882, 7901, 7939, 7958, 7977, 7996, 8034, 8053,
    8072, 8091, 8121, 8159, 8186, 8216, 8235, 8254, 8273, 8311, 8330,
    8341, 8349, 8368, 8444, 8463, 8474, 8493, 8531, 8569, 8588, 8626,
    8664, 8683, 8694, 8702, 8713, 8721, 8751, 8789, 8808, 8816, 8827,
    8846, 8884, 8903, 8922, 8941, 8971, 9036, 9066, 9085, 9104, 9123,
    9142, 9161, 9180, 9199, 9218, 9256, 9294, 9313, 9324, 9343, 9362,
    9381, 9419, 9438, 9476, 9514, 9533, 9544, 9552, 9563, 9571, 9582,
    9601, 9639, 9658, 9666, 9677, 9696, 9734, 9753, 9772, 9791, 9802,
    9821, 9886, 9897, 9916, 9935, 9954, 9973, 9992
  ]
);
__static_funk!(
  __LEAP_12,
  Vec<i64>,
  vec![
    37, 56, 113, 132, 151, 189, 208, 227, 246, 284, 303, 341, 360, 379,
    417, 436, 458, 477, 496, 515, 534, 572, 591, 629, 648, 667, 697,
    716, 792, 811, 830, 868, 906, 925, 944, 963, 982, 1001, 1020, 1039,
    1058, 1088, 1153, 1202, 1221, 1240, 1297, 1335, 1392, 1411, 1422,
    1430, 1517, 1525, 1536, 1574, 3358, 3472, 3806, 3988, 4751, 4941,
    5066, 5123, 5275, 5343, 5438, 5457, 5495, 5533, 5552, 5715, 5810,
    5829, 5905, 5924, 6421, 6535, 6793, 6812, 6888, 6907, 7002, 7184,
    7260, 7279, 7374, 7556, 7746, 7757, 7776, 7833, 7852, 7871, 7966,
    8015, 8110, 8129, 8148, 8224, 8243, 8338, 8406, 8425, 8482, 8501,
    8520, 8558, 8596, 8607, 8615, 8645, 8740, 8778, 8835, 8865, 8930,
    8960, 8979, 8998, 9017, 9055, 9074, 9093, 9112, 9150, 9188, 9237,
    9275, 9332, 9351, 9370, 9408, 9427, 9446, 9457, 9465, 9495, 9560,
    9590, 9628, 9647, 9685, 9715, 9742, 9780, 9810, 9818, 9829, 9848,
    9867, 9905, 9924, 9943, 9962, 10000
  ]
);
