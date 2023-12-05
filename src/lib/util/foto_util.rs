use super::mmacro::{
  __static_funk, static_funk, static_vec_string_funk,
};
use crate::foto_festival::{FotoFestival, FotoFestivalRef};
#[allow(unused_imports)]
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;

///
/// 获取27星宿
///
/// ## Arguments
/// month: **i64** - 佛历月
/// day: **i64** - 佛历日
///
/// ## Returns
/// 星宿: String
///
#[allow(non_snake_case)]
pub fn get_XIU(month: i64, day: i64) -> String {
  XIU_27()[((XIU_OFFSET()[month as usize - 1] + day - 1)
    % XIU_27().len() as i64) as usize]
    .to_string()
}

// """
// 佛历工具
// """

// 观音斋日期
static_vec_string_funk!(
  DAY_ZHAI_GUAN_YIN,
  [
    "1-8", "2-7", "2-9", "2-19", "3-3", "3-6", "3-13", "4-22", "5-3",
    "5-17", "6-16", "6-18", "6-19", "6-23", "7-13", "8-16", "9-19",
    "9-23", "10-2", "11-19", "11-24", "12-25"
  ]
);

// 27星宿，佛教从印度传入中国，印度把28星宿改为27星宿，把牛宿(牛金牛)纳入了女宿(女土蝠)。
static_vec_string_funk!(
  XIU_27,
  [
    "角", "亢", "氐", "房", "心", "尾", "箕", "斗", "女", "虚", "危",
    "室", "壁", "奎", "娄", "胃", "昴", "毕", "觜", "参", "井", "鬼",
    "柳", "星", "张", "翼", "轸"
  ]
);

// 每月初一的27星宿偏移
static_funk!(
  XIU_OFFSET,
  Vec<i64>,
  vec![11, 13, 15, 17, 19, 21, 24, 0, 2, 4, 7, 9]
);

//
static_funk!(FESTIVAL, HashMap<String, Vec<FotoFestivalRef>>, {
  __FESTIVAL_KEY().iter().zip(__FESTIVAL_VAL().iter()).map(|(key, value)| {
    (key.to_string(), value.clone())
  }).collect::<HashMap<_,_>>()
});

__static_funk!(
  __FESTIVAL_KEY,
  Vec<String>,
  [
    "1-1", "1-3", "1-5", "1-6", "1-7", "1-8", "1-9", "1-13", "1-14",
    "1-15", "1-16", "1-19", "1-23", "1-25", "1-27", "1-28", "1-29",
    "1-30", "2-1", "2-2", "2-3", "2-6", "2-8", "2-11", "2-14", "2-15",
    "2-17", "2-18", "2-19", "2-21", "2-23", "2-25", "2-27", "2-28",
    "2-29", "2-30", "3-1", "3-3", "3-6", "3-8", "3-9", "3-12", "3-14",
    "3-15", "3-16", "3-19", "3-20", "3-23", "3-25", "3-27", "3-28",
    "3-29", "3-30", "4-1", "4-3", "4-4", "4-6", "4-7", "4-8", "4-14",
    "4-15", "4-16", "4-17", "4-18", "4-20", "4-23", "4-25", "4-27",
    "4-28", "4-29", "4-30", "5-1", "5-3", "5-5", "5-6", "5-7", "5-8",
    "5-11", "5-12", "5-13", "5-14", "5-15", "5-16", "5-17", "5-18",
    "5-22", "5-23", "5-25", "5-26", "5-27", "5-28", "5-29", "5-30",
    "6-1", "6-3", "6-5", "6-6", "6-8", "6-10", "6-14", "6-15", "6-19",
    "6-23", "6-24", "6-25", "6-27", "6-28", "6-29", "6-30", "7-1",
    "7-3", "7-5", "7-6", "7-7", "7-8", "7-10", "7-12", "7-13", "7-14",
    "7-15", "7-16", "7-18", "7-19", "7-22", "7-23", "7-25", "7-27",
    "7-28", "7-29", "7-30", "8-1", "8-3", "8-5", "8-6", "8-8", "8-10",
    "8-12", "8-14", "8-15", "8-16", "8-18", "8-23", "8-24", "8-25",
    "8-27", "8-28", "8-29", "8-30", "9-1", "9-3", "9-6", "9-8", "9-9",
    "9-10", "9-11", "9-13", "9-14", "9-15", "9-17", "9-19", "9-23",
    "9-25", "9-27", "9-28", "9-29", "9-30", "10-1", "10-3", "10-5",
    "10-6", "10-8", "10-10", "10-11", "10-14", "10-15", "10-16",
    "10-23", "10-25", "10-27", "10-28", "10-29", "10-30", "11-1",
    "11-3", "11-4", "11-6", "11-8", "11-11", "11-14", "11-15", "11-17",
    "11-19", "11-21", "11-23", "11-25", "11-26", "11-27", "11-28",
    "11-29", "11-30", "12-1", "12-3", "12-6", "12-7", "12-8", "12-12",
    "12-14", "12-15", "12-16", "12-19", "12-20", "12-21", "12-23",
    "12-24", "12-25", "12-27", "12-28", "12-29", "12-30"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(__FESTIVAL_VAL, Vec<Vec<FotoFestivalRef>>, {
  #[allow(non_snake_case)]
  let __DJ = "犯者夺纪";
  #[allow(non_snake_case)]
  let __JS = "犯者减寿";
  #[allow(non_snake_case)]
  let __SS = "犯者损寿";
  #[allow(non_snake_case)]
  let __XL = "犯者削禄夺纪";
  #[allow(non_snake_case)]
  let __JW = "犯者三年内夫妇俱亡";

  #[allow(non_snake_case)]
  let __Y = FotoFestival::new_1("杨公忌");
  #[allow(non_snake_case)]
  let __T = FotoFestival::new_3("四天王巡行", "", true);
  #[allow(non_snake_case)]
  let __D = FotoFestival::new_3("斗降", __DJ, true);
  #[allow(non_snake_case)]
  let __S = FotoFestival::new_3("月朔", __DJ, true);
  #[allow(non_snake_case)]
  let __W = FotoFestival::new_3("月望", __DJ, true);
  #[allow(non_snake_case)]
  let __H = FotoFestival::new_3("月晦", __JS, true);
  #[allow(non_snake_case)]
  let __L = FotoFestival::new_3("雷斋日", __JS, true);
  #[allow(non_snake_case)]
  let __J = FotoFestival::new_2("九毒日", "犯者夭亡，奇祸不测");
  #[allow(non_snake_case)]
  let __R =
    FotoFestival::new_4("人神在阴", "犯者得病", true, "宜先一日即戒");
  #[allow(non_snake_case)]
  let __M =
    FotoFestival::new_4("司命奏事", __JS, true, "如月小，即戒廿九");
  #[allow(non_snake_case)]
  let __HH =
    FotoFestival::new_4("月晦", __JS, true, "如月小，即戒廿九");

  vec![
    vec![
      FotoFestival::new_2("天腊，玉帝校世人神气禄命", __XL),
      __S.clone(),
    ],
    vec![FotoFestival::new_2("万神都会", __DJ), __D.clone()],
    vec![FotoFestival::new_1("五虚忌")],
    vec![FotoFestival::new_1("六耗忌"), __L.clone()],
    vec![FotoFestival::new_2("上会日", __SS)],
    vec![FotoFestival::new_2("五殿阎罗天子诞", __DJ), __T.clone()],
    vec![FotoFestival::new_2("玉皇上帝诞", __DJ)],
    vec![__Y.clone()],
    vec![FotoFestival::new_2("三元降", __JS), __T.clone()],
    vec![
      FotoFestival::new_2("三元降", __JS),
      FotoFestival::new_2("上元神会", __DJ),
      __W.clone(),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("三元降", __JS)],
    vec![FotoFestival::new_1("长春真人诞")],
    vec![FotoFestival::new_1("三尸神奏事"), __T.clone()],
    vec![
      __H.clone(),
      FotoFestival::new_2("天地仓开日", "犯者损寿，子带疾"),
    ],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![FotoFestival::new_2("一殿秦广王诞", __DJ), __S.clone()],
    vec![
      FotoFestival::new_2("万神都会", __DJ),
      FotoFestival::new_2("福德土地正神诞", "犯者得祸"),
    ],
    vec![FotoFestival::new_2("文昌帝君诞", __XL), __D.clone()],
    vec![FotoFestival::new_1("东华帝君诞"), __L.clone()],
    vec![
      FotoFestival::new_2("释迦牟尼佛出家", __DJ),
      FotoFestival::new_2("三殿宋帝王诞", __DJ),
      FotoFestival::new_2("张大帝诞", __DJ),
      __T.clone(),
    ],
    vec![__Y.clone()],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("释迦牟尼佛涅槃", __XL),
      FotoFestival::new_2("太上老君诞", __XL),
      FotoFestival::new_3("月望", __XL, true),
      __T.clone(),
    ],
    vec![FotoFestival::new_1("东方杜将军诞")],
    vec![
      FotoFestival::new_2("四殿五官王诞", __XL),
      FotoFestival::new_2("至圣先师孔子讳辰", __XL),
    ],
    vec![FotoFestival::new_2("观音大士诞", __DJ)],
    vec![FotoFestival::new_1("普贤菩萨诞")],
    vec![__T.clone()],
    vec![__H.clone()],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![FotoFestival::new_2("二殿楚江王诞", __DJ), __S.clone()],
    vec![FotoFestival::new_2("玄天上帝诞", __DJ), __D.clone()],
    vec![__L.clone()],
    vec![FotoFestival::new_2("六殿卞城王诞", __DJ), __T.clone()],
    vec![FotoFestival::new_2("牛鬼神出", "犯者产恶胎"), __Y.clone()],
    vec![FotoFestival::new_1("中央五道诞")],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("昊天上帝诞", __DJ),
      FotoFestival::new_2("玄坛诞", __DJ),
      __W.clone(),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("准提菩萨诞", __DJ)],
    vec![
      FotoFestival::new_1("中岳大帝诞"),
      FotoFestival::new_1("后土娘娘诞"),
      FotoFestival::new_1("三茅降"),
    ],
    vec![
      FotoFestival::new_2("天地仓开日", __SS),
      FotoFestival::new_1("子孙娘娘诞"),
    ],
    vec![__T.clone()],
    vec![__H.clone()],
    vec![FotoFestival::new_1("七殿泰山王诞"), __D.clone()],
    vec![
      __R.clone(),
      FotoFestival::new_2("苍颉至圣先师诞", __XL),
      FotoFestival::new_1("东岳大帝诞"),
    ],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![FotoFestival::new_2("八殿都市王诞", __DJ), __S.clone()],
    vec![__D.clone()],
    vec![
      FotoFestival::new_2("万神善会", "犯者失瘼夭胎"),
      FotoFestival::new_1("文殊菩萨诞"),
    ],
    vec![__L.clone()],
    vec![
      FotoFestival::new_2("南斗、北斗、西斗同降", __JS),
      __Y.clone(),
    ],
    vec![
      FotoFestival::new_2("释迦牟尼佛诞", __DJ),
      FotoFestival::new_2("万神善会", "犯者失瘼夭胎"),
      FotoFestival::new_2("善恶童子降", "犯者血死"),
      FotoFestival::new_1("九殿平等王诞"),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("纯阳祖师诞", __JS), __T.clone()],
    vec![__W.clone(), FotoFestival::new_1("钟离祖师诞"), __T.clone()],
    vec![FotoFestival::new_2("天地仓开日", __SS)],
    vec![FotoFestival::new_2("十殿转轮王诞", __DJ)],
    vec![
      FotoFestival::new_2("天地仓开日", __SS),
      FotoFestival::new_2("紫徽大帝诞", __SS),
    ],
    vec![FotoFestival::new_1("眼光圣母诞")],
    vec![__T.clone()],
    vec![__H.clone()],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![FotoFestival::new_2("南极长生大帝诞", __DJ), __S.clone()],
    vec![__D.clone()],
    vec![
      FotoFestival::new_2("地腊", __XL),
      FotoFestival::new_2("五帝校定生人官爵", __XL),
      __J.clone(),
      __Y.clone(),
    ],
    vec![__J.clone(), __L.clone()],
    vec![__J.clone()],
    vec![FotoFestival::new_1("南方五道诞"), __T.clone()],
    vec![
      FotoFestival::new_2("天地仓开日", __SS),
      FotoFestival::new_1("天下都城隍诞"),
    ],
    vec![FotoFestival::new_1("炳灵公诞")],
    vec![FotoFestival::new_2("关圣降", __XL)],
    vec![FotoFestival::new_2("夜子时为天地交泰", __JW), __T.clone()],
    vec![__W.clone(), __J.clone(), __T.clone()],
    vec![
      FotoFestival::new_2("九毒日", __JW),
      FotoFestival::new_2("天地元气造化万物之辰", __JW),
    ],
    vec![__J.clone()],
    vec![FotoFestival::new_1("张天师诞")],
    vec![FotoFestival::new_2("孝娥神诞", __DJ)],
    vec![__T.clone()],
    vec![__J.clone(), __H.clone()],
    vec![__J.clone()],
    vec![__J.clone(), __D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![__S.clone()],
    vec![
      FotoFestival::new_1("韦驮菩萨圣诞"),
      __D.clone(),
      __Y.clone(),
    ],
    vec![FotoFestival::new_2("南赡部洲转大轮", __SS)],
    vec![FotoFestival::new_2("天地仓开日", __SS), __L.clone()],
    vec![__T.clone()],
    vec![FotoFestival::new_1("金粟如来诞")],
    vec![__T.clone()],
    vec![__W.clone(), __T.clone()],
    vec![FotoFestival::new_2("观世音菩萨成道", __DJ)],
    vec![FotoFestival::new_2("南方火神诞", "犯者遭回禄"), __T.clone()],
    vec![
      FotoFestival::new_2("雷祖诞", __XL),
      FotoFestival::new_2("关帝诞", __XL),
    ],
    vec![__H.clone()],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![__S.clone(), __Y.clone()],
    vec![__D.clone()],
    vec![FotoFestival::new_4("中会日", __SS, false, "一作初七")],
    vec![__L.clone()],
    vec![
      FotoFestival::new_2("道德腊", __XL),
      FotoFestival::new_2("五帝校生人善恶", __XL),
      FotoFestival::new_2("魁星诞", __XL),
    ],
    vec![__T.clone()],
    vec![FotoFestival::new_4("阴毒日", "", false, "大忌")],
    vec![FotoFestival::new_1("长真谭真人诞")],
    vec![FotoFestival::new_2("大势至菩萨诞", __JS)],
    vec![FotoFestival::new_2("三元降", __JS), __T.clone()],
    vec![
      __W.clone(),
      FotoFestival::new_2("三元降", __DJ),
      FotoFestival::new_2("地官校籍", __DJ),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("三元降", __JS)],
    vec![FotoFestival::new_2("西王母诞", __DJ)],
    vec![FotoFestival::new_2("太岁诞", __DJ)],
    vec![FotoFestival::new_2("增福财神诞", __XL)],
    vec![__T.clone()],
    vec![__H.clone()],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__Y.clone(), __T.clone()],
    vec![
      FotoFestival::new_2("地藏菩萨诞", __DJ),
      __HH.clone(),
      __M.clone(),
      __T.clone(),
    ],
    vec![__S.clone(), FotoFestival::new_1("许真君诞")],
    vec![
      __D.clone(),
      FotoFestival::new_2("北斗诞", __XL),
      FotoFestival::new_2("司命灶君诞", "犯者遭回禄"),
    ],
    vec![FotoFestival::new_2("雷声大帝诞", __DJ)],
    vec![__L.clone()],
    vec![__T.clone()],
    vec![FotoFestival::new_1("北斗大帝诞")],
    vec![FotoFestival::new_1("西方五道诞")],
    vec![__T.clone()],
    vec![
      __W.clone(),
      FotoFestival::new_4("太明朝元", "犯者暴亡", false, "宜焚香守夜"),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("天曹掠刷真君降", "犯者贫夭")],
    vec![FotoFestival::new_4(
      "天人兴福之辰",
      "",
      false,
      "宜斋戒，存想吉事",
    )],
    vec![FotoFestival::new_1("汉恒候张显王诞"), __T.clone()],
    vec![FotoFestival::new_1("灶君夫人诞")],
    vec![__H.clone()],
    vec![
      __D.clone(),
      FotoFestival::new_2("至圣先师孔子诞", __XL),
      __Y.clone(),
    ],
    vec![__R.clone(), FotoFestival::new_1("四天会事")],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("诸神考校", "犯者夺算"),
      __HH.clone(),
      __M.clone(),
      __T.clone(),
    ],
    vec![
      __S.clone(),
      FotoFestival::new_2("南斗诞", __XL),
      FotoFestival::new_4(
        "北斗九星降世",
        __DJ,
        false,
        "此九日俱宜斋戒",
      ),
    ],
    vec![__D.clone(), FotoFestival::new_1("五瘟神诞")],
    vec![__L.clone()],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("斗母诞", __XL),
      FotoFestival::new_1("酆都大帝诞"),
      FotoFestival::new_1("玄天上帝飞升"),
    ],
    vec![FotoFestival::new_2("斗母降", __DJ)],
    vec![FotoFestival::new_1("宜戒")],
    vec![FotoFestival::new_1("孟婆尊神诞")],
    vec![__T.clone()],
    vec![__W.clone(), __T.clone()],
    vec![FotoFestival::new_2("金龙四大王诞", "犯者遭水厄")],
    vec![
      FotoFestival::new_2("日宫月宫会合", __JS),
      FotoFestival::new_2("观世音菩萨诞", __JS),
    ],
    vec![__T.clone()],
    vec![__H.clone(), __Y.clone()],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("药师琉璃光佛诞", "犯者危疾"),
      __HH.clone(),
      __M.clone(),
      __T.clone(),
    ],
    vec![
      __S.clone(),
      FotoFestival::new_2("民岁腊", __DJ),
      FotoFestival::new_2("四天王降", "犯者一年内死"),
    ],
    vec![__D.clone(), FotoFestival::new_1("三茅诞")],
    vec![
      FotoFestival::new_2("下会日", __JS),
      FotoFestival::new_2("达摩祖师诞", __JS),
    ],
    vec![__L.clone(), FotoFestival::new_2("天曹考察", __DJ)],
    vec![
      FotoFestival::new_4("佛涅槃日", "", false, "大忌色欲"),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("四天王降", "犯者一年内死")],
    vec![FotoFestival::new_1("宜戒")],
    vec![FotoFestival::new_2("三元降", __JS), __T.clone()],
    vec![
      __W.clone(),
      FotoFestival::new_2("三元降", __DJ),
      FotoFestival::new_2("下元水府校籍", __DJ),
      __T.clone(),
    ],
    vec![FotoFestival::new_2("三元降", __JS), __T.clone()],
    vec![__Y.clone(), __T.clone()],
    vec![__H.clone()],
    vec![__D.clone(), FotoFestival::new_1("北极紫徽大帝降")],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![__S.clone()],
    vec![__D.clone()],
    vec![FotoFestival::new_2("至圣先师孔子诞", __XL)],
    vec![FotoFestival::new_1("西岳大帝诞")],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("天地仓开日", __DJ),
      FotoFestival::new_2("太乙救苦天尊诞", __DJ),
    ],
    vec![__T.clone()],
    vec![
      FotoFestival::new_2("月望", "上半夜犯男死 下半夜犯女死"),
      FotoFestival::new_2("四天王巡行", "上半夜犯男死 下半夜犯女死"),
    ],
    vec![FotoFestival::new_1("阿弥陀佛诞")],
    vec![FotoFestival::new_2("太阳日宫诞", "犯者得奇祸")],
    vec![__Y.clone()],
    vec![FotoFestival::new_2("张仙诞", "犯者绝嗣"), __T.clone()],
    vec![FotoFestival::new_2("掠刷大夫降", "犯者遭大凶"), __H.clone()],
    vec![FotoFestival::new_1("北方五道诞")],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![__T.clone()],
    vec![__HH.clone(), __M.clone(), __T.clone()],
    vec![__S.clone()],
    vec![__D.clone()],
    vec![FotoFestival::new_2("天地仓开日", __JS), __L.clone()],
    vec![FotoFestival::new_2("掠刷大夫降", "犯者得恶疾")],
    vec![
      FotoFestival::new_2("王侯腊", __DJ),
      FotoFestival::new_1("释迦如来成佛之辰"),
      __T.clone(),
      FotoFestival::new_2("初旬内戊日，亦名王侯腊", __DJ),
    ],
    vec![FotoFestival::new_1("太素三元君朝真")],
    vec![__T.clone()],
    vec![__W.clone(), __T.clone()],
    vec![FotoFestival::new_1("南岳大帝诞")],
    vec![__Y.clone()],
    vec![FotoFestival::new_2("天地交道", "犯者促寿")],
    vec![FotoFestival::new_1("天猷上帝诞")],
    vec![FotoFestival::new_1("五岳诞降"), __T.clone()],
    vec![FotoFestival::new_2("司今朝天奏人善恶", "犯者得大祸")],
    vec![
      FotoFestival::new_2("三清玉帝同降，考察善恶", "犯者得奇祸"),
      __H.clone(),
    ],
    vec![__D.clone()],
    vec![__R.clone()],
    vec![FotoFestival::new_1("华严菩萨诞"), __T.clone()],
    vec![FotoFestival::new_2("诸神下降，察访善恶", "犯者男女俱亡")],
  ]
});

static_funk!(OTHER_FESTIVAL, HashMap<String, Vec<String>>, {
  let v: Value = serde_json::from_str(r#"{
    "1-1": ["弥勒菩萨圣诞"],
    "1-6": ["定光佛圣诞"],
    "2-8": ["释迦牟尼佛出家"],
    "2-15": ["释迦牟尼佛涅槃"],
    "2-19": ["观世音菩萨圣诞"],
    "2-21": ["普贤菩萨圣诞"],
    "3-16": ["准提菩萨圣诞"],
    "4-4": ["文殊菩萨圣诞"],
    "4-8": ["释迦牟尼佛圣诞"],
    "4-15": ["佛吉祥日"],
    "4-28": ["药王菩萨圣诞"],
    "5-13": ["伽蓝菩萨圣诞"],
    "6-3": ["韦驮菩萨圣诞"],
    "6-19": ["观音菩萨成道"],
    "7-13": ["大势至菩萨圣诞"],
    "7-15": ["佛欢喜日"],
    "7-24": ["龙树菩萨圣诞"],
    "7-30": ["地藏菩萨圣诞"],
    "8-15": ["月光菩萨圣诞"],
    "8-22": ["燃灯佛圣诞"],
    "9-9": ["摩利支天菩萨圣诞"],
    "9-19": ["观世音菩萨出家"],
    "9-30": ["药师琉璃光佛圣诞"],
    "10-5": ["达摩祖师圣诞"],
    "10-20": ["文殊菩萨出家"],
    "11-17": ["阿弥陀佛圣诞"],
    "11-19": ["日光菩萨圣诞"],
    "12-8": ["释迦牟尼佛成道"],
    "12-23": ["监斋菩萨圣诞"],
    "12-29": ["华严菩萨圣诞"]
  }"#).unwrap();
  let mut hm = HashMap::new();
  v.as_object().unwrap().iter().for_each(|(k, v)| {
    hm.insert(k.clone(), v.as_array().unwrap().iter().map(|vv| vv.as_str().unwrap().to_string()).collect::<Vec<_>>());
  });
  hm
});
