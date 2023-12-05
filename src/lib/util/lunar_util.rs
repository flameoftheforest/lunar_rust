use super::{
  day_shen_sha::DayShenSha,
  day_yi_ji::DayYiJi,
  mmacro::{static_funk, static_vec_string_funk},
  time_yi_ji::TimeYiJi,
};
use chrono::NaiveTime;
use serde_json::Value;
use std::collections::HashMap;

// 农历工具
#[derive(Clone, Debug)]
pub struct LunarUtil {}

impl LunarUtil {
  fn __default() -> Self {
    Self {}
  }
}

impl LunarUtil {
  /// 获取HH:mm时刻的地支序号，非法的时刻返回 **0**
  ///
  /// ## Arguments
  ///
  /// + hm: **&str** - HH:mm 时刻
  ///
  /// ## Returns
  /// + 地支序号: **i64** - 0到11
  ///
  pub fn get_time_zhi_index(hm: &str) -> i64 {
    if hm.len() <= 0 {
      return 0;
    }
    let (hour, min) = {
      let hm = hm.split(":").collect::<Vec<_>>();
      (hm[0].parse::<i64>(), hm[1].parse::<i64>())
    };
    if hour.is_err() || min.is_err() {
      return 0;
    }
    let (hour, min) = (hour.unwrap(), min.unwrap());
    for (idx, (start, end)) in [
      ((1, 0), (2, 59)),
      ((3, 0), (4, 59)),
      ((5, 0), (6, 59)),
      ((7, 0), (8, 59)),
      ((9, 0), (10, 59)),
      ((11, 0), (12, 59)),
      ((13, 0), (14, 59)),
      ((15, 0), (16, 59)),
      ((17, 0), (18, 59)),
      ((19, 0), (20, 59)),
      ((21, 0), (22, 59)),
      // ((23, 0), (0, 59)),
    ]
    .iter()
    .enumerate()
    {
      let start = NaiveTime::from_hms_opt(start.0, start.1, 0).unwrap();
      let end = NaiveTime::from_hms_opt(end.0, end.1, 0).unwrap();
      let test =
        NaiveTime::from_hms_opt(hour as u32, min as u32, 0).unwrap();
      if test >= start && test <= end {
        return (idx + 1) as i64;
      }
    }

    return 0;
  }

  ///
  /// 将HH:mm时刻转换为时辰（地支），非法的时刻返回"子"
  ///
  /// ## Arguments
  /// + hm: **&str** - HH:mm时刻
  ///
  /// ## Returns
  /// + 时辰(地支): **String**，如"子"
  ///
  fn convert_time(hm: &str) -> String {
    ZHI()[LunarUtil::get_time_zhi_index(hm) as usize + 1].to_string()
  }

  ///
  /// 数字转十六进制
  ///
  /// ## Arguments
  /// + n: **i64** 数字
  ///
  /// ## Returns
  /// + 十六进制: **String**
  ///
  pub fn __hex(n: i64) -> String {
    let n = format!("{:X}", n);
    match n.len() < 2 {
      true => format!("0{}", n),
      _ => n,
    }
  }

  ///
  /// 获取干支对应的甲子序号
  ///
  /// ## Arguments
  /// + gan_zhi: **&str** - 干支
  ///
  /// ## Returns
  /// + 甲子序号: **i64**
  ///
  pub fn get_jia_zi_index(gan_zhi: &str) -> i64 {
    for (idx, jia_zi) in JIA_ZI().iter().enumerate() {
      if *jia_zi == gan_zhi {
        return idx as i64;
      }
    }
    return -1;
  }

  ///
  /// 获取日宜
  ///
  /// ## Arguments
  /// + month_gan_zhi: **&str** - 月干支
  /// + day_gan_zhi: **&str** - 日干支
  ///
  /// ## Returns
  /// + 宜: **Vec\<String\>**
  ///
  pub fn get_day_yi(
    month_gan_zhi: &str,
    day_gan_zhi: &str,
  ) -> Vec<String> {
    let day_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(day_gan_zhi));
    let month_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(month_gan_zhi));
    let mut yi = __DAY_YI_JI()
      .clone()
      .collect_yi_hexs(&day_hex, &month_hex)
      .iter()
      .map(|yih| i64::from_str_radix(&yih, 16).unwrap() as usize)
      .map(|yih| __YI_JI()[yih].to_string())
      .collect::<Vec<_>>();
    if yi.is_empty() {
      yi.push("无".to_string());
    }
    yi
  }

  ///
  /// 获取日忌
  ///
  /// ## Arguments
  /// + month_gan_zhi: **&str** - 月干支
  /// + day_gan_zhi: **&str** - 日干支
  ///
  /// ## Returns
  /// + 忌: **Vec\<String\>**
  ///
  pub fn get_day_ji(
    month_gan_zhi: &str,
    day_gan_zhi: &str,
  ) -> Vec<String> {
    let day_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(day_gan_zhi));
    let month_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(month_gan_zhi));
    let mut ji = __DAY_YI_JI()
      .clone()
      .collect_ji_hexs(&day_hex, &month_hex)
      .iter()
      .map(|jih| i64::from_str_radix(&jih, 16).unwrap() as usize)
      .map(|jih| __YI_JI()[jih].to_string())
      .collect::<Vec<_>>();
    if ji.is_empty() {
      ji.push("无".to_string());
    }
    ji
  }

  ///
  /// 获取日吉神
  ///
  /// ## Arguments
  /// + lunar_month: **i64** - 月
  /// + day_gan_zhi: **&str** - 日干支
  ///
  /// ## Returns
  /// + 日吉神: **Vec\<String\>**
  ///
  pub fn get_day_ji_shen(
    lunar_month: i64,
    day_gan_zhi: &str,
  ) -> Vec<String> {
    let day_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(day_gan_zhi));
    let month_hex = format!("{:X}", lunar_month);
    let mut ji_shen = __DAY_SHEN_SHA()
      .clone()
      .collect_ji_shen_hexs(&month_hex, &day_hex)
      .iter()
      .map(|jsh| i64::from_str_radix(&jsh, 16).unwrap() as usize)
      .map(|jsh| __SHEN_SHA()[jsh].to_string())
      .collect::<Vec<_>>();
    if ji_shen.is_empty() {
      ji_shen.push("无".to_string());
    }
    ji_shen
  }

  ///
  /// 获取日凶煞
  ///
  /// ## Arguments
  /// + lunar_month: **i64** - 月
  /// + day_gan_zhi: **&str** - 日干支
  ///
  /// ## Returns
  /// + 日凶煞: **Vec\<String\>**
  ///
  pub fn get_day_xiong_sha(
    lunar_month: i64,
    day_gan_zhi: &str,
  ) -> Vec<String> {
    let day_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(day_gan_zhi));
    let month_hex = format!("{:X}", lunar_month);
    let mut xiong_sha = __DAY_SHEN_SHA()
      .clone()
      .collect_xiong_sha_hexs(&month_hex, &day_hex)
      .iter()
      .map(|xsh| i64::from_str_radix(&xsh, 16).unwrap() as usize)
      .map(|xsh| __SHEN_SHA()[xsh].to_string())
      .collect::<Vec<_>>();
    if xiong_sha.is_empty() {
      xiong_sha.push("无".to_string());
    }
    xiong_sha
  }

  ///
  /// 获取时宜
  ///
  /// ## Arguments
  /// + day_gan_zhi: &str - 日干支
  /// + time_gan_zhi: &str - 时干支
  ///
  /// ## Returns
  /// + 宜: **Vec\<String\>**
  ///
  pub fn get_time_yi(
    day_gan_zhi: &str,
    time_gan_zhi: &str,
  ) -> Vec<String> {
    let day_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(day_gan_zhi));
    let time_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(time_gan_zhi));
    let mut yi = __TIME_YI_JI()
      .collect_yi_hexs(&day_hex, &time_hex)
      .iter()
      .map(|yh| i64::from_str_radix(&yh, 16).unwrap() as usize)
      .map(|yh| __YI_JI()[yh].to_string())
      .collect::<Vec<_>>();
    if yi.is_empty() {
      yi.push("无".to_string());
    }
    yi
  }

  ///
  /// 获取时忌
  ///
  /// ## Arguments
  /// + day_gan_zhi: **&str** - 日干支
  /// + time_gan_zhi: **&str** - 时干支
  ///
  /// ## Returns
  /// + 忌: **Vec\<String\>**
  ///
  pub fn get_time_ji(
    day_gan_zhi: &str,
    time_gan_zhi: &str,
  ) -> Vec<String> {
    let day_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(day_gan_zhi));
    let time_hex =
      LunarUtil::__hex(LunarUtil::get_jia_zi_index(time_gan_zhi));
    let mut ji = __TIME_YI_JI()
      .collect_ji_hexs(&day_hex, &time_hex)
      .iter()
      .map(|jh| i64::from_str_radix(&jh, 16).unwrap() as usize)
      .map(|jh| __YI_JI()[jh].to_string())
      .collect::<Vec<_>>();
    if ji.is_empty() {
      ji.push("无".to_string());
    }
    ji
  }

  ///
  /// 获取干支所在旬下标，0-5
  ///
  /// ## Arguments
  /// + gan_zhi: **&str** - 干支
  ///
  /// ## Returns
  /// + 旬下标: *i64** - 0-5
  ///
  pub fn get_xun_index(gan_zhi: &str) -> i64 {
    let (gan, zhi) = LunarUtil::split_ganzhi(gan_zhi);
    let gan_index = GAN()
      .iter()
      .enumerate()
      .find(|(_, g)| g.to_string() == gan)
      .unwrap()
      .0;
    let zhi_index = ZHI()
      .iter()
      .enumerate()
      .find(|(_, z)| z.to_string() == zhi)
      .unwrap()
      .0;
    let mut diff = gan_index as i64 - zhi_index as i64;
    if diff < 0 {
      diff = diff + 12;
    }
    (diff as f64 / 2.) as i64
  }

  ///
  /// 获取干支所在旬
  ///
  /// ## Arguments
  /// + gan_zhi: **&str** - 干支
  ///
  /// ## Returns
  /// + 旬: **String**
  ///
  pub fn get_xun(gan_zhi: &str) -> String {
    XUN()[LunarUtil::get_xun_index(gan_zhi) as usize].to_string()
  }

  ///
  ///
  /// 获取干支所在旬对应的旬空(空亡)
  ///
  /// ## Arguments
  /// + gan_zhi: **&str** - 干支
  ///
  /// ## Returns
  /// + 旬空(空亡): **String**
  ///
  pub fn get_xun_kong(gan_zhi: &str) -> String {
    XUN_KONG()[LunarUtil::get_xun_index(gan_zhi) as usize].to_string()
  }

  pub fn split_ganzhi(gan_zhi: &str) -> (String, String) {
    let gz = gan_zhi.chars().map(|c| c.to_string()).collect::<Vec<_>>();
    (gz[0].clone(), gz[1].clone())
  }

}

fn gen_word_map(json_str: &str) -> HashMap<String, String> {
  let mut hm = HashMap::new();
  let v: Value = serde_json::from_str(json_str).unwrap();
  for (key, value) in v.as_object().unwrap() {
    hm.insert(key.to_string(), value.to_string().replace("\"", ""));
  }
  hm
}

static_funk!(BASE_MONTH_ZHI_INDEX, i64, 2);
static_vec_string_funk!(
  XUN,
  ["甲子", "甲戌", "甲申", "甲午", "甲辰", "甲寅"]
);
static_vec_string_funk!(
  XUN_KONG,
  ["戌亥", "申酉", "午未", "辰巳", "寅卯", "子丑"]
);
static_vec_string_funk!(
  LIU_YAO,
  ["先胜", "友引", "先负", "佛灭", "大安", "赤口"]
);
static_vec_string_funk!(HOU, ["初候", "二候", "三候"]);
static_vec_string_funk!(
  WU_HOU,
  [
    "蚯蚓结",
    "麋角解",
    "水泉动",
    "雁北乡",
    "鹊始巢",
    "雉始雊",
    "鸡始乳",
    "征鸟厉疾",
    "水泽腹坚",
    "东风解冻",
    "蛰虫始振",
    "鱼陟负冰",
    "獭祭鱼",
    "候雁北",
    "草木萌动",
    "桃始华",
    "仓庚鸣",
    "鹰化为鸠",
    "玄鸟至",
    "雷乃发声",
    "始电",
    "桐始华",
    "田鼠化为鴽",
    "虹始见",
    "萍始生",
    "鸣鸠拂奇羽",
    "戴胜降于桑",
    "蝼蝈鸣",
    "蚯蚓出",
    "王瓜生",
    "苦菜秀",
    "靡草死",
    "麦秋至",
    "螳螂生",
    "鵙始鸣",
    "反舌无声",
    "鹿角解",
    "蜩始鸣",
    "半夏生",
    "温风至",
    "蟋蟀居壁",
    "鹰始挚",
    "腐草为萤",
    "土润溽暑",
    "大雨行时",
    "凉风至",
    "白露降",
    "寒蝉鸣",
    "鹰乃祭鸟",
    "天地始肃",
    "禾乃登",
    "鸿雁来",
    "玄鸟归",
    "群鸟养羞",
    "雷始收声",
    "蛰虫坯户",
    "水始涸",
    "鸿雁来宾",
    "雀入大水为蛤",
    "菊有黄花",
    "豺乃祭兽",
    "草木黄落",
    "蛰虫咸俯",
    "水始冰",
    "地始冻",
    "雉入大水为蜃",
    "虹藏不见",
    "天气上升地气下降",
    "闭塞而成冬",
    "鹖鴠不鸣",
    "虎始交",
    "荔挺出"
  ]
);
static_vec_string_funk!(
  GAN,
  [
    "", "甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"
  ]
);
static_vec_string_funk!(
  POSITION_XI,
  [
    "", "艮", "乾", "坤", "离", "巽", "艮", "乾", "坤", "离", "巽"
  ]
);
static_vec_string_funk!(
  POSITION_YANG_GUI,
  [
    "", "坤", "坤", "兑", "乾", "艮", "坎", "离", "艮", "震", "巽"
  ]
);
static_vec_string_funk!(
  POSITION_YIN_GUI,
  [
    "", "艮", "坎", "乾", "兑", "坤", "坤", "艮", "离", "巽", "震"
  ]
);
static_vec_string_funk!(
  POSITION_FU,
  [
    "", "巽", "巽", "震", "震", "坎", "离", "坤", "坤", "乾", "兑"
  ]
);
static_vec_string_funk!(
  POSITION_FU_2,
  [
    "", "坎", "坤", "乾", "巽", "艮", "坎", "坤", "乾", "巽", "艮"
  ]
);
static_vec_string_funk!(
  POSITION_CAI,
  [
    "", "艮", "艮", "坤", "坤", "坎", "坎", "震", "震", "离", "离"
  ]
);
static_vec_string_funk!(
  POSITION_TAI_SUI_YEAR,
  [
    "坎", "艮", "艮", "震", "巽", "巽", "离", "坤", "坤", "兑", "坎",
    "坎"
  ]
);
static_vec_string_funk!(
  POSITION_GAN,
  ["震", "震", "离", "离", "中", "中", "兑", "兑", "坎", "坎"]
);
static_vec_string_funk!(
  POSITION_ZHI,
  [
    "坎", "中", "震", "震", "中", "离", "离", "中", "兑", "兑", "中",
    "坎"
  ]
);
static_vec_string_funk!(
  POSITION_TAI_DAY,
  [
    "占门碓 外东南",
    "碓磨厕 外东南",
    "厨灶炉 外正南",
    "仓库门 外正南",
    "房床栖 外正南",
    "占门床 外正南",
    "占碓磨 外正南",
    "厨灶厕 外西南",
    "仓库炉 外西南",
    "房床门 外西南",
    "占门栖 外西南",
    "碓磨床 外西南",
    "厨灶碓 外西南",
    "仓库厕 外正西",
    "房床炉 外正西",
    "占大门 外正西",
    "碓磨栖 外正西",
    "厨灶床 外正西",
    "仓库碓 外西北",
    "房床厕 外西北",
    "占门炉 外西北",
    "碓磨门 外西北",
    "厨灶栖 外西北",
    "仓库床 外西北",
    "房床碓 外正北",
    "占门厕 外正北",
    "碓磨炉 外正北",
    "厨灶门 外正北",
    "仓库栖 外正北",
    "占房床 房内北",
    "占门碓 房内北",
    "碓磨厕 房内北",
    "厨灶炉 房内北",
    "仓库门 房内北",
    "房床栖 房内中",
    "占门床 房内中",
    "占碓磨 房内南",
    "厨灶厕 房内南",
    "仓库炉 房内南",
    "房床门 房内西",
    "占门栖 房内东",
    "碓磨床 房内东",
    "厨灶碓 房内东",
    "仓库厕 房内东",
    "房床炉 房内中",
    "占大门 外东北",
    "碓磨栖 外东北",
    "厨灶床 外东北",
    "仓库碓 外东北",
    "房床厕 外东北",
    "占门炉 外东北",
    "碓磨门 外正东",
    "厨灶栖 外正东",
    "仓库床 外正东",
    "房床碓 外正东",
    "占门厕 外正东",
    "碓磨炉 外东南",
    "厨灶门 外东南",
    "仓库栖 外东南",
    "占房床 外东南"
  ]
);
static_vec_string_funk!(
  POSITION_TAI_MONTH,
  [
    "占房床",
    "占户窗",
    "占门堂",
    "占厨灶",
    "占房床",
    "占床仓",
    "占碓磨",
    "占厕户",
    "占门房",
    "占房床",
    "占灶炉",
    "占房床"
  ]
);
static_vec_string_funk!(
  ZHI,
  [
    "", "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉",
    "戌", "亥"
  ]
);
static_vec_string_funk!(
  JIA_ZI,
  [
    "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未",
    "壬申", "癸酉", "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯",
    "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉", "丙戌", "丁亥",
    "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未",
    "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯",
    "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥",
    "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未",
    "庚申", "辛酉", "壬戌", "癸亥"
  ]
);
static_vec_string_funk!(
  ZHI_XING,
  [
    "", "建", "除", "满", "平", "定", "执", "破", "危", "成", "收",
    "开", "闭"
  ]
);
static_vec_string_funk!(
  TIAN_SHEN,
  [
    "", "青龙", "明堂", "天刑", "朱雀", "金匮", "天德", "白虎", "玉堂",
    "天牢", "玄武", "司命", "勾陈"
  ]
);

static_funk!(SHOU, HashMap<String, String>, {
  gen_word_map(r#"{
    "东": "青龙",
    "南": "朱雀",
    "西": "白虎",
    "北": "玄武"
  }"#)
});

static_funk!(GONG,HashMap<String, String>, {
  gen_word_map(r#"{
    "角": "东",
    "井": "南",
    "奎": "西",
    "斗": "北",
    "亢": "东",
    "鬼": "南",
    "娄": "西",
    "牛": "北",
    "氐": "东",
    "柳": "南",
    "胃": "西",
    "女": "北",
    "房": "东",
    "星": "南",
    "昴": "西",
    "虚": "北",
    "心": "东",
    "张": "南",
    "毕": "西",
    "危": "北",
    "尾": "东",
    "翼": "南",
    "觜": "西",
    "室": "北",
    "箕": "东",
    "轸": "南",
    "参": "西",
    "壁": "北"
  }"#)
});

static_funk!(WU_XING_GAN, HashMap<String, String>, {
  gen_word_map(r#"{
    "甲": "木",
    "乙": "木",
    "丙": "火",
    "丁": "火",
    "戊": "土",
    "己": "土",
    "庚": "金",
    "辛": "金",
    "壬": "水",
    "癸": "水"
  }"#)
});

static_vec_string_funk!(
  CHONG,
  [
    "午", "未", "申", "酉", "戌", "亥", "子", "丑", "寅", "卯", "辰",
    "巳"
  ]
);
static_vec_string_funk!(
  CHONG_GAN,
  ["戊", "己", "庚", "辛", "壬", "癸", "甲", "乙", "丙", "丁"]
);
static_vec_string_funk!(
  CHONG_GAN_TIE,
  ["己", "戊", "辛", "庚", "癸", "壬", "乙", "甲", "丁", "丙"]
);
static_vec_string_funk!(
  CHONG_GAN_4,
  ["庚", "辛", "壬", "癸", "", "", "甲", "乙", "丙", "丁"]
);
static_vec_string_funk!(
  HE_GAN_5,
  ["己", "庚", "辛", "壬", "癸", "甲", "乙", "丙", "丁", "戊"]
);
static_vec_string_funk!(
  HE_ZHI_6,
  [
    "丑", "子", "亥", "戌", "酉", "申", "未", "午", "巳", "辰", "卯",
    "寅"
  ]
);

static_funk!(POSITION_DESC, HashMap<String, String>, {
  gen_word_map(r#"{
    "坎": "正北",
    "艮": "东北",
    "震": "正东",
    "巽": "东南",
    "离": "正南",
    "坤": "西南",
    "兑": "正西",
    "乾": "西北",
    "中": "中宫"
  }"#)
});

static_funk!(SHA, HashMap<String, String>, {
  gen_word_map(r#"{"子": "南",
    "丑": "东",
    "寅": "北",
    "卯": "西",
    "辰": "南",
    "巳": "东",
    "午": "北",
    "未": "西",
    "申": "南",
    "酉": "东",
    "戌": "北",
    "亥": "西"
  }"#)
});

static_funk!(XIU_SONG, HashMap<String, String>, {
  gen_word_map(r#"{
    "角": "角星造作主荣昌，外进田财及女郎，嫁娶婚姻出贵子，文人及第见君王，惟有埋葬不可用，三年之后主瘟疫，起工修筑坟基地，堂前立见主人凶。",
    "亢": "亢星造作长房当，十日之中主有殃，田地消磨官失职，接运定是虎狼伤，嫁娶婚姻用此日，儿孙新妇守空房，埋葬若还用此日，当时害祸主重伤。",
    "氐": "氐星造作主灾凶，费尽田园仓库空，埋葬不可用此日，悬绳吊颈祸重重，若是婚姻离别散，夜招浪子入房中，行船必定遭沉没，更生聋哑子孙穷。",
    "房": "房星造作田园进，钱财牛马遍山岗，更招外处田庄宅，荣华富贵福禄康，埋葬若然用此日，高官进职拜君王，嫁娶嫦娥至月殿，三年抱子至朝堂。",
    "心": "心星造作大为凶，更遭刑讼狱囚中，忤逆官非宅产退，埋葬卒暴死相从，婚姻若是用此日，子死儿亡泪满胸，三年之内连遭祸，事事教君没始终。",
    "尾": "尾星造作主天恩，富贵荣华福禄增，招财进宝兴家宅，和合婚姻贵子孙，埋葬若能依此日，男清女正子孙兴，开门放水招田宅，代代公侯远播名。",
    "箕": "箕星造作主高强，岁岁年年大吉昌，埋葬修坟大吉利，田蚕牛马遍山岗，开门放水招田宅，箧满金银谷满仓，福荫高官加禄位，六亲丰禄乐安康。",
    "斗": "斗星造作主招财，文武官员位鼎台，田宅家财千万进，坟堂修筑贵富来，开门放水招牛马，旺蚕男女主和谐，遇此吉宿来照护，时支福庆永无灾。",
    "牛": "牛星造作主灾危，九横三灾不可推，家宅不安人口退，田蚕不利主人衰，嫁娶婚姻皆自损，金银财谷渐无之，若是开门并放水，牛猪羊马亦伤悲。",
    "女": "女星造作损婆娘，兄弟相嫌似虎狼，埋葬生灾逢鬼怪，颠邪疾病主瘟惶，为事遭官财失散，泻利留连不可当，开门放水用此日，全家财散主离乡。",
    "虚": "虚星造作主灾殃，男女孤眠不一双，内乱风声无礼节，儿孙媳妇伴人床，开门放水遭灾祸，虎咬蛇伤又卒亡，三三五五连年病，家破人亡不可当。",
    "危": "危星不可造高楼，自遭刑吊见血光，三年孩子遭水厄，后生出外永不还，埋葬若还逢此日，周年百日取高堂，三年两载一悲伤，开门放水到官堂。",
    "室": "室星修造进田牛，儿孙代代近王侯，家贵荣华天上至，寿如彭祖八千秋，开门放水招财帛，和合婚姻生贵儿，埋葬若能依此日，门庭兴旺福无休。",
    "壁": "壁星造作主增财，丝蚕大熟福滔天，奴婢自来人口进，开门放水出英贤，埋葬招财官品进，家中诸事乐陶然，婚姻吉利主贵子，早播名誉著祖鞭。",
    "奎": "奎星造作得祯祥，家内荣和大吉昌，若是埋葬阴卒死，当年定主两三伤，看看军令刑伤到，重重官事主瘟惶，开门放水遭灾祸，三年两次损儿郎。",
    "娄": "娄星修造起门庭，财旺家和事事兴，外进钱财百日进，一家兄弟播高名，婚姻进益生贵子，玉帛金银箱满盈，放水开门皆吉利，男荣女贵寿康宁。",
    "胃": "胃星造作事如何，家贵荣华喜气多，埋葬贵临官禄位，夫妇齐眉永保康，婚姻遇此家富贵，三灾九祸不逢他，从此门前多吉庆，儿孙代代拜金阶。",
    "昴": "昴星造作进田牛，埋葬官灾不得休，重丧二日三人死，尽卖田园不记增，开门放水招灾祸，三岁孩儿白了头，婚姻不可逢此日，死别生离是可愁。",
    "毕": "毕星造作主光前，买得田园有余钱，埋葬此日添官职，田蚕大熟永丰年，开门放水多吉庆，合家人口得安然，婚姻若得逢此日，生得孩儿福寿全。",
    "觜": "觜星造作有徒刑，三年必定主伶丁，埋葬卒死多因此，取定寅年使杀人，三丧不止皆由此，一人药毒二人身，家门田地皆退败，仓库金银化作尘。",
    "参": "参星造作旺人家，文星照耀大光华，只因造作田财旺，埋葬招疾哭黄沙，开门放水加官职，房房子孙见田加，婚姻许遁遭刑克，男女朝开幕落花。",
    "井": "井星造作旺蚕田，金榜题名第一光，埋葬须防惊卒死，狂颠风疾入黄泉，开门放水招财帛，牛马猪羊旺莫言，贵人田塘来入宅，儿孙兴旺有余钱。",
    "鬼": "鬼星起造卒人亡，堂前不见主人郎，埋葬此日官禄至，儿孙代代近君王，开门放水须伤死，嫁娶夫妻不久长，修土筑墙伤产女，手扶双女泪汪汪。",
    "柳": "柳星造作主遭官，昼夜偷闭不暂安，埋葬瘟惶多疾病，田园退尽守冬寒，开门放水遭聋瞎，腰驼背曲似弓弯，更有棒刑宜谨慎，妇人随客走盘桓。",
    "星": "星宿日好造新房，进职加官近帝王，不可埋葬并放水，凶星临位女人亡，生离死别无心恋，要自归休别嫁郎，孔子九曲殊难度，放水开门天命伤。",
    "张": "张星日好造龙轩，年年并见进庄田，埋葬不久升官职，代代为官近帝前，开门放水招财帛，婚姻和合福绵绵，田蚕人满仓库满，百般顺意自安然。",
    "翼": "翼星不利架高堂，三年二载见瘟惶，埋葬若还逢此日，子孙必定走他乡，婚姻此日不宜利，归家定是不相当，开门放水家须破，少女恋花贪外郎。",
    "轸": "轸星临水造龙宫，代代为官受皇封，富贵荣华增寿禄，库满仓盈自昌隆，埋葬文昌来照助，宅舍安宁不见凶，更有为官沾帝宠，婚姻龙子入龙宫。"
  }"#)
});

static_funk!(XIU_LUCK, HashMap<String, String>, {
  gen_word_map(r#"{
    "角": "吉",
    "亢": "凶",
    "氐": "凶",
    "房": "吉",
    "心": "凶",
    "尾": "吉",
    "箕": "吉",
    "斗": "吉",
    "牛": "凶",
    "女": "凶",
    "虚": "凶",
    "危": "凶",
    "室": "吉",
    "壁": "吉",
    "奎": "凶",
    "娄": "吉",
    "胃": "吉",
    "昴": "凶",
    "毕": "吉",
    "觜": "凶",
    "参": "吉",
    "井": "吉",
    "鬼": "凶",
    "柳": "凶",
    "星": "凶",
    "张": "吉",
    "翼": "凶",
    "轸": "吉"
  }"#)
});

static_funk!(WU_XING_ZHI, HashMap<String, String>, {
  gen_word_map(r#"{
    "寅": "木",
    "卯": "木",
    "巳": "火",
    "午": "火",
    "辰": "土",
    "丑": "土",
    "戌": "土",
    "未": "土",
    "申": "金",
    "酉": "金",
    "亥": "水",
    "子": "水"
  }"#)
});

static_funk!(ANIMAL, HashMap<String, String>, {
  gen_word_map(r#"{
    "角": "蛟",
    "斗": "獬",
    "奎": "狼",
    "井": "犴",
    "亢": "龙",
    "牛": "牛",
    "娄": "狗",
    "鬼": "羊",
    "女": "蝠",
    "氐": "貉",
    "胃": "彘",
    "柳": "獐",
    "房": "兔",
    "虚": "鼠",
    "昴": "鸡",
    "星": "马",
    "心": "狐",
    "危": "燕",
    "毕": "乌",
    "张": "鹿",
    "尾": "虎",
    "室": "猪",
    "觜": "猴",
    "翼": "蛇",
    "箕": "豹",
    "壁": "獝",
    "参": "猿",
    "轸": "蚓"
  }"#)
});

static_funk!(OTHER_FESTIVAL, HashMap<(i64, i64), Vec<String>>, {
  let json_str = r#"{
  "1-4": ["接神日"],
  "1-5": ["隔开日"],
  "1-7": ["人日"],
  "1-8": ["谷日", "顺星节"],
  "1-9": ["天日"],
  "1-10": ["地日"],
  "1-20": ["天穿节"],
  "1-25": ["填仓节"],
  "1-30": ["正月晦"],
  "2-1": ["中和节"],
  "2-2": ["社日节"],
  "3-3": ["上巳节"],
  "5-20": ["分龙节"],
  "5-25": ["会龙节"],
  "6-6": ["天贶节"],
  "6-24": ["观莲节"],
  "6-25": ["五谷母节"],
  "7-15": ["中元节"],
  "7-22": ["财神节"],
  "7-29": ["地藏节"],
  "8-1": ["天灸日"],
  "10-1": ["寒衣节"],
  "10-10": ["十成节"],
  "10-15": ["下元节"],
  "12-7": ["驱傩日"],
  "12-16": ["尾牙"],
  "12-24": ["祭灶日"]
  }"#;
  let v: Value = serde_json::from_str(json_str).unwrap();
  let hm = v.as_object().unwrap().iter().map(|(key, value)| {
    let key = key
      .split("-")
      .collect::<Vec<_>>()
      .iter()
      .map(|v| v.parse::<i64>().unwrap())
      .collect::<Vec<_>>();
    let value = value
      .as_array()
      .unwrap()
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<_>>();
    ((key[0], key[1]), value)
  }).collect::<HashMap<_,_>>();
  hm
});

static_vec_string_funk!(
  PENG_ZU_GAN,
  [
    "",
    "甲不开仓财物耗散",
    "乙不栽植千株不长",
    "丙不修灶必见灾殃",
    "丁不剃头头必生疮",
    "戊不受田田主不祥",
    "己不破券二比并亡",
    "庚不经络织机虚张",
    "辛不合酱主人不尝",
    "壬不泱水更难提防",
    "癸不词讼理弱敌强"
  ]
);
static_vec_string_funk!(
  PENG_ZU_ZHI,
  [
    "",
    "子不问卜自惹祸殃",
    "丑不冠带主不还乡",
    "寅不祭祀神鬼不尝",
    "卯不穿井水泉不香",
    "辰不哭泣必主重丧",
    "巳不远行财物伏藏",
    "午不苫盖屋主更张",
    "未不服药毒气入肠",
    "申不安床鬼祟入房",
    "酉不会客醉坐颠狂",
    "戌不吃犬作怪上床",
    "亥不嫁娶不利新郎"
  ]
);
static_vec_string_funk!(
  NUMBER,
  [
    "〇", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十",
    "十一", "十二"
  ]
);
static_vec_string_funk!(
  MONTH,
  [
    "", "正", "二", "三", "四", "五", "六", "七", "八", "九", "十",
    "冬", "腊"
  ]
);
static_vec_string_funk!(
  SEASON,
  [
    "", "孟春", "仲春", "季春", "孟夏", "仲夏", "季夏", "孟秋", "仲秋",
    "季秋", "孟冬", "仲冬", "季冬"
  ]
);
static_vec_string_funk!(
  SHENGXIAO,
  [
    "", "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡",
    "狗", "猪"
  ]
);
static_vec_string_funk!(
  DAY,
  [
    "", "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八",
    "初九", "初十", "十一", "十二", "十三", "十四", "十五", "十六",
    "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十"
  ]
);
static_vec_string_funk!(
  YUE_XIANG,
  [
    "",
    "朔",
    "既朔",
    "蛾眉新",
    "蛾眉新",
    "蛾眉",
    "夕",
    "上弦",
    "上弦",
    "九夜",
    "宵",
    "宵",
    "宵",
    "渐盈凸",
    "小望",
    "望",
    "既望",
    "立待",
    "居待",
    "寝待",
    "更待",
    "渐亏凸",
    "下弦",
    "下弦",
    "有明",
    "有明",
    "蛾眉残",
    "蛾眉残",
    "残",
    "晓",
    "晦"
  ]
);

static_funk!(FESTIVAL, HashMap<(i64, i64), String>, {
  let mut hm = HashMap::new();
  let json_str = r#"{
    "1-1": "春节",
    "1-15": "元宵节",
    "2-2": "龙头节",
    "5-5": "端午节",
    "7-7": "七夕节",
    "8-15": "中秋节",
    "9-9": "重阳节",
    "12-8": "腊八节"        
  }"#;
  let v: Value = serde_json::from_str(json_str).unwrap();
  v.as_object().unwrap().iter().for_each(|(key, value)| {
    let tup = {
      let key = key
        .split("-")
        .collect::<Vec<_>>()
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
      (key[0], key[1])
    };
    let v = value.as_str().unwrap().to_string();
    hm.insert(tup, v);
  });
  hm
});

static_funk!(ZHI_TIAN_SHEN_OFFSET, HashMap<String, i64>, {
  let mut hm = HashMap::new();
  let v: Value = serde_json::from_str(r#"{
    "子": 4,
    "丑": 2,
    "寅": 0,
    "卯": 10,
    "辰": 8,
    "巳": 6,
    "午": 4,
    "未": 2,
    "申": 0,
    "酉": 10,
    "戌": 8,
    "亥": 6
  }"#).unwrap();
  v.as_object().unwrap().iter().for_each(|(key, val)| {
    hm.insert(key.to_string(), val.as_i64().unwrap());
  });
  hm
});

static_funk!(TIAN_SHEN_TYPE, HashMap<String, String>, {
  gen_word_map(r#"{
    "青龙": "黄道",
    "明堂": "黄道",
    "金匮": "黄道",
    "天德": "黄道",
    "玉堂": "黄道",
    "司命": "黄道",
    "天刑": "黑道",
    "朱雀": "黑道",
    "白虎": "黑道",
    "天牢": "黑道",
    "玄武": "黑道",
    "勾陈": "黑道"
  }"#)
});

static_funk!(TIAN_SHEN_TYPE_LUCK, HashMap<String, String>, {
  gen_word_map(r#"{
    "黄道": "吉",
    "黑道": "凶"
  }"#)
});

static_funk!(LU, HashMap<String, Vec<String>>, {
  let mut hm = HashMap::new();
  let json_str = r#"{
    "甲": ["寅"],
    "乙": ["卯"],
    "丙": ["巳"],
    "丁": ["午"],
    "戊": ["巳"],
    "己": ["午"],
    "庚": ["申"],
    "辛": ["酉"],
    "壬": ["亥"],
    "癸": ["子"],
    "寅": ["甲"],
    "卯": ["乙"],
    "巳": ["丙","戊"],
    "午": ["丁","己"],
    "申": ["庚"],
    "酉": ["辛"],
    "亥": ["壬"],
    "子": ["癸"]
  }"#;
  let v: Value = serde_json::from_str(json_str).unwrap();
  v.as_object().unwrap().iter().for_each(|(key, value)| {
    let value = value.as_array().unwrap().iter().map(|vv| vv.as_str().unwrap().to_string()).collect::<Vec<_>>();
    hm.insert(key.to_string(), value);
  });
  hm
});

static_funk!(XIU, HashMap<String, String>, {
  gen_word_map(r#"{
    "申1": "毕",
    "申2": "翼",
    "申3": "箕",
    "申4": "奎",
    "申5": "鬼",
    "申6": "氐",
    "申0": "虚",

    "子1": "毕",
    "子2": "翼",
    "子3": "箕",
    "子4": "奎",
    "子5": "鬼",
    "子6": "氐",
    "子0": "虚",

    "辰1": "毕",
    "辰2": "翼",
    "辰3": "箕",
    "辰4": "奎",
    "辰5": "鬼",
    "辰6": "氐",
    "辰0": "虚",

    "巳1": "危",
    "巳2": "觜",
    "巳3": "轸",
    "巳4": "斗",
    "巳5": "娄",
    "巳6": "柳",
    "巳0": "房",

    "酉1": "危",
    "酉2": "觜",
    "酉3": "轸",
    "酉4": "斗",
    "酉5": "娄",
    "酉6": "柳",
    "酉0": "房",

    "丑1": "危",
    "丑2": "觜",
    "丑3": "轸",
    "丑4": "斗",
    "丑5": "娄",
    "丑6": "柳",
    "丑0": "房",

    "寅1": "心",
    "寅2": "室",
    "寅3": "参",
    "寅4": "角",
    "寅5": "牛",
    "寅6": "胃",
    "寅0": "星",

    "午1": "心",
    "午2": "室",
    "午3": "参",
    "午4": "角",
    "午5": "牛",
    "午6": "胃",
    "午0": "星",

    "戌1": "心",
    "戌2": "室",
    "戌3": "参",
    "戌4": "角",
    "戌5": "牛",
    "戌6": "胃",
    "戌0": "星",

    "亥1": "张",
    "亥2": "尾",
    "亥3": "壁",
    "亥4": "井",
    "亥5": "亢",
    "亥6": "女",
    "亥0": "昴",

    "卯1": "张",
    "卯2": "尾",
    "卯3": "壁",
    "卯4": "井",
    "卯5": "亢",
    "卯6": "女",
    "卯0": "昴",

    "未1": "张",
    "未2": "尾",
    "未3": "壁",
    "未4": "井",
    "未5": "亢",
    "未6": "女",
    "未0": "昴"
  }"#)
});

static_funk!(ZHENG, HashMap<String, String>, {
  gen_word_map(r#"{
    "角": "木",
    "井": "木",
    "奎": "木",
    "斗": "木",
    "亢": "金",
    "鬼": "金",
    "娄": "金",
    "牛": "金",
    "氐": "土",
    "柳": "土",
    "胃": "土",
    "女": "土",
    "房": "日",
    "星": "日",
    "昴": "日",
    "虚": "日",
    "心": "月",
    "张": "月",
    "毕": "月",
    "危": "月",
    "尾": "火",
    "翼": "火",
    "觜": "火",
    "室": "火",
    "箕": "水",
    "轸": "水",
    "参": "水",
    "壁": "水"
  }"#)
});

static_funk!(NAYIN, HashMap<String, String>, {
  gen_word_map(r#"{
    "甲子": "海中金",
    "甲午": "沙中金",
    "丙寅": "炉中火",
    "丙申": "山下火",
    "戊辰": "大林木",
    "戊戌": "平地木",
    "庚午": "路旁土",
    "庚子": "壁上土",
    "壬申": "剑锋金",
    "壬寅": "金箔金",
    "甲戌": "山头火",
    "甲辰": "覆灯火",
    "丙子": "涧下水",
    "丙午": "天河水",
    "戊寅": "城头土",
    "戊申": "大驿土",
    "庚辰": "白蜡金",
    "庚戌": "钗钏金",
    "壬午": "杨柳木",
    "壬子": "桑柘木",
    "甲申": "泉中水",
    "甲寅": "大溪水",
    "丙戌": "屋上土",
    "丙辰": "沙中土",
    "戊子": "霹雳火",
    "戊午": "天上火",
    "庚寅": "松柏木",
    "庚申": "石榴木",
    "壬辰": "长流水",
    "壬戌": "大海水",
    "乙丑": "海中金",
    "乙未": "沙中金",
    "丁卯": "炉中火",
    "丁酉": "山下火",
    "己巳": "大林木",
    "己亥": "平地木",
    "辛未": "路旁土",
    "辛丑": "壁上土",
    "癸酉": "剑锋金",
    "癸卯": "金箔金",
    "乙亥": "山头火",
    "乙巳": "覆灯火",
    "丁丑": "涧下水",
    "丁未": "天河水",
    "己卯": "城头土",
    "己酉": "大驿土",
    "辛巳": "白蜡金",
    "辛亥": "钗钏金",
    "癸未": "杨柳木",
    "癸丑": "桑柘木",
    "乙酉": "泉中水",
    "乙卯": "大溪水",
    "丁亥": "屋上土",
    "丁巳": "沙中土",
    "己丑": "霹雳火",
    "己未": "天上火",
    "辛卯": "松柏木",
    "辛酉": "石榴木",
    "癸巳": "长流水",
    "癸亥": "大海水"
  }"#)
});

static_funk!(ZHI_HIDE_GAN, HashMap<String, Vec<String>>, {
  let mut hm = HashMap::new();
  let v: Value = serde_json::from_str(r#"{
    "子": ["癸"],
    "丑": ["己", "癸", "辛"],
    "寅": ["甲", "丙", "戊"],
    "卯": ["乙"],
    "辰": ["戊", "乙", "癸"],
    "巳": ["丙", "庚", "戊"],
    "午": ["丁", "己"],
    "未": ["己", "丁", "乙"],
    "申": ["庚", "壬", "戊"],
    "酉": ["辛"],
    "戌": ["戊", "辛", "丁"],
    "亥": ["壬", "甲"]
  }"#).unwrap();
  v.as_object().unwrap().iter().for_each(|(key, value)| {
    let value = value.as_array().unwrap();
    let value = value.iter().map(|vv| vv.to_string().replace("\"", "")).collect::<Vec<_>>();
    hm.insert(key.to_string(), value);
  });
  hm
});

static_funk!(SHI_SHEN, HashMap<String, String>, {
  gen_word_map(r#"{
    "甲甲": "比肩",
    "甲乙": "劫财",
    "甲丙": "食神",
    "甲丁": "伤官",
    "甲戊": "偏财",
    "甲己": "正财",
    "甲庚": "七杀",
    "甲辛": "正官",
    "甲壬": "偏印",
    "甲癸": "正印",
    "乙乙": "比肩",
    "乙甲": "劫财",
    "乙丁": "食神",
    "乙丙": "伤官",
    "乙己": "偏财",
    "乙戊": "正财",
    "乙辛": "七杀",
    "乙庚": "正官",
    "乙癸": "偏印",
    "乙壬": "正印",
    "丙丙": "比肩",
    "丙丁": "劫财",
    "丙戊": "食神",
    "丙己": "伤官",
    "丙庚": "偏财",
    "丙辛": "正财",
    "丙壬": "七杀",
    "丙癸": "正官",
    "丙甲": "偏印",
    "丙乙": "正印",
    "丁丁": "比肩",
    "丁丙": "劫财",
    "丁己": "食神",
    "丁戊": "伤官",
    "丁辛": "偏财",
    "丁庚": "正财",
    "丁癸": "七杀",
    "丁壬": "正官",
    "丁乙": "偏印",
    "丁甲": "正印",
    "戊戊": "比肩",
    "戊己": "劫财",
    "戊庚": "食神",
    "戊辛": "伤官",
    "戊壬": "偏财",
    "戊癸": "正财",
    "戊甲": "七杀",
    "戊乙": "正官",
    "戊丙": "偏印",
    "戊丁": "正印",
    "己己": "比肩",
    "己戊": "劫财",
    "己辛": "食神",
    "己庚": "伤官",
    "己癸": "偏财",
    "己壬": "正财",
    "己乙": "七杀",
    "己甲": "正官",
    "己丁": "偏印",
    "己丙": "正印",
    "庚庚": "比肩",
    "庚辛": "劫财",
    "庚壬": "食神",
    "庚癸": "伤官",
    "庚甲": "偏财",
    "庚乙": "正财",
    "庚丙": "七杀",
    "庚丁": "正官",
    "庚戊": "偏印",
    "庚己": "正印",
    "辛辛": "比肩",
    "辛庚": "劫财",
    "辛癸": "食神",
    "辛壬": "伤官",
    "辛乙": "偏财",
    "辛甲": "正财",
    "辛丁": "七杀",
    "辛丙": "正官",
    "辛己": "偏印",
    "辛戊": "正印",
    "壬壬": "比肩",
    "壬癸": "劫财",
    "壬甲": "食神",
    "壬乙": "伤官",
    "壬丙": "偏财",
    "壬丁": "正财",
    "壬戊": "七杀",
    "壬己": "正官",
    "壬庚": "偏印",
    "壬辛": "正印",
    "癸癸": "比肩",
    "癸壬": "劫财",
    "癸乙": "食神",
    "癸甲": "伤官",
    "癸丁": "偏财",
    "癸丙": "正财",
    "癸己": "七杀",
    "癸戊": "正官",
    "癸辛": "偏印",
    "癸庚": "正印"
  }"#)
});

static_vec_string_funk!(
  __YI_JI,
  [
    "祭祀",
    "祈福",
    "求嗣",
    "开光",
    "塑绘",
    "齐醮",
    "斋醮",
    "沐浴",
    "酬神",
    "造庙",
    "祀灶",
    "焚香",
    "谢土",
    "出火",
    "雕刻",
    "嫁娶",
    "订婚",
    "纳采",
    "问名",
    "纳婿",
    "归宁",
    "安床",
    "合帐",
    "冠笄",
    "订盟",
    "进人口",
    "裁衣",
    "挽面",
    "开容",
    "修坟",
    "启钻",
    "破土",
    "安葬",
    "立碑",
    "成服",
    "除服",
    "开生坟",
    "合寿木",
    "入殓",
    "移柩",
    "普渡",
    "入宅",
    "安香",
    "安门",
    "修造",
    "起基",
    "动土",
    "上梁",
    "竖柱",
    "开井开池",
    "作陂放水",
    "拆卸",
    "破屋",
    "坏垣",
    "补垣",
    "伐木做梁",
    "作灶",
    "解除",
    "开柱眼",
    "穿屏扇架",
    "盖屋合脊",
    "开厕",
    "造仓",
    "塞穴",
    "平治道涂",
    "造桥",
    "作厕",
    "筑堤",
    "开池",
    "伐木",
    "开渠",
    "掘井",
    "扫舍",
    "放水",
    "造屋",
    "合脊",
    "造畜稠",
    "修门",
    "定磉",
    "作梁",
    "修饰垣墙",
    "架马",
    "开市",
    "挂匾",
    "纳财",
    "求财",
    "开仓",
    "买车",
    "置产",
    "雇庸",
    "出货财",
    "安机械",
    "造车器",
    "经络",
    "酝酿",
    "作染",
    "鼓铸",
    "造船",
    "割蜜",
    "栽种",
    "取渔",
    "结网",
    "牧养",
    "安碓磑",
    "习艺",
    "入学",
    "理发",
    "探病",
    "见贵",
    "乘船",
    "渡水",
    "针灸",
    "出行",
    "移徙",
    "分居",
    "剃头",
    "整手足甲",
    "纳畜",
    "捕捉",
    "畋猎",
    "教牛马",
    "会亲友",
    "赴任",
    "求医",
    "治病",
    "词讼",
    "起基动土",
    "破屋坏垣",
    "盖屋",
    "造仓库",
    "立券交易",
    "交易",
    "立券",
    "安机",
    "会友",
    "求医疗病",
    "诸事不宜",
    "馀事勿取",
    "行丧",
    "断蚁",
    "归岫",
    "无"
  ]
);
static_funk!(__DAY_YI_JI, DayYiJi, DayYiJi::default());
static_funk!(__TIME_YI_JI, TimeYiJi, TimeYiJi::default());
static_vec_string_funk!(
  __SHEN_SHA,
  [
    "无",
    "天恩",
    "母仓",
    "时阳",
    "生气",
    "益后",
    "青龙",
    "灾煞",
    "天火",
    "四忌",
    "八龙",
    "复日",
    "续世",
    "明堂",
    "月煞",
    "月虚",
    "血支",
    "天贼",
    "五虚",
    "土符",
    "归忌",
    "血忌",
    "月德",
    "月恩",
    "四相",
    "王日",
    "天仓",
    "不将",
    "要安",
    "五合",
    "鸣吠对",
    "月建",
    "小时",
    "土府",
    "往亡",
    "天刑",
    "天德",
    "官日",
    "吉期",
    "玉宇",
    "大时",
    "大败",
    "咸池",
    "朱雀",
    "守日",
    "天巫",
    "福德",
    "六仪",
    "金堂",
    "金匮",
    "厌对",
    "招摇",
    "九空",
    "九坎",
    "九焦",
    "相日",
    "宝光",
    "天罡",
    "死神",
    "月刑",
    "月害",
    "游祸",
    "重日",
    "时德",
    "民日",
    "三合",
    "临日",
    "天马",
    "时阴",
    "鸣吠",
    "死气",
    "地囊",
    "白虎",
    "月德合",
    "敬安",
    "玉堂",
    "普护",
    "解神",
    "小耗",
    "天德合",
    "月空",
    "驿马",
    "天后",
    "除神",
    "月破",
    "大耗",
    "五离",
    "天牢",
    "阴德",
    "福生",
    "天吏",
    "致死",
    "元武",
    "阳德",
    "天喜",
    "天医",
    "司命",
    "月厌",
    "地火",
    "四击",
    "大煞",
    "大会",
    "天愿",
    "六合",
    "五富",
    "圣心",
    "河魁",
    "劫煞",
    "四穷",
    "勾陈",
    "触水龙",
    "八风",
    "天赦",
    "五墓",
    "八专",
    "阴错",
    "四耗",
    "阳错",
    "四废",
    "三阴",
    "小会",
    "阴道冲阳",
    "单阴",
    "孤辰",
    "阴位",
    "行狠",
    "了戾",
    "绝阴",
    "纯阳",
    "七鸟",
    "岁薄",
    "阴阳交破",
    "阴阳俱错",
    "阴阳击冲",
    "逐阵",
    "阳错阴冲",
    "七符",
    "天狗",
    "九虎",
    "成日",
    "天符",
    "孤阳",
    "绝阳",
    "纯阴",
    "六蛇",
    "阴神",
    "解除",
    "阳破阴冲"
  ]
);
static_funk!(__DAY_SHEN_SHA, DayShenSha, DayShenSha::default());
