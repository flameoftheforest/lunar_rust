use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct TimeElapsed {
  pub n_start: NaiveDateTime,
  pub n_end: NaiveDateTime,
  pub n_tag: String,
}

impl TimeElapsed {
  pub fn start(tag: &str) -> Self {
    let now = Local::now().naive_local();
    Self {
      n_start: now,
      n_end: now,
      n_tag: tag.to_string(),
    }
  }

  pub fn start_print(tag: &str) -> Self {
    println!("--- START{} ---", {
      match tag.len() {
        0 => "".to_string(),
        _ => format!(" {}", tag),
      }
    });
    Self::start(tag)
  }

  pub fn end(&mut self) -> i64 {
    self.n_end = Local::now().naive_local();
    self.delta()
  }

  pub fn delta(&self) -> i64 {
    (self.n_end - self.n_start).num_milliseconds()
  }

  pub fn end_print_delta(&mut self, restart: bool) {
    self.end();
    println!("time taken {} ms", self.delta());
    println!("--- END{} ---", {
      match self.n_tag.len() {
        0 => "".to_string(),
        _ => format!(" {}", self.n_tag),
      }
    });
    if restart {
      let now = Local::now().naive_local();
      self.n_start = now;
      self.n_end = now;
      println!("--- START{} ---", {
        match self.n_tag.len() {
          0 => "".to_string(),
          _ => format!(" {}", self.n_tag),
        }
      });
    }
  }
}
