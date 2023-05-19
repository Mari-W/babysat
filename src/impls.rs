use std::{fmt::Display, ops::Index};

use crate::{
  data::{Assignment, Clause, Cnf},
  nvec::NVec,
};

/// iterating a clause iterates the literals
impl IntoIterator for Clause {
  type Item = isize;

  type IntoIter = <Vec<isize> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.literals.into_iter()
  }
}

/// iterating a clause iterates the literals
impl<'a> IntoIterator for &'a Clause {
  type Item = &'a isize;

  type IntoIter = std::slice::Iter<'a, isize>;

  fn into_iter(self) -> Self::IntoIter {
    self.literals.iter()
  }
}

/// iterating a cnf iterates the clauses
impl IntoIterator for Cnf {
  type Item = Clause;

  type IntoIter = <Vec<Clause> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.clauses.into_iter()
  }
}

/// iterating a cnf iterates the clauses
impl<'a> IntoIterator for &'a Cnf {
  type Item = &'a Clause;

  type IntoIter = std::slice::Iter<'a, Clause>;

  fn into_iter(self) -> Self::IntoIter {
    self.clauses.iter()
  }
}

/// indexing a clause results in a literal
impl Index<usize> for Clause {
  type Output = isize;

  fn index(&self, idx: usize) -> &Self::Output {
    &self.literals[idx]
  }
}

/// pretty printing clauses
impl Display for Clause {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({})",
      self
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ∨ ")
    )
  }
}

/// pretty printing cnf
impl Display for Cnf {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({})",
      self
        .into_iter()
        .map(|c| format!("{}", c))
        .collect::<Vec<String>>()
        .join(" ∧ ")
    )
  }
}

/// pretty printing assignment
impl Display for NVec<Assignment> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "v {} 0",
      self
        .into_iter()
        .enumerate()
        .map(|(i, a)| match a.0 {
          Assignment::Unassigned | Assignment::True => (i + 1).to_string(),
          Assignment::False => format!("-{}", (i + 1)),
        })
        .collect::<Vec<String>>()
        .join(" ∨ ")
    )
  }
}
