pub trait ToVec {
  fn to_vec_hex_str(&self) -> Vec<String>;
}

impl ToVec for String {
  fn to_vec_hex_str(&self) -> Vec<String> {
    self
      .chars()
      .collect::<Vec<_>>()
      .chunks(2)
      .map(|c| c.iter().collect::<String>())
      .collect::<Vec<_>>()
  }
}
