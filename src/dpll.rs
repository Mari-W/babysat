use log::trace;

#[cfg(debug_assertions)]
use log::info;

use crate::data::{Assignment, Clause, Cnf, State, Status};
use crate::nvec::NVec;

/// assigns a literal to true
#[inline]
fn assign(literal: isize, assignments: &mut NVec<Assignment>, trail: &mut Vec<isize>) {
  debug_assert!(literal != 0);

  // update matrix
  assignments[literal] = Assignment::True;
  assignments[-literal] = Assignment::False;

  // push to literal to trail
  trail.push(literal);

  trace!("assigned literal {}", literal);
}

/// removes assignment for a literal
#[inline]
fn unassign(assignments: &mut NVec<Assignment>, literal: isize) {
  assignments[literal] = Assignment::Unassigned;
  assignments[-literal] = Assignment::Unassigned;

  trace!("unassigned literal {}", literal);
}

/// backtracks to the last trail height on the control stack
/// and removes assignments for the affected literals
/// returns true if backtracking succeeded and false when formula is unsatisfiable
fn backtrack(state: &mut State) -> bool {
  // backtracking on the root level means
  // the formula is falsified
  if state.level == 0 {
    return false;
  }

  // get old trail length
  let length = state
    .control
    .pop()
    .expect("backtracked empty control stack");

  debug_assert!(length < state.trail.len());

  // unassign all literals from the trail
  for literal in state.trail.drain(length + 1..) {
    unassign(&mut state.assignments, literal)
  }

  // and negate the element that initiated the propagations
  let literal = state.trail.pop().expect("expected ..");
  assign(-literal, &mut state.assignments, &mut state.trail);

  // decrement level
  state.level -= 1;
  trace!("backtracked to level {}", state.level);

  // reset propagated
  state.propagated = length;

  true
}

/// decides on a new literal that can be assigned
/// returns true if a decision could made and false when formula is satisfiable
fn decide(state: &mut State) -> bool {
  // find new literal that is unassigned
  // by naively iterating the assignment vector until
  // some unassigned literal is found
  let mut literal = 0;
  for (idx, (pos, neg)) in (&state.assignments).into_iter().enumerate() {
    debug_assert!(!(matches!(pos, Assignment::Unassigned) ^ matches!(neg, Assignment::Unassigned)));

    match pos {
      Assignment::Unassigned => {
        literal = (idx + 1) as isize;
        break;
      }
      Assignment::True | Assignment::False => continue,
    }
  }

  // no literal was found so the cnf is satisfiable
  if literal == 0 {
    trace!("all variables assigned");
    return false;
  }

  // increment level
  state.level += 1;

  // push trail length to control stack
  state.control.push(state.trail.len());

  // assign decided literal
  assign(literal, &mut state.assignments, &mut state.trail);

  trace!(
    "decided on literal {} and incremented to level {}",
    literal,
    state.level
  );

  #[cfg(debug_assertions)]
  {
    state.stats.decisions += 1;
  }

  true
}

/// returns the current status of a clause given an assignment
#[inline]
fn status(assignments: &NVec<Assignment>, clause: &Clause) -> Status {
  let mut unassigned = None;
  for literal in clause {
    match assignments[*literal] {
      Assignment::Unassigned => {
        // two unassigned literals means we know nothing
        match unassigned {
          Some(_) => return Status::None,
          None => unassigned = Some(*literal),
        }
      }
      // if one literal is satisfied then the clause is satisfied
      Assignment::True => return Status::Satisfied,
      Assignment::False => (),
    }
  }
  match unassigned {
    Some(literal) => Status::Forcing(literal),
    None => Status::Falsified,
  }
}

/// boolean constraint propagation
fn propagate(state: &mut State) -> bool {
  // while not all propagated
  while state.propagated < state.trail.len() {
    // iterate all negative occurrences of a literal from the trail
    for clause in &state.references[-state.trail[state.propagated]] {
      match status(&state.assignments, clause) {
        // nothing to do
        Status::None | Status::Satisfied => (),
        // conflict
        Status::Falsified => {
          trace!("found falsified clause {}", clause);

          #[cfg(debug_assertions)]
          {
            state.stats.conflicts += 1;
          }

          return false;
        }
        // forcing literal is assigned
        Status::Forcing(literal) => {
          trace!("found forcing clause {} that forced {}", clause, literal);
          assign(literal, &mut state.assignments, &mut state.trail);
        }
      }
    }
    state.propagated += 1;

    #[cfg(debug_assertions)]
    {
      state.stats.propagations += 1;
    }
  }
  true
}

/// connects all literals in a list of clauses to the matrix
/// returns true if all clauses connected and false if formula is unsat
fn connect_clauses<'a>(clauses: &'a Vec<Clause>, state: &mut State<'a>) -> bool {
  for clause in clauses {
    match clause.size {
      // handle empty clause added
      0 => {
        trace!("found empty clause in list of clauses to solve");
        return false;
      }
      // handle unit clause added
      1 => {
        let literal = clause[0];
        match state.assignments[literal] {
          // unassigned means we do unit propagation
          Assignment::Unassigned => {
            assign(literal, &mut state.assignments, &mut state.trail);
            trace!("assigned initial unit clause {}", clause)
          }
          // false means the given clauses are inconsistent
          Assignment::False => {
            trace!("found inconsistent initial unit clause {}", clause);
            return false;
          }
          Assignment::True => (),
        }
      }
      _ => (),
    };

    // connect literals to clauses in matrix
    for literal in clause {
      state.references[*literal].push(clause);
    }

    #[cfg(debug_assertions)]
    {
      state.stats.added += 1;
    }
  }
  true
}

pub fn solve(cnf: Cnf) -> Option<NVec<Assignment>> {
  // instantiate state
  let mut state = State::new(cnf.num_variables);

  // connect the literals to clause references in matrix
  if !connect_clauses(&cnf.clauses, &mut state) {
    return None;
  }

  // dpll algorithm
  loop {
    // if propagation succeeds
    if propagate(&mut state) {
      // decide on new literal
      if !decide(&mut state) {
        // except if there are non left to assign and formula is sat

        #[cfg(debug_assertions)]
        check_model(&cnf.clauses, &state);

        #[cfg(debug_assertions)]
        info!("{:#?}", state.stats);

        return Some(state.assignments);
      }
    // if propagation fails we try to backtrack
    } else if !backtrack(&mut state) {
      // except if we are on the last level then formula is unsat
      #[cfg(debug_assertions)]
      info!("{:#?}", state.stats);
      return None;
    }
  }
}

/// checks a model given an assignment
#[cfg(debug_assertions)]
fn check_model(clauses: &Vec<Clause>, state: &State) -> bool {
  let check_clause = |clause: &Clause| -> bool {
    for literal in clause {
      if matches!(state.assignments[*literal], Assignment::True) {
        return true;
      }
    }
    false
  };
  for clause in clauses {
    if !check_clause(clause) {
      return false;
    }
  }
  trace!("model check succeeded");
  true
}
