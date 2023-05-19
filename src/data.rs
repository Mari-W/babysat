use crate::nvec::NVec;

/// represents a clause
#[derive(Debug)]
pub struct Clause {
  pub size: usize,
  pub literals: Vec<isize>,
}

/// represents a cnf
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
  Forcing(isize),
}

#[derive(Debug, Default)]
#[cfg(debug_assertions)]
pub struct Stats {
  // number of added clauses
  pub added: usize,
  // number of conflicts
  pub conflicts: usize,
  // number of decisions
  pub decisions: usize,
  // number of propagated literals
  pub propagations: usize,
  // number of calls to 'report'
  pub reports: usize,
  // number of root-level assigned variables
  pub fixed: isize,
}

/// represents the state of the dpll algorithm
pub struct State<'a> {
  // stores the assignment for each literal
  pub assignments: NVec<Assignment>,
  // stores the indexes of all clauses where a literal occurs
  pub references: NVec<Vec<&'a Clause>>,
  // decision level
  pub level: usize,
  // next position on trail to propagate
  pub propagated: usize,
  // stack for trail length
  pub control: Vec<usize>,
  // stores assigned literals
  pub trail: Vec<isize>,
  // stores statistics
  #[cfg(debug_assertions)]
  pub stats: Stats,
}
