use crate::{lunar::{LunarRefHelper, self}, solar::{SolarRefHelper, self}};

#[test]
fn lunar_test0() {
  let date = lunar::from_ymd_hms(2019, 3, 27, 0, 0, 0);
  assert_eq!("二〇一九年三月廿七", date.to_string());
  assert_eq!("二〇一九年三月廿七 己亥(猪)年 戊辰(龙)月 戊戌(狗)日 子(鼠)时 纳音[平地木 大林木 平地木 桑柘木] 星期三 西方白虎 星宿[参水猿](吉) 彭祖百忌[戊不受田田主不祥 戌不吃犬作怪上床] 喜神方位[巽](东南) 阳贵神方位[艮](东北) 阴贵神方位[坤](西南) 福神方位[艮](东北) 财神方位[坎](正北) 冲[(壬辰)龙] 煞[北]", date.to_full_string());
  assert_eq!("2019-05-01", date.get_solar().to_string());
  assert_eq!("2019-05-01 00:00:00 星期三 (劳动节) 金牛座", date.get_solar().to_full_string())
}
#[test]
fn lunar_test1() {
  let solar = solar::from_ymdhms(100, 1, 1, 12, 0, 0);
  assert_eq!("九九年腊月初二", solar.get_lunar().to_string());
}

#[test]
fn lunar_test2() {
  let solar = solar::from_ymdhms(3218, 12, 31, 12, 0, 0);
  assert_eq!("三二一八年冬月廿二", solar.get_lunar().to_string());
}

#[test]
fn lunar_test3() {
  let lunar = lunar::from_ymd_hms(5, 1, 6, 12, 0, 0);
  assert_eq!("0005-02-03", lunar.get_solar().to_string());
}

#[test]
fn lunar_test4() {
  let lunar = lunar::from_ymd_hms(9997, 12, 21, 12, 0, 0);
  assert_eq!("9998-01-11", lunar.get_solar().to_string());
}

#[test]
fn lunar_test5() {
  let lunar = lunar::from_ymd_hms(1905, 1, 1, 12, 0, 0);
  assert_eq!("1905-02-04", lunar.get_solar().to_string());
}

#[test]
fn lunar_test6() {
  let lunar = lunar::from_ymd_hms(2038, 12, 29, 12, 0, 0);
  assert_eq!("2039-01-23", lunar.get_solar().to_string());
}

#[test]
fn lunar_test7() {
  let lunar = lunar::from_ymd_hms(2020, -4, 2, 13, 0, 0);
  assert_eq!("二〇二〇年闰四月初二", lunar.to_string());
  assert_eq!("2020-05-24", lunar.get_solar().to_string());
}
#[test]
fn lunar_test8() {
  let lunar = lunar::from_ymd_hms(2020, 12, 10, 13, 0, 0);
  assert_eq!("二〇二〇年腊月初十", lunar.to_string());
  assert_eq!("2021-01-22", lunar.get_solar().to_string());
}

#[test]
fn lunar_test9() {
  let lunar = lunar::from_ymd_hms(1500, 1, 1, 12, 0, 0);
  assert_eq!("1500-01-31", lunar.get_solar().to_string());
}

#[test]
fn lunar_test10() {
  let lunar = lunar::from_ymd_hms(1500, 12, 29, 12, 0, 0);
  assert_eq!("1501-01-18", lunar.get_solar().to_string());
}

#[test]
fn lunar_test11() {
  let solar = solar::from_ymdhms(1500, 1, 1, 12, 0, 0);
  assert_eq!("一四九九年腊月初一", solar.get_lunar().to_string());
}

#[test]
fn lunar_test12() {
  let solar = solar::from_ymdhms(1500, 12, 31, 12, 0, 0);
  assert_eq!("一五〇〇年腊月十一", solar.get_lunar().to_string());
}
#[test]
fn lunar_test13() {
  let solar = solar::from_ymdhms(1582, 10, 4, 12, 0, 0);
  assert_eq!("一五八二年九月十八", solar.get_lunar().to_string());
}

#[test]
fn lunar_test14() {
  let solar = solar::from_ymdhms(1582, 10, 15, 12, 0, 0);
  assert_eq!("一五八二年九月十九", solar.get_lunar().to_string());
}

#[test]
fn lunar_test15() {
  let lunar = lunar::from_ymd_hms(1582, 9, 18, 12, 0, 0);
  assert_eq!("1582-10-04", lunar.get_solar().to_string());
}

#[test]
fn lunar_test16() {
  let lunar = lunar::from_ymd_hms(1582, 9, 19, 12, 0, 0);
  assert_eq!("1582-10-15", lunar.get_solar().to_string());
}

#[test]
fn lunar_test17() {
  let lunar = lunar::from_ymd_hms(2019, 12, 12, 11, 22, 0);
  assert_eq!("2020-01-06", lunar.get_solar().to_string());
}

#[test]
fn lunar_test18() {
  let lunar = lunar::from_ymd(2021, 12, 29);
  assert_eq!("除夕", lunar.get_festivals()[0]);
}

#[test]
fn lunar_test19() {
  let lunar = lunar::from_ymd(2020, 12, 30);
  assert_eq!("除夕", lunar.get_festivals()[0]);
}

#[test]
fn lunar_test20() {
  let lunar = lunar::from_ymd(2020, 12, 29);
  assert_eq!(0, lunar.get_festivals().len());
}

#[test]
fn lunar_test21() {
  let solar = solar::from_ymd(2022, 1, 31);
  let lunar = solar.get_lunar();
  assert_eq!("除夕", lunar.get_festivals()[0]);
}

#[test]
fn lunar_test22() {
  let lunar = lunar::from_ymd(2033, -11, 1);
  assert_eq!("2033-12-22", lunar.get_solar().to_ymd());
}
#[test]
fn lunar_test25() {
  let solar = solar::from_ymdhms(2021, 6, 7, 21, 18, 0);
  assert_eq!("二〇二一年四月廿七", solar.get_lunar().to_string());
}

#[test]
fn lunar_test26() {
  let lunar = lunar::from_ymd_hms(2021, 6, 7, 21, 18, 0);
  assert_eq!("2021-07-16", lunar.get_solar().to_string());
}
#[test]
fn lunar_test26_1() {
  let solar = solar::from_ymdhms(2020, 1, 10, 12, 0, 0);
  let lunar = solar.get_lunar();
  for i in -1..1 {
    assert_eq!(solar.next(i, None).get_lunar().to_full_string(), lunar.next(i).to_full_string())
  }
}

#[test]
fn lunar_test27() {
  let solar = solar::from_ymd(1989, 4, 28);
  assert_eq!(23, solar.get_lunar().get_day());
}

#[test]
fn lunar_test28() {
  let solar = solar::from_ymd(1990, 10, 8);
  assert_eq!("乙酉", solar.get_lunar().get_month_in_gan_zhi_exact());
}

#[test]
fn lunar_test29() {
  let solar = solar::from_ymd(1990, 10, 9);
  assert_eq!("丙戌", solar.get_lunar().get_month_in_gan_zhi_exact());
}

#[test]
fn lunar_test30() {
  let solar = solar::from_ymd(1990, 10, 8);
  assert_eq!("丙戌", solar.get_lunar().get_month_in_gan_zhi());
}

#[test]
fn lunar_test31() {
  let solar = solar::from_ymdhms(1987, 4, 17, 9, 0, 0);
  assert_eq!("一九八七年三月二十", solar.get_lunar().to_string());
}

#[test]
fn lunar_test32() {
  let lunar = lunar::from_ymd(2034, 1, 1);
  assert_eq!("2034-02-19", lunar.get_solar().to_ymd());
}

#[test]
fn lunar_test33() {
  let lunar = lunar::from_ymd(2033, 12, 1);
  assert_eq!("2034-01-20", lunar.get_solar().to_ymd());
}

#[test]
fn lunar_test34() {
  let lunar = lunar::from_ymd(37, -12, 1);
  assert_eq!("闰腊", lunar.get_month_in_chinese());
}

#[test]
fn lunar_test36() {
  let solar = solar::from_ymd(5553, 1, 22);
  assert_eq!("五五五二年闰腊月初二", solar.get_lunar().to_string());
}

#[test]
fn lunar_test37() {
  let solar = solar::from_ymd(7013, 12, 24);
  assert_eq!("七〇一三年闰冬月初四", solar.get_lunar().to_string());
}

#[test]
fn lunar_test38() {
  let lunar = lunar::from_ymd(7013, -11, 4);
  assert_eq!("7013-12-24", lunar.get_solar().to_string());
}

#[test]
fn lunar_test39() {
  let solar = solar::from_ymd(1987, 4, 12);
  let lunar = solar.get_lunar();
  assert_eq!("一九八七年三月十五", lunar.to_string());
}

#[test]
fn lunar_test40() {
  let solar = solar::from_ymd(1987, 4, 13);
  let lunar = solar.get_lunar();
  assert_eq!("一九八七年三月十六", lunar.to_string());
}

#[test]
fn lunar_test41() {
  let solar = solar::from_ymd(4, 2, 10);
  let lunar = solar.get_lunar();
  assert_eq!("鼠", lunar.get_year_sheng_xiao());
}

#[test]
fn lunar_test42() {
  let solar = solar::from_ymd(4, 2, 9);
  let lunar = solar.get_lunar();
  assert_eq!("猪", lunar.get_year_sheng_xiao());
}

#[test]
fn lunar_test43() {
  let solar = solar::from_ymd(2017, 2, 15);
  let lunar = solar.get_lunar();
  assert_eq!("子命互禄 辛命进禄", lunar.get_day_lu());
}

#[test]
fn lunar_test44() {
  let solar = solar::from_ymd(2017, 2, 16);
  let lunar = solar.get_lunar();
  assert_eq!("寅命互禄", lunar.get_day_lu())
}

#[test]
fn lunar_test48() {
  let solar = solar::from_ymd(2021, 11, 13);
  let lunar = solar.get_lunar();
  assert_eq!("碓磨厕 外东南", lunar.get_day_position_tai());
}

#[test]
fn lunar_test49() {
  let solar = solar::from_ymd(2021, 11, 12);
  let lunar = solar.get_lunar();
  assert_eq!("占门碓 外东南", lunar.get_day_position_tai());
}

#[test]
fn lunar_test50() {
  let solar = solar::from_ymd(2021, 11, 13);
  let lunar = solar.get_lunar();
  assert_eq!("西南", lunar.get_day_position_fu_desc(None));
}

#[test]
fn lunar_test51() {
  let solar = solar::from_ymd(2021, 11, 12);
  let lunar = solar.get_lunar();
  assert_eq!("正北", lunar.get_day_position_fu_desc(None));
}

#[test]
fn lunar_test52() {
  let solar = solar::from_ymd(2011, 11, 12);
  let lunar = solar.get_lunar();
  assert_eq!("厨灶厕 外西南", lunar.get_day_position_tai());
}

#[test]
fn lunar_test53() {
  let solar = solar::from_ymd(1722, 9, 25);
  let lunar = solar.get_lunar();
  assert_eq!("秋社", lunar.get_other_festivals()[0]);
}

#[test]
fn lunar_test54() {
  let solar = solar::from_ymd(840, 9, 14);
  let lunar = solar.get_lunar();
  assert_eq!("秋社", lunar.get_other_festivals()[0]);
}

#[test]
fn lunar_test55() {
  let solar = solar::from_ymd(2022, 3, 16);
  let lunar = solar.get_lunar();
  assert_eq!("春社", lunar.get_other_festivals()[0]);
}

#[test]
fn lunar_test56() {
  let solar = solar::from_ymd(2021, 3, 21);
  let lunar = solar.get_lunar();
  assert_eq!("春社", lunar.get_other_festivals()[0]);
}

#[test]
fn lunar_test57() {
  assert_eq!("1582-10-04", lunar::from_ymd(1582, 9, 18).get_solar().to_ymd());
}

#[test]
fn lunar_test58() {
  assert_eq!("1582-10-15", lunar::from_ymd(1582, 9, 19).get_solar().to_ymd());
}

#[test]
fn lunar_test59() {
  assert_eq!("1518-02-10", lunar::from_ymd(1518, 1, 1).get_solar().to_ymd());
}

#[test]
fn lunar_test60() {
  assert_eq!("0793-02-15", lunar::from_ymd(793, 1, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test61() {
  assert_eq!("2025-07-25", lunar::from_ymd(2025, -6, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test62() {
  assert_eq!("2025-06-25", lunar::from_ymd(2025, 6, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test63() {
  assert_eq!("0193-02-19", lunar::from_ymd(193, 1, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test64() {
  assert_eq!("0041-02-20", lunar::from_ymd(41, 1, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test65() {
  assert_eq!("0554-02-18", lunar::from_ymd(554, 1, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test66() {
  assert_eq!("1070-02-14", lunar::from_ymd(1070, 1, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test67() {
  assert_eq!("1537-02-10", lunar::from_ymd(1537, 1, 1).get_solar().to_ymd());
}
#[test]
fn lunar_test68() {
  assert_eq!("九一七年闰十月十四", solar::from_ymd(917, 12, 1).get_lunar().to_string());
}
#[test]
fn lunar_test69() {
  assert_eq!("九一七年冬月十五", solar::from_ymd(917, 12, 31).get_lunar().to_string());
}
#[test]
fn lunar_test70() {
  assert_eq!("九一七年冬月十六", solar::from_ymd(918, 1, 1).get_lunar().to_string());
}
