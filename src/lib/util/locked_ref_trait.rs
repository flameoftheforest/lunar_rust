pub trait LockRef {
  type Output<'a>
  where
    Self: 'a;
  fn as_locked_ref<'a>(&'a self) -> Self::Output<'a>;
}
