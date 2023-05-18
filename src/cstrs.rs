use crate::{
  data::{Clause, Cnf, Matrix, State},
  nvec::NVec,
};

impl Clause {
  /// constructor for clauses
  pub fn new(literals: Vec<i64>) -> Clause {
    Clause {
      size: literals.len(),
      literals,
    }
  }

  /// constructor for a unit clause
  pub fn new_unit(lit: i64) -> Clause {
    Clause {
      size: 1,
      literals: vec![lit],
    }
  }
}

impl Cnf {
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
