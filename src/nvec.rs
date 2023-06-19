use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct NVec<T> {
  pub length: usize,
  data: Box<[T]>,
}

impl<T> NVec<T> {
  pub fn new(length: usize) -> Self
  where
    T: Default + Clone,
  {
    let data = vec![T::default(); length * 2 + 1].into_boxed_slice();
    Self { length, data }
  }
}

impl<T> Index<isize> for NVec<T> {
  type Output = T;

  #[inline]
  fn index(&self, index: isize) -> &Self::Output {
    debug_assert!(index.unsigned_abs() <= self.length && index != 0);
    &self.data[(self.length as isize - index) as usize]
  }
}

impl<T> IndexMut<isize> for NVec<T> {
  #[inline]
  fn index_mut(&mut self, index: isize) -> &mut Self::Output {
    debug_assert!(index.unsigned_abs() <= self.length && index != 0);
    &mut self.data[(self.length as isize - index) as usize]
  }
}

impl<'a, T> IntoIterator for &'a NVec<T> {
  type Item = &'a T;
  type IntoIter = std::slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.iter()
  }
}
