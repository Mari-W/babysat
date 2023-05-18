use crate::exit_unsatisfiable;
use log::trace;

use crate::data::{Assignment, Clause, Cnf, Matrix, State, Status};
use crate::nvec::NVec;

/// connects a literal to a clause inside the matrix
fn connect_literal(matrix: &mut Matrix, clause: usize, lit: i64) {
  // add clause idx to list of references
  matrix.references[lit].push(clause);

  trace!("connected literal {} to clause {}", lit, clause);
}

/// connects all literals in a list of clauses to the matrix
fn connect_clauses(matrix: &mut Matrix, clauses: &Vec<Clause>, state: &mut State) {
  for (idx, clause) in clauses.iter().enumerate() {
    match clause.size {
      // handle empty clause added
      0 => {
        trace!("found empty clause in list of clauses to solve");
        exit_unsatisfiable()
      }
      // handle unit clause added
      1 => {
        let literal = clause[0];
        match matrix.assignments[literal] {
          // unassigned means we do unit propagation
          Assignment::Unassigned => {
            assign(&mut matrix.assignments, state, literal);
            trace!("assigned initial unit clause {}", clause)
          }
          // false means the given clauses are inconsistent
          Assignment::False => {
            trace!("found inconsistent initial unit clause {}", clause);
            exit_unsatisfiable()
          }
          Assignment::True => (),
        }
      }
      _ => (),
    };

    // connect literals to clauses in matrix
    for literal in clause {
      connect_literal(matrix, idx, *literal)
    }
  }
}

fn add_unit_clause(cnf: &mut Cnf, matrix: &mut Matrix, state: &mut State, literal: i64) {
  assign(&mut matrix.assignments, state, literal);
  connect_literal(matrix, cnf.clauses.len(), literal);

  let clause = Clause::new_unit(literal);
  cnf.clauses.push(clause)
}

fn neg_unit_clause(cnf: &mut Cnf, matrix: &mut Matrix, state: &mut State, literal: i64) {
  assign(&mut matrix.assignments, state, -literal);

  let clause = matrix.references[literal]
    .pop()
    .expect("expected unit clause to be already added to list of references when negated");

  debug_assert!(cnf.clauses[clause].literals.len() == 1);
  debug_assert!(cnf.clauses[clause].literals[0] == literal);

  cnf.clauses[clause].literals[0] *= -1;
  connect_literal(matrix, clause, -literal);

  trace!(
    "negated unit clause ({})[{}] now {}",
    literal,
    clause,
    cnf.clauses[clause]
  );
}

/// assigns a literal to true
fn assign(assignments: &mut NVec<Assignment>, state: &mut State, literal: i64) {
  debug_assert!(literal != 0);

  // update level
  let idx: usize = (literal.abs() - 1)
    .try_into()
    .expect("unreasonable large literal");
  state.levels[idx] = state.level;

  // update matrix
  assignments[literal] = Assignment::True;
  assignments[-literal] = Assignment::False;

  // push to literal to trail
  state.trail.push(literal);

  trace!("assigned literal {}", literal);
}

fn unassign(matrix: &mut Matrix, literal: i64) {
  matrix.assignments[literal] = Assignment::Unassigned;
  matrix.assignments[-literal] = Assignment::Unassigned;
  trace!("unassigned literal {}", literal);
}

/// returns the current status of a clause given an assignment
fn status(assignments: &NVec<Assignment>, clause: &Clause) -> Status {
  let mut unassigned = None;
  // trace!("getting status of {}", self);
  for literal in clause {
    match assignments[*literal] {
      Assignment::Unassigned => {
        // two unassigned literals means we know nothing
        if matches!(unassigned, Some(_)) {
          return Status::None;
        } else {
          // first unassigned clause is saved
          unassigned = Some(*literal)
        }
      }
      // if one literal is satisfied then the clause is satisfied
      Assignment::True => return Status::Satisfied,
      Assignment::False => (),
    }
  }
  if let Some(lit) = unassigned {
    // the clause is forced if it is the only unassigned clause
    // and the clause has no true literal
    return Status::Forcing(lit);
  } else {
    // if there is no unassigned clause and no satisfied clause
    // then the clause is falsified
    return Status::Falsified;
  }
}

fn propagate(cnf: &Cnf, matrix: &mut Matrix, state: &mut State) -> bool {
  trace!(
    "propagating.. {} -- {}",
    state.propagated,
    state.trail.len()
  );
  while state.propagated < state.trail.len() {
    trace!("joined");
    let literal = state.trail[state.propagated];
    state.propagated += 1;
    for clause in &matrix.references[-literal] {
      let clause = &cnf.clauses[*clause];
      match status(&matrix.assignments, &clause) {
        // nothing to do
        Status::None | Status::Satisfied => {
          trace!("nothing")
        }
        // conflict
        Status::Falsified => {
          trace!("found falsified clause {}", clause);
          return false;
        }
        // forcing literal is assigned
        Status::Forcing(literal) => {
          trace!("found forcing clause {} that forced {}", clause, literal);
          assign(&mut matrix.assignments, state, literal);
        }
      }
    }
  }
  return true;
}

fn backtrack(state: &mut State, matrix: &mut Matrix) {
  // get old trail length
  let length = state
    .control
    .pop()
    .expect("backtracked empty control stack");

  debug_assert!(length < state.trail.len());

  // unassign literals
  for literal in state.trail.drain(length..) {
    unassign(matrix, literal)
  }

  // update propagated
  state.propagated = length;

  // decrement level
  state.level -= 1;

  trace!("backtracked to level {}", state.level);
}

/// decides on a new literal that can be assigned
fn decide(matrix: &Matrix, state: &mut State) -> Option<i64> {
  // find new literal that is unassigned
  // by naively iterating the assignment vector until
  // some unassigned literal is found
  let mut literal = 0;
  for (idx, (pos, neg)) in (&matrix.assignments).into_iter().enumerate() {
    if matches!(pos, Assignment::Unassigned) && matches!(neg, Assignment::Unassigned) {
      literal = (idx + 1) as i64
    }
  }

  // no literal was found so the cnf is satisfiable
  if literal == 0 {
    trace!("all variables assigned");
    return None;
  }

  // increment level
  state.level += 1;

  // push trail length to control stack
  state.control.push(state.trail.len());

  trace!(
    "decided on literal {} and jumped to level {}",
    literal,
    state.level
  );

  Some(literal)
}

fn dpll(cnf: &mut Cnf, matrix: &mut Matrix, state: &mut State) -> bool {
  // if propagate returns false its a conflict
  if !propagate(cnf, matrix, state) {
    return false;
  }

  // decide on literal
  let literal = match decide(matrix, state) {
    Some(literal) => literal,
    // none means no variable left to assign
    None => return true,
  };

  // add unassigned unit
  add_unit_clause(cnf, matrix, state, literal);

  // recurse left
  if dpll(cnf, matrix, state) {
    return true;
  }

  // backtrack left
  backtrack(state, matrix);

  // negate unit clause
  neg_unit_clause(cnf, matrix, state, literal);

  // recurse right
  dpll(cnf, matrix, state)
}

pub fn solve(mut cnf: Cnf) -> Option<NVec<Assignment>> {
  trace!("now solving {}", cnf);

  // initialize state
  let mut state = State::new(cnf.num_variables);

  // initialize matrix
  let mut matrix = Matrix::new(cnf.num_variables);

  // connect clauses in matrix
  connect_clauses(&mut matrix, &cnf.clauses, &mut state);

  // run dpll
  if dpll(&mut cnf, &mut matrix, &mut state) {
    // check model in debug mode
    #[cfg(debug_assertions)]
    check_model(&cnf, &matrix);

    // return witness if satisfiable
    return Some(matrix.assignments);
  }

  return None;
}

#[allow(dead_code)]
fn check_model(cnf: &Cnf, matrix: &Matrix) -> bool {
  let check_clause = |clause: &Clause| -> bool {
    for literal in clause {
      if matches!(matrix.assignments[*literal], Assignment::True) {
        return true;
      }
    }
    return false;
  };
  for clause in cnf {
    if !check_clause(&clause) {
      return false;
    }
  }
  trace!("model check succeeded");
  return true;
}
