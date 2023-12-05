use crate::util::find_char_index::FindCharIndex;

#[test]
fn chinese_test() {
  let gz = "甲午".to_string();
  let g = gz.slice_by_char_idx(0, 1);
  let z = gz.slice_by_char_idx(1, gz.chars().count());
  assert_eq!(g, "甲".to_string());
  assert_eq!(z, "午".to_string());
}