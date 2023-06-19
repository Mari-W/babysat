use std::fmt::Display;

use crate::{
  data::{Assignment, Clause, Cnf},
  nvec::NVec,
};

/// iterating a clause iterates the literals
impl<'a> IntoIterator for &'a Clause {
  type Item = &'a isize;

  type IntoIter = std::slice::Iter<'a, isize>;

  fn into_iter(self) -> Self::IntoIter {
    self.literals.iter()
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
        .skip(self.length + 1)
        .enumerate()
        .map(|(i, a)| match a {
          Assignment::Unassigned | Assignment::True => (i + 1).to_string(),
          Assignment::False => format!("-{}", (i + 1)),
        })
        .collect::<Vec<String>>()
        .join(" ∨ ")
    )
  }
}
