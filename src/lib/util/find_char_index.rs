pub trait FindCharIndex {
  fn find_char_index(&self, pat: &str) -> Option<usize>;
  fn rfind_char_index(&self, pat: &str) -> Option<usize>;
  fn find_char_index_offset(
    &self,
    offset: usize,
    pat: &str,
  ) -> Option<usize>;
  fn slice_by_char_idx(&self, start: usize, end: usize) -> String;
}

impl FindCharIndex for String {
  fn find_char_index(&self, pat: &str) -> Option<usize> {
    self.find_char_index_offset(0, pat)
  }

  fn rfind_char_index(&self, pat: &str) -> Option<usize> {
    let pat_size = pat.chars().count();
    let mut offset = self.chars().count() - pat_size;
    while offset > 0 {
      let found = self.find_char_index_offset(offset, pat);
      if found.is_some() {
        return found;
      }
      offset = offset - pat_size;
    }
    None
  }

  fn find_char_index_offset(
    &self,
    offset: usize,
    pat: &str,
  ) -> Option<usize> {
    let src = self.chars().collect::<Vec<_>>();
    let pat_len = pat.chars().count();

    if offset < src.len() {
      for i in offset..(src.len() - pat_len + 1) {
        let slice: String = src[i..i + pat_len].iter().collect();
        if slice.as_str() == pat {
          return Some(i);
        }
      }
    }
    return None;
  }

  fn slice_by_char_idx(&self, start: usize, end: usize) -> String {
    self.chars().collect::<Vec<_>>()[start..end]
      .iter()
      .collect::<String>()
  }
}
