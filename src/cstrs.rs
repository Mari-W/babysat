use crate::{
  data::{Clause, Cnf, State},
  nvec::NVec,
};

#[cfg(debug_assertions)]
use crate::data::Stats;

impl Clause {
  /// constructs a clause from literals
  pub fn new(literals: Vec<isize>) -> Clause {
    let sum = literals.iter().sum();
    Clause {
      size: literals.len(),
      literals,
      num_true: 0,
      num_false: 0,
      sum,
    }
  }
}

impl Cnf {
  /// constructs a cnf
  pub fn new(
    filename: String,
    clauses: Vec<Clause>,
    num_variables: usize,
    num_clauses: usize,
  ) -> Cnf {
    Cnf {
      filename,
      clauses,
      num_variables,
      num_clauses,
    }
  }
}

impl State {
  /// constructs the default state
  pub fn new(num_variables: usize) -> State {
    State {
      assignments: NVec::new(num_variables),
      references: NVec::new(num_variables),
      level: 0,
      propagated: 0,
      control: vec![],
      trail: vec![],
      #[cfg(debug_assertions)]
      stats: Stats::default(),
    }
  }
}
