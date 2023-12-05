//
// 道历工具
//

use super::mmacro::{
  __static_funk, static_funk, static_vec_string_funk,
};
use crate::tao_festival::{self, TaoFestivalRef};
use serde_json::Value;
use std::collections::HashMap;

// 三会日
static_vec_string_funk!(SAN_HUI, ["1-7", "7-7", "10-15"]);

// 三元日
static_vec_string_funk!(SAN_YUAN, ["1-15", "7-15", "10-15"]);

// 五腊日
static_vec_string_funk!(WU_LA, ["1-1", "5-5", "7-7", "10-1", "12-8"]);

// 暗戊
static_vec_string_funk!(
  AN_WU,
  [
    "未", "戌", "辰", "寅", "午", "子", "酉", "申", "巳", "亥", "卯",
    "丑"
  ]
);

// 八会日
static_funk!(BA_HUI, HashMap<String, String>, {
  str_to_map(r#"{
    "丙午": "天会",
    "壬午": "地会",
    "壬子": "人会",
    "庚午": "日会",
    "庚申": "月会",
    "辛酉": "星辰会",
    "甲辰": "五行会",
    "甲戌": "四时会"
  }"#)
});

// 八节日
static_funk!(BA_JIE, HashMap<String, String>, {
  str_to_map(r#"{
      "立春": "东北方度仙上圣天尊同梵炁始青天君下降",
      "春分": "东方玉宝星上天尊同青帝九炁天君下降",
      "立夏": "东南方好生度命天尊同梵炁始丹天君下降",
      "夏至": "南方玄真万福天尊同赤帝三炁天君下降",
      "立秋": "西南方太灵虚皇天尊同梵炁始素天君下降",
      "秋分": "西方太妙至极天尊同白帝七炁天君下降",
      "立冬": "西北方无量太华天尊同梵炁始玄天君下降",
      "冬至": "北方玄上玉宸天尊同黑帝五炁天君下降"
  }"#)
});

// 日期对应的节日
static_funk!(FESTIVAL, HashMap<String, Vec<TaoFestivalRef>>, {
  __FESTIVAL_KEY().iter().zip(__FESTIVAL_VALUE().iter()).map(|(k, v)| {
    (k.to_string(), v.clone())
  }).collect::<HashMap<_,_>>()
});

__static_funk!(
  __FESTIVAL_KEY,
  Vec<String>,
  [
    "1-1", "1-3", "1-5", "1-7", "1-9", "1-13", "1-15", "1-19", "1-28",
    "2-1", "2-2", "2-3", "2-6", "2-13", "2-15", "2-19", "3-1", "3-3",
    "3-6", "3-15", "3-16", "3-18", "3-19", "3-20", "3-23", "3-26",
    "3-28", "4-1", "4-10", "4-14", "4-15", "4-18", "4-20", "4-28",
    "5-1", "5-5", "5-11", "5-13", "5-18", "5-20", "5-29", "6-1", "6-2",
    "6-3", "6-4", "6-5", "6-6", "6-10", "6-15", "6-19", "6-23", "6-24",
    "6-26", "7-7", "7-12", "7-15", "7-18", "7-20", "7-22", "7-26",
    "8-1", "8-3", "8-5", "8-10", "8-15", "9-1", "9-2", "9-3", "9-4",
    "9-5", "9-6", "9-7", "9-8", "9-9", "9-22", "9-23", "9-28", "10-1",
    "10-3", "10-6", "10-15", "10-18", "10-19", "10-20", "11-6", "11-9",
    "11-11", "11-26", "12-8", "12-16", "12-20", "12-21", "12-22",
    "12-23", "12-25", "12-29"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  __FESTIVAL_VALUE,
  Vec<Vec<TaoFestivalRef>>,
  vec![
    vec![tao_festival::new_2(
      "天腊之辰",
      "天腊，此日五帝会于东方九炁青天"
    )],
    vec![
      tao_festival::new_1("郝真人圣诞"),
      tao_festival::new_1("孙真人圣诞")
    ],
    vec![tao_festival::new_1("孙祖清静元君诞")],
    vec![tao_festival::new_2(
      "举迁赏会",
      "此日上元赐福，天官同地水二官考校罪福"
    )],
    vec![tao_festival::new_1("玉皇上帝圣诞")],
    vec![tao_festival::new_1("关圣帝君飞升")],
    vec![
      tao_festival::new_1("上元天官圣诞"),
      tao_festival::new_1("老祖天师圣诞")
    ],
    vec![tao_festival::new_1("长春邱真人(邱处机)圣诞")],
    vec![tao_festival::new_1("许真君(许逊天师)圣诞")],
    vec![
      tao_festival::new_1("勾陈天皇大帝圣诞"),
      tao_festival::new_1("长春刘真人(刘渊然)圣诞")
    ],
    vec![
      tao_festival::new_1("土地正神诞"),
      tao_festival::new_1("姜太公圣诞")
    ],
    vec![tao_festival::new_1("文昌梓潼帝君圣诞")],
    vec![tao_festival::new_1("东华帝君圣诞")],
    vec![tao_festival::new_1("度人无量葛真君圣诞")],
    vec![tao_festival::new_1("太清道德天尊(太上老君)圣诞")],
    vec![tao_festival::new_1("慈航真人圣诞")],
    vec![tao_festival::new_1("谭祖(谭处端)长真真人圣诞")],
    vec![tao_festival::new_1("玄天上帝圣诞")],
    vec![tao_festival::new_1("眼光娘娘圣诞")],
    vec![
      tao_festival::new_1("天师张大真人圣诞"),
      tao_festival::new_1("财神赵公元帅圣诞")
    ],
    vec![
      tao_festival::new_1("三茅真君得道之辰"),
      tao_festival::new_1("中岳大帝圣诞")
    ],
    vec![
      tao_festival::new_1("王祖(王处一)玉阳真人圣诞"),
      tao_festival::new_1("后土娘娘圣诞")
    ],
    vec![tao_festival::new_1("太阳星君圣诞")],
    vec![tao_festival::new_1("子孙娘娘圣诞")],
    vec![tao_festival::new_1("天后妈祖圣诞")],
    vec![tao_festival::new_1("鬼谷先师诞")],
    vec![tao_festival::new_1("东岳大帝圣诞")],
    vec![tao_festival::new_1("长生谭真君成道之辰")],
    vec![tao_festival::new_1("何仙姑圣诞")],
    vec![tao_festival::new_1("吕祖纯阳祖师圣诞")],
    vec![tao_festival::new_1("钟离祖师圣诞")],
    vec![
      tao_festival::new_1("北极紫微大帝圣诞"),
      tao_festival::new_1("泰山圣母碧霞元君诞"),
      tao_festival::new_1("华佗神医先师诞")
    ],
    vec![tao_festival::new_1("眼光圣母娘娘诞")],
    vec![tao_festival::new_1("神农先帝诞")],
    vec![tao_festival::new_1("南极长生大帝圣诞")],
    vec![
      tao_festival::new_2("地腊之辰", "地腊，此日五帝会于南方三炁丹天"),
      tao_festival::new_1("南方雷祖圣诞"),
      tao_festival::new_1("地祗温元帅圣诞"),
      tao_festival::new_1("雷霆邓天君圣诞")
    ],
    vec![tao_festival::new_1("城隍爷圣诞")],
    vec![
      tao_festival::new_1("关圣帝君降神"),
      tao_festival::new_1("关平太子圣诞")
    ],
    vec![tao_festival::new_1("张天师圣诞")],
    vec![tao_festival::new_1("马祖丹阳真人圣诞")],
    vec![tao_festival::new_1("紫青白祖师圣诞")],
    vec![tao_festival::new_1("南斗星君下降")],
    vec![tao_festival::new_1("南斗星君下降")],
    vec![tao_festival::new_1("南斗星君下降")],
    vec![tao_festival::new_1("南斗星君下降")],
    vec![tao_festival::new_1("南斗星君下降")],
    vec![tao_festival::new_1("南斗星君下降")],
    vec![tao_festival::new_1("刘海蟾祖师圣诞")],
    vec![tao_festival::new_1("灵官王天君圣诞")],
    vec![tao_festival::new_1("慈航(观音)成道日")],
    vec![tao_festival::new_1("火神圣诞")],
    vec![
      tao_festival::new_1("南极大帝中方雷祖圣诞"),
      tao_festival::new_1("关圣帝君圣诞")
    ],
    vec![tao_festival::new_1("二郎真君圣诞")],
    vec![
      tao_festival::new_2(
        "道德腊之辰",
        "道德腊，此日五帝会于西方七炁素天"
      ),
      tao_festival::new_2(
        "庆生中会",
        "此日中元赦罪，地官同天水二官考校罪福"
      )
    ],
    vec![tao_festival::new_1("西方雷祖圣诞")],
    vec![tao_festival::new_1("中元地官大帝圣诞")],
    vec![tao_festival::new_1("王母娘娘圣诞")],
    vec![tao_festival::new_1("刘祖(刘处玄)长生真人圣诞")],
    vec![tao_festival::new_1("财帛星君文财神增福相公李诡祖圣诞")],
    vec![tao_festival::new_1("张三丰祖师圣诞")],
    vec![tao_festival::new_1("许真君飞升日")],
    vec![tao_festival::new_1("九天司命灶君诞")],
    vec![tao_festival::new_1("北方雷祖圣诞")],
    vec![tao_festival::new_1("北岳大帝诞辰")],
    vec![tao_festival::new_1("太阴星君诞")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![tao_festival::new_1("北斗九皇降世之辰")],
    vec![
      tao_festival::new_1("北斗九皇降世之辰"),
      tao_festival::new_1("斗姥元君圣诞"),
      tao_festival::new_1("重阳帝君圣诞"),
      tao_festival::new_1("玄天上帝飞升"),
      tao_festival::new_1("酆都大帝圣诞")
    ],
    vec![tao_festival::new_1("增福财神诞")],
    vec![tao_festival::new_1("萨翁真君圣诞")],
    vec![tao_festival::new_1("五显灵官马元帅圣诞")],
    vec![
      tao_festival::new_2(
        "民岁腊之辰",
        "民岁腊，此日五帝会于北方五炁黑天"
      ),
      tao_festival::new_1("东皇大帝圣诞")
    ],
    vec![tao_festival::new_1("三茅应化真君圣诞")],
    vec![tao_festival::new_1("天曹诸司五岳五帝圣诞")],
    vec![
      tao_festival::new_1("下元水官大帝圣诞"),
      tao_festival::new_2(
        "建生大会",
        "此日下元解厄，水官同天地二官考校罪福"
      )
    ],
    vec![tao_festival::new_1("地母娘娘圣诞")],
    vec![tao_festival::new_1("长春邱真君飞升")],
    vec![tao_festival::new_1("虚靖天师(即三十代天师弘悟张真人)诞")],
    vec![tao_festival::new_1("西岳大帝圣诞")],
    vec![tao_festival::new_1("湘子韩祖圣诞")],
    vec![tao_festival::new_1("太乙救苦天尊圣诞")],
    vec![tao_festival::new_1("北方五道圣诞")],
    vec![tao_festival::new_2(
      "王侯腊之辰",
      "王侯腊，此日五帝会于上方玄都玉京"
    )],
    vec![
      tao_festival::new_1("南岳大帝圣诞"),
      tao_festival::new_1("福德正神诞")
    ],
    vec![tao_festival::new_1("鲁班先师圣诞")],
    vec![tao_festival::new_1("天猷上帝圣诞")],
    vec![tao_festival::new_1("重阳祖师圣诞")],
    vec![tao_festival::new_2(
      "祭灶王",
      "最适宜谢旧年太岁，开启拜新年太岁"
    )],
    vec![
      tao_festival::new_1("玉帝巡天"),
      tao_festival::new_1("天神下降")
    ],
    vec![tao_festival::new_1("清静孙真君(孙不二)成道")]
  ]
);

fn str_to_map(raw: &str) -> HashMap<String, String> {
  let mut hm = HashMap::new();
  let v: Value = serde_json::from_str(raw).unwrap();
  v.as_object().unwrap().iter().for_each(|(key, value)| {
    hm.insert(key.clone(), value.as_str().unwrap().to_string());
  });
  hm
}
