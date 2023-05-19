use std::ops::{Index, IndexMut};

/// a negative indexable vector
///
/// currently naively represented by a vector of tuples
/// and some arithmetics on indexing
#[derive(Debug)]
pub struct NVec<T>(Vec<(T, T)>);

impl<T> NVec<T> {
  /// constructs a new negative indexable vector of size `size`
  /// filled with the default value of a type (e.g 0 for i64)
  pub fn new(size: usize) -> NVec<T>
  where
    T: Default + Clone,
  {
    NVec(vec![(T::default(), T::default()); size])
  }

  /// the length of the vector
  pub fn len(&self) -> usize {
    self.0.len()
  }
}

/// iterate the positive and negative index as tuples
impl<T> IntoIterator for NVec<T> {
  type Item = (T, T);

  type IntoIter = <Vec<(T, T)> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

/// iterate the positive and negative index as tuples
impl<'a, T> IntoIterator for &'a NVec<T> {
  type Item = &'a (T, T);

  type IntoIter = std::slice::Iter<'a, (T, T)>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

/// index by integers
impl<T> Index<isize> for NVec<T> {
  type Output = T;

  fn index(&self, idx: isize) -> &Self::Output {
    debug_assert!(idx != 0);

    // the index is the absolute value of the possibly negative index
    // shifted to the left by one
    let n_idx: usize = (idx.abs() - 1)
      .try_into()
      .expect("unreasonably large index");
    // the first or second tuple is chosen by the sign of the idx
    if idx >= 0 {
      &self.0[n_idx].0
    } else {
      &self.0[n_idx].1
    }
  }
}

/// index by integers
impl<T> IndexMut<isize> for NVec<T> {
  fn index_mut(&mut self, idx: isize) -> &mut Self::Output {
    debug_assert!(idx != 0);

    // the index is the absolute value of the possibly negative index
    // shifted to the left by one
    let n_idx: usize = (idx.abs() - 1)
      .try_into()
      .expect("unreasonably large index");
    // the first or second tuple is chosen by the sign of the idx
    if idx >= 0 {
      &mut self.0[n_idx].0
    } else {
      &mut self.0[n_idx].1
    }
  }
}
