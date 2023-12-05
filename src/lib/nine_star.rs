use crate::util::{
  locked_ref_trait::LockRef, lunar_util::POSITION_DESC,
  mmacro::__static_funk, stringx::StringX,
};
use once_cell::sync::Lazy;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Debug)]
pub struct NineStar {
  __index: usize,
}

pub type NineStarRef = Arc<Mutex<NineStar>>;
pub trait NineStarRefHelper: LockRef {
  fn get_number(&self) -> String;
  fn get_color(&self) -> String;
  fn get_wu_xing(&self) -> String;
  fn get_position(&self) -> String;
  fn get_position_desc(&self) -> String;
  fn get_name_in_xuan_kong(&self) -> String;
  fn get_name_in_bei_dou(&self) -> String;
  fn get_name_in_qi_men(&self) -> String;
  fn get_name_in_tai_yi(&self) -> String;
  fn get_luck_in_qi_men(&self) -> String;
  fn get_luck_in_xuan_kong(&self) -> String;
  fn get_yin_yang_in_qi_men(&self) -> String;
  fn get_type_in_tai_yi(&self) -> String;
  fn get_ba_men_in_qi_men(&self) -> String;
  fn get_song_in_tai_yi(&self) -> String;
  fn get_index(&self) -> usize;
  fn to_string(&self) -> String;
  fn to_full_string(&self) -> String;
}

impl NineStarRefHelper for NineStarRef {
  fn get_number(&self) -> String {
    NUMBER()[self.as_locked_ref().__index].to_string()
  }

  fn get_color(&self) -> String {
    COLOR()[self.as_locked_ref().__index].to_string()
  }

  fn get_wu_xing(&self) -> String {
    WU_XING()[self.as_locked_ref().__index].to_string()
  }

  fn get_position(&self) -> String {
    POSITION()[self.as_locked_ref().__index].to_string()
  }

  fn get_position_desc(&self) -> String {
    POSITION_DESC().get(&self.get_position()).unwrap().clone()
  }

  fn get_name_in_xuan_kong(&self) -> String {
    NAME_XUAN_KONG()[self.as_locked_ref().__index].to_string()
  }

  fn get_name_in_bei_dou(&self) -> String {
    NAME_BEI_DOU()[self.as_locked_ref().__index].to_string()
  }

  fn get_name_in_qi_men(&self) -> String {
    NAME_QI_MEN()[self.as_locked_ref().__index].to_string()
  }

  fn get_name_in_tai_yi(&self) -> String {
    NAME_TAI_YI()[self.as_locked_ref().__index].to_string()
  }

  fn get_luck_in_qi_men(&self) -> String {
    LUCK_QI_MEN()[self.as_locked_ref().__index].to_string()
  }

  fn get_luck_in_xuan_kong(&self) -> String {
    LUCK_XUAN_KONG()[self.as_locked_ref().__index].to_string()
  }

  fn get_yin_yang_in_qi_men(&self) -> String {
    YIN_YANG_QI_MEN()[self.as_locked_ref().__index].to_string()
  }

  fn get_type_in_tai_yi(&self) -> String {
    TYPE_TAI_YI()[self.as_locked_ref().__index].to_string()
  }

  fn get_ba_men_in_qi_men(&self) -> String {
    BA_MEN_QI_MEN()[self.as_locked_ref().__index].to_string()
  }

  fn get_song_in_tai_yi(&self) -> String {
    SONG_TAI_YI()[self.as_locked_ref().__index].to_string()
  }

  fn get_index(&self) -> usize {
    self.as_locked_ref().__index
  }

  fn to_string(&self) -> String {
    let mut s = StringX::new("");
    s = s + self.get_number();
    s = s + self.get_color();
    s = s + self.get_wu_xing();
    s = s + self.get_name_in_bei_dou();
    s.get()
  }

  fn to_full_string(&self) -> String {
    let mut s = StringX::new("");
    s = s + self.get_number();
    s = s + self.get_color();
    s = s + self.get_wu_xing();
    s = s + " ";
    s = s + self.get_position();
    s = s + "(";
    s = s + self.get_position_desc();
    s = s + ") ";
    s = s + self.get_name_in_bei_dou();
    s = s + " 玄空[";
    s = s + self.get_name_in_xuan_kong();
    s = s + " ";
    s = s + self.get_luck_in_xuan_kong();
    s = s + "] 奇门[";
    s = s + self.get_name_in_qi_men();
    s = s + " ";
    s = s + self.get_luck_in_qi_men();
    if self.get_ba_men_in_qi_men().len() > 0 {
      s = s + " ";
      s = s + self.get_ba_men_in_qi_men();
      s = s + "门";
    }
    s = s + " ";
    s = s + self.get_yin_yang_in_qi_men();
    s = s + "] 太乙[";
    s = s + self.get_name_in_tai_yi();
    s = s + " ";
    s = s + self.get_type_in_tai_yi();
    s = s + "]";
    s.get()
  }
}

impl LockRef for NineStarRef {
  type Output<'a> = MutexGuard<'a, NineStar,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}

impl NineStar {
  pub fn from_index(index: usize) -> NineStarRef {
    let found = {
      __NINESTAR()
        .lock()
        .unwrap()
        .get(&index)
        .map_or(None, |v| Some(v.clone()))
    };

    if found.is_some() {
      return found.unwrap().clone();
    }

    {
      __NINESTAR()
        .lock()
        .unwrap()
        .insert(index, Self::__new(index));
    }

    let found = {
      __NINESTAR()
        .lock()
        .unwrap()
        .get(&index)
        .map_or(None, |v| Some(v.clone()))
    };

    assert!(found.is_some());

    return found.unwrap();
  }

  fn __new(__index: usize) -> Arc<Mutex<Self>> {
    Arc::new(Mutex::new(Self { __index }))
  }
}

__static_funk!(__NINESTAR, Arc<Mutex<HashMap<usize, NineStarRef>>>, {
  Arc::new(Mutex::new(HashMap::new()))
});

__static_funk!(
  NUMBER,
  Vec<String>,
  ["一", "二", "三", "四", "五", "六", "七", "八", "九"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  COLOR,
  Vec<String>,
  ["白", "黒", "碧", "绿", "黄", "白", "赤", "白", "紫"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  WU_XING,
  Vec<String>,
  ["水", "土", "木", "木", "土", "金", "金", "土", "火"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  POSITION,
  Vec<String>,
  ["坎", "坤", "震", "巽", "中", "乾", "兑", "艮", "离"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  NAME_BEI_DOU,
  Vec<String>,
  [
    "天枢", "天璇", "天玑", "天权", "玉衡", "开阳", "摇光", "洞明",
    "隐元"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  NAME_XUAN_KONG,
  Vec<String>,
  [
    "贪狼", "巨门", "禄存", "文曲", "廉贞", "武曲", "破军", "左辅",
    "右弼"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  NAME_QI_MEN,
  Vec<String>,
  [
    "天蓬", "天芮", "天冲", "天辅", "天禽", "天心", "天柱", "天任",
    "天英"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  BA_MEN_QI_MEN,
  Vec<String>,
  ["休", "死", "伤", "杜", "", "开", "惊", "生", "景"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  NAME_TAI_YI,
  Vec<String>,
  [
    "太乙", "摄提", "轩辕", "招摇", "天符", "青龙", "咸池", "太阴",
    "天乙"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  TYPE_TAI_YI,
  Vec<String>,
  [
    "吉神", "凶神", "安神", "安神", "凶神", "吉神", "凶神", "吉神",
    "吉神"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(SONG_TAI_YI, Vec<String>, ["门中太乙明，星官号贪狼，赌彩财喜旺，婚姻大吉昌，出入无阻挡，参谒见贤良，此行三五里，黑衣别阴阳。", "门前见摄提，百事必忧疑，相生犹自可，相克祸必临，死门并相会，老妇哭悲啼，求谋并吉事，尽皆不相宜，只可藏隐遁，若动伤身疾。", "出入会轩辕，凡事必缠牵，相生全不美，相克更忧煎，远行多不利，博彩尽输钱，九天玄女法，句句不虚言。", "招摇号木星，当之事莫行，相克行人阻，阴人口舌迎，梦寐多惊惧，屋响斧自鸣，阴阳消息理，万法弗违情。", "五鬼为天符，当门阴女谋，相克无好事，行路阻中途，走失难寻觅，道逢有尼姑，此星当门值，万事有灾除。", "神光跃青龙，财气喜重重，投入有酒食，赌彩最兴隆，更逢相生旺，休言克破凶，见贵安营寨，万事总吉同。", "吾将为咸池，当之尽不宜，出入多不利，相克有灾情，赌彩全输尽，求财空手回，仙人真妙语，愚人莫与知，动用虚惊退，反复逆风吹。", "坐临太阴星，百祸不相侵，求谋悉成就，知交有觅寻，回风归来路，恐有殃伏起，密语中记取，慎乎莫轻行。", "迎来天乙星，相逢百事兴，运用和合庆，茶酒喜相迎，求谋并嫁娶，好合有天成，祸福如神验，吉凶甚分明。"].iter().map(|c| c.to_string()).collect::<Vec<_>>());
__static_funk!(
  LUCK_XUAN_KONG,
  Vec<String>,
  ["吉", "凶", "凶", "吉", "凶", "吉", "凶", "吉", "吉"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
__static_funk!(
  LUCK_QI_MEN,
  Vec<String>,
  [
    "大凶", "大凶", "小吉", "大吉", "大吉", "大吉", "小凶", "小吉",
    "小凶"
  ]
  .iter()
  .map(|c| c.to_string())
  .collect::<Vec<_>>()
);
__static_funk!(
  YIN_YANG_QI_MEN,
  Vec<String>,
  ["阳", "阴", "阳", "阳", "阳", "阴", "阴", "阳", "阴"]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<_>>()
);
