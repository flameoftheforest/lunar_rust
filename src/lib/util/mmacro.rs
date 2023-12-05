macro_rules! static_funk {
  ($name:ident, $ret_typ:ty, $initiator:expr) => {
    pub fn $name() -> $ret_typ {
      static INSTANCE: once_cell::sync::Lazy<$ret_typ> =
        once_cell::sync::Lazy::new(|| $initiator);
      INSTANCE.clone()
    }
  };
}

macro_rules! __static_funk {
  ($name:ident, $ret_typ:ty, $initiator:expr) => {
    fn $name() -> $ret_typ {
      static INSTANCE: once_cell::sync::Lazy<$ret_typ> =
        once_cell::sync::Lazy::new(|| $initiator);
      INSTANCE.clone()
    }
  };
}

macro_rules! static_vec_string_funk {
  ($name:ident, $initiator:expr) => {
    static_funk!($name, Vec<String>, {
      $initiator.iter().map(|c| c.to_string()).collect::<Vec<_>>()
    });
  };
}

pub(crate) use __static_funk;
pub(crate) use static_funk;
pub(crate) use static_vec_string_funk;
