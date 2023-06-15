use std::cmp::max;

use log::trace;

#[cfg(debug_assertions)]
use log::info;

use crate::data::{Assignment, Clause, Cnf, State, Status};
use crate::nvec::NVec;

/// assigns a literal to true
#[inline]
fn assign(
  literal: isize,
  clauses: &mut [Clause],
  references: &NVec<Vec<usize>>,
  assignments: &mut NVec<Assignment>,
  trail: &mut Vec<isize>,
) {
  debug_assert!(literal != 0);

  // update matrix
  assignments[literal] = Assignment::True;
  assignments[-literal] = Assignment::False;

  // update counters
  for clause in &references[literal] {
    clauses[*clause].num_true += 1;
  }

  for clause in &references[-literal] {
    let clause = &mut clauses[*clause];
    clause.num_false += 1;
    clause.sum += literal
  }

  // push to literal to trail
  trail.push(literal);

  trace!("assigned literal {}", literal);
}

/// removes assignment for a literal
#[inline]
fn unassign(
  literal: isize,
  clauses: &mut [Clause],
  references: &NVec<Vec<usize>>,
  assignments: &mut NVec<Assignment>,
) {
  debug_assert!(
    matches!(assignments[literal], Assignment::True)
      && matches!(assignments[-literal], Assignment::False)
  );

  assignments[literal] = Assignment::Unassigned;
  assignments[-literal] = Assignment::Unassigned;

  for clause in &references[literal] {
    clauses[*clause].num_true -= 1;
  }

  for clause in &references[-literal] {
    let clause = &mut clauses[*clause];
    clause.num_false -= 1;
    clause.sum -= literal
  }

  trace!("unassigned literal {}", literal);
}

/// backtracks to the last trail height on the control stack
/// and removes assignments for the affected literals
/// returns true if backtracking succeeded and false when formula is unsatisfiable
fn backtrack(clauses: &mut [Clause], state: &mut State) -> bool {
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
    unassign(literal, clauses, &state.references, &mut state.assignments)
  }

  // and negate the element that initiated the propagations
  let literal = state
    .trail
    .pop()
    .expect("expected to find the literal that was split on this level to negate on");

  unassign(literal, clauses, &state.references, &mut state.assignments);
  assign(
    -literal,
    clauses,
    &state.references,
    &mut state.assignments,
    &mut state.trail,
  );

  // decrement level
  state.level -= 1;
  trace!("backtracked to level {}", state.level);

  // reset propagated
  state.propagated = length;

  true
}

/// decides on a new literal that can be assigned
/// returns true if a decision could made and false when formula is satisfiable
fn decide(cnf: &mut Cnf, state: &mut State) -> bool {
  // DLIS
  let mut scores: NVec<usize> = NVec::new(cnf.num_variables);
  let mut max: usize = 0;
  let mut literal: isize = 0;
  for clause in &cnf.clauses {
    match status(clause) {
      Status::None => {
        for lit in clause {
          // increment counter for all literals that are present in unsatisfied clause
          // if literal is unassigned
          if matches!(
            &state.assignments[*lit],
            Assignment::Unassigned
          ) {
            let score = scores[*lit] + 1;
            if scores[*lit] > max {
              max = score;
              literal = *lit;
            }
            scores[*lit] = score;
          }
        }
      }
      Status::Forcing(_) => unreachable!(),
      Status::Satisfied | Status::Falsified => continue,
    }
  }

  if literal == 0 {
    // no literal left
    return false
  }

  // increment level
  state.level += 1;

  // push trail length to control stack
  state.control.push(state.trail.len());

  // assign decided literal
  assign(
    literal as isize,
    &mut cnf.clauses,
    &state.references,
    &mut state.assignments,
    &mut state.trail,
  );

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
fn status(clause: &Clause) -> Status {
  if clause.num_true > 0 {
    Status::Satisfied
  } else if clause.size == clause.num_false {
    Status::Falsified
  } else if clause.size - 1 == clause.num_false {
    Status::Forcing(clause.sum)
  } else {
    Status::None
  }
}

/// boolean constraint propagation
fn propagate(clauses: &mut [Clause], state: &mut State) -> bool {
  // while not all propagated
  while state.propagated < state.trail.len() {
    // iterate all negative occurrences of a literal from the trail
    for clause in &state.references[-state.trail[state.propagated]] {
      let clause = &clauses[*clause];
      match status(clause) {
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
          assign(
            literal,
            clauses,
            &state.references,
            &mut state.assignments,
            &mut state.trail,
          );
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
fn connect_clauses(clauses: &mut [Clause], state: &mut State) -> bool {
  for (i, clause) in clauses.iter().enumerate() {
    // connect literals to clauses in matrix
    for literal in clause {
      state.references[*literal].push(i);
    }
  }
  for i in 0..clauses.len() {
    let clause = &clauses[i];
    match clause.size {
      // handle empty clause added
      0 => {
        trace!("found empty clause in list of clauses to solve");
        return false;
      }
      // handle unit clause added
      1 => {
        let literal = clause.sum;
        match state.assignments[literal] {
          // unassigned means we do unit propagation
          Assignment::Unassigned => {
            assign(
              literal,
              clauses,
              &state.references,
              &mut state.assignments,
              &mut state.trail,
            );
            trace!("assigned initial unit clause {}", i)
          }
          // false means the given clauses are inconsistent
          Assignment::False => {
            trace!("found inconsistent initial unit clause {}", i);
            return false;
          }
          Assignment::True => (),
        }
      }
      _ => (),
    };

    #[cfg(debug_assertions)]
    {
      state.stats.added += 1;
    }
  }
  true
}

pub fn solve(mut cnf: Cnf) -> Option<NVec<Assignment>> {
  // instantiate state
  let mut state = State::new(cnf.num_variables);

  if cnf.clauses.is_empty() {
    // empty formula is true
    return Some(NVec::new(0));
  }

  // connect the literals to clause references in matrix
  if !connect_clauses(&mut cnf.clauses, &mut state) {
    return None;
  }

  // dpll algorithm
  loop {
    // if propagation succeeds
    if propagate(&mut cnf.clauses, &mut state) {
      // decide on new literal
      if !decide(&mut cnf, &mut state) {
        // except if there are non left to assign and formula is sat

        #[cfg(debug_assertions)]
        debug_assert!(check_model(&cnf.clauses));

        #[cfg(debug_assertions)]
        info!("{:#?}", state.stats);

        return Some(state.assignments);
      }
    // if propagation fails we try to backtrack
    } else if !backtrack(&mut cnf.clauses, &mut state) {
      // except if we are on the last level then formula is unsat
      #[cfg(debug_assertions)]
      info!("{:#?}", state.stats);
      return None;
    }
  }
}

/// checks a model given an assignment
#[cfg(debug_assertions)]
fn check_model(clauses: &[Clause]) -> bool {
  for clause in clauses {
    if clause.num_true == 0 {
      return false;
    }
  }
  trace!("model check succeeded");
  true
}
