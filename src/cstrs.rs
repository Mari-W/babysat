use crate::{
  data::{Clause, Cnf, Matrix, State, Stats},
  nvec::NVec,
};

impl Clause {
  /// constructs a clause from literals
  pub fn new(literals: Vec<i64>) -> Clause {
    Clause {
      size: literals.len(),
      literals,
    }
  }

  /// constructs a unit clause from one literal
  pub fn new_unit(lit: i64) -> Clause {
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

impl Matrix {
  /// constructs a new matrix of size `num_variables`
  pub fn new(num_variables: usize) -> Matrix {
    Matrix {
      assignments: NVec::new(num_variables),
      references: NVec::new(num_variables),
    }
  }
}

impl State {
  /// constructs the default state
  pub fn new(num_variables: usize) -> State {
    State {
      level: usize::default(),
      propagated: usize::default(),
      levels: vec![usize::default(); num_variables],
      control: Vec::default(),
      trail: Vec::default(),
    }
  }
}

impl Stats {
  /// constructs a new stats object
  pub fn new() -> Stats {
    Stats::default()
  }
}
