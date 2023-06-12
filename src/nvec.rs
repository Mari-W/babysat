use std::{
  iter::Zip,
  ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct NVec<T> {
  max_index: usize,
  #[allow(dead_code)]
  data: Box<[T]>,
  p: *mut T,
}

#[derive(Debug)]
pub struct NVecIterator<'a, T> {
  array: &'a NVec<T>,
  current: isize,
  up: bool,
}

impl<T> NVec<T> {
  pub fn new(max_index: usize) -> Self
  where
    T: Default + Clone,
  {
    let mut data = vec![T::default(); max_index * 2 + 1].into_boxed_slice();
    let p = unsafe { data.as_mut_ptr().add(max_index) };
    Self { max_index, data, p }
  }

  pub fn iter_pos(&self) -> NVecIterator<'_, T> {
    NVecIterator {
      array: self,
      current: 1,
      up: true,
    }
  }

  pub fn iter_neg(&self) -> NVecIterator<'_, T> {
    NVecIterator {
      array: self,
      current: -1,
      up: false,
    }
  }

  pub fn iter_zipped(&self) -> Zip<NVecIterator<'_, T>, NVecIterator<'_, T>> {
    self.iter_pos().zip(self.iter_neg())
  }
}

impl<T> Index<isize> for NVec<T> {
  type Output = T;

  #[inline]
  fn index(&self, index: isize) -> &Self::Output {
    debug_assert!(index.unsigned_abs() <= self.max_index && index != 0);
    unsafe { &*self.p.offset(index) }
  }
}

impl<T> IndexMut<isize> for NVec<T> {
  #[inline]
  fn index_mut(&mut self, index: isize) -> &mut Self::Output {
    debug_assert!(index.unsigned_abs() <= self.max_index && index != 0);
    unsafe { &mut *self.p.offset(index) }
  }
}

impl<'a, T> Iterator for NVecIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.up && self.current <= self.array.max_index as isize {
      let result = Some(&self.array[self.current]);
      self.current += 1;
      result
    } else if !self.up && self.current >= -(self.array.max_index as isize) {
      let result = Some(&self.array[self.current]);
      self.current -= 1;
      result
    } else {
      None
    }
  }
}
