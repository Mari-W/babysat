use crate::nvec::NVec;

/// represents a clause
#[derive(Debug)]
pub struct Clause {
  pub size: usize,
  pub literals: Vec<i64>,
}

/// represents a clause
#[derive(Debug)]
pub struct Cnf {
  pub filename: String,
  pub clauses: Vec<Clause>,
  pub num_clauses: usize,
  pub num_variables: usize,
}

/// represents an assignment state in 8 bits
#[repr(i8)]
#[derive(Debug, Default, Clone)]
pub enum Assignment {
  #[default]
  Unassigned = 0,
  True = 1,
  False = -1,
}

/// represents the matrix that references clauses for literals
#[derive(Debug)]
pub struct Matrix {
  /// stores the assignment for each literal
  pub assignments: NVec<Assignment>,
  /// stores the indexes of all clauses where a literal occurs
  pub references: NVec<Vec<usize>>,
  // NOTE: this should be should (could) be references &'a Clause, but since
  // clauses get added during the runtime of the algorithm
  // figuring out the lifetime requirements is hard at least.
  // I will try to optimize this (maybe using raw pointers) in the optimization part
}

/// represents the state of the dpll algorithm
pub struct State {
  // decision level
  pub level: usize,
  // next position on trail to propagate
  pub propagated: usize,
  // maps literals to levels
  pub levels: Vec<usize>,
  // stack for trail length
  pub control: Vec<usize>,
  // stores assigned literals
  pub trail: Vec<i64>,
}

/// represents the current status of a clause
/// given an assignment
#[derive(Debug)]
pub enum Status {
  // nothing can be said about the clause
  None,
  // clause is satisfied
  Satisfied,
  // clause is falsified
  Falsified,
  // clause has exactly one variable unassigned 
  // and its the only one that makes the clause true
  Forcing(i64),
}

#[derive(Debug, Default)]
pub struct Stats {
  // number of added clauses
  added: usize,
  // number of conflicts
  conflicts: usize,
  // number of decisions
  decisions: usize,
  // number of propagated literals
  propagations: usize,
  // number of calls to 'report'
  reports: usize,
  // number of root-level assigned variables
  fixed: i64,
}
