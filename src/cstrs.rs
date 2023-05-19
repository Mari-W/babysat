use crate::{
  data::{Clause, Cnf, State, Stats},
  nvec::NVec,
};

impl Clause {
  /// constructs a clause from literals
  pub fn new(literals: Vec<isize>) -> Clause {
    Clause {
      size: literals.len(),
      literals,
    }
  }

  /// constructs a unit clause from one literal
  pub fn new_unit(lit: isize) -> Clause {
    Clause {
      size: 1,
      literals: vec![lit],
    }
  }
}

impl Cnf {
  /// constructs a cnf
  pub fn new(
    filename: String,
    clauses: Vec<Clause>,
    num_clauses: usize,
    num_variables: usize,
  ) -> Cnf {
    Cnf {
      filename,
      clauses,
      num_clauses,
      num_variables,
    }
  }
}

impl<'a> State<'a> {
  /// constructs the default state
  pub fn new(num_variables: usize) -> State<'a> {
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
