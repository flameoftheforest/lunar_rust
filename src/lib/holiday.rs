use std::sync::{Arc, Mutex, MutexGuard};

use crate::util::locked_ref_trait::LockRef;

///
/// 节假日
///
#[derive(Clone, Debug)]
pub struct Holiday {
  __day: String,
  __name: String,
  __work: bool,
  __target: String,
}

pub type HolidayRef = Arc<Mutex<Holiday>>;

pub trait HolidayRefHelper: LockRef {
  fn get_day(&self) -> String;
  fn get_name(&self) -> String;
  fn is_work(&self) -> bool;
  fn get_target(&self) -> String;
  fn set_day(&mut self, day: &str);
  fn set_name(&mut self, name: &str);
  fn set_work(&mut self, work: bool);
  fn set_target(&mut self, target: &str);
  fn to_string(&self) -> String;
}

impl Holiday {
  fn __ymd(s: &str) -> String {
    match s.find("-").is_some() {
      true => s.to_string(),
      _ => {
        let c = s.chars().collect::<Vec<_>>();
        format!(
          "{}-{}-{}",
          c[0..4].to_vec().into_iter().collect::<String>(),
          c[4..6].to_vec().into_iter().collect::<String>(),
          c[6..].to_vec().into_iter().collect::<String>()
        )
      }
    }
  }

  pub fn default(
    day: &str,
    name: &str,
    work: bool,
    target: &str,
  ) -> HolidayRef {
    Arc::new(Mutex::new(Self {
      __day: day.to_string(),
      __name: name.to_string(),
      __work: work,
      __target: Self::__ymd(target),
    }))
  }
}

impl HolidayRefHelper for HolidayRef {
  #[allow(dead_code)]
  fn get_day(&self) -> String {
    self.as_locked_ref().__day.clone()
  }

  fn get_name(&self) -> String {
    self.as_locked_ref().__name.clone()
  }

  fn is_work(&self) -> bool {
    self.as_locked_ref().__work
  }

  fn get_target(&self) -> String {
    self.as_locked_ref().__target.clone()
  }

  fn set_day(&mut self, day: &str) {
    self.as_locked_ref().__day = day.to_string();
  }

  fn set_name(&mut self, name: &str) {
    self.as_locked_ref().__name = name.to_string();
  }

  fn set_work(&mut self, work: bool) {
    self.as_locked_ref().__work = work;
  }

  fn set_target(&mut self, target: &str) {
    self.as_locked_ref().__target = Holiday::__ymd(target);
  }

  fn to_string(&self) -> String {
    format!(
      "{} {}{} {}",
      self.get_day(),
      self.get_name(),
      {
        match self.is_work() {
          true => "调休",
          _ => "",
        }
      },
      self.get_target()
    )
  }
}

impl LockRef for HolidayRef {
  type Output<'a> = MutexGuard<'a, Holiday,> where Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a> {
    self.lock().unwrap()
  }
}
