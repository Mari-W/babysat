use crate::data::{Clause, Cnf};
use std::{borrow::Borrow, ffi::OsStr, fs, io, path::Path};

// TODO: infer header if not present
// TODO: write parser without library for performance
peg::parser! {
  /// dimacs format file parser
  grammar dimacs() for str {

    rule _ = quiet!{[' ' | '\n' | '\t' | '\r']+}
    rule __ = quiet!{[' ' | '\t' | '\r']+}
    rule ___ = quiet!{[' ' | '\n' | '\t' | '\r']*}

    /// parses a number that is not zero
    rule num<T: std::str::FromStr>() -> T
      = !(['0']) n:$(['0'..='9' | '-']+) {?
        n.parse::<T>().or(Err("expected integer"))
      }

    /// parses a number that is possibly zero
    rule num0<T: std::str::FromStr>() -> T
      = n:$(['0'..='9' | '-']+) {?
        n.parse::<T>().or(Err("expected integer"))
      }

    /// parses a comment line
    rule comment() = "c" ((!"\n"[_])+) "\n"

    /// parses a single clause
    rule clause(num_variables: usize) -> Clause
      = comment()* ___ literals:num()**_ ___ "0" ___ {?
        if cfg!(debug_assertions) && literals.iter().map(|c| (c as &isize).abs()).max().unwrap_or(0) as usize > num_variables {
          return Err("literal out of bounds")
        }
        Ok(Clause::new(literals))
      }

    /// parses the header and the list of clauses
    rule problem() -> (usize, usize, Vec<Clause>)
      = "p" __ "cnf" __ num_variables:num0() __ num_clauses:num0() ___ clauses:clause(num_variables)* ___ {
        (num_variables, num_clauses, clauses)
      }

    /// parses a .cnf file formatted string in dimacs format
    pub rule cnf<'a>(filename: &str) -> (usize, usize, Vec<Clause>)
      = comment()* p:problem() {?
        let (num_variables, num_clauses, clauses) = p;
        if num_clauses != clauses.len() {
            Err("too many clauses")
        } else {
            Ok((num_variables, num_clauses, clauses))
        }
      }
  }
}

/// parses a .cnf from a file path
pub fn parse_file<'a, P: AsRef<Path> + 'a>(path: Option<P>) -> crate::Result<Cnf> {
  // read file path if present and stdin otherwise
  let content = match &path {
    Some(path) => fs::read_to_string(path.as_ref())?,
    None => io::read_to_string(io::stdin())?,
  };

  let filename = match &path {
    Some(path) => path
      .as_ref()
      .file_name()
      .unwrap_or(OsStr::new("unknown.cnf"))
      .to_string_lossy()
      .to_string(),
    None => "stdin".into(),
  };

  // parse clauses and information
  let (num_variables, num_clauses, clauses) = dimacs::cnf(content.as_str(), filename.borrow())?;

  // build dpll instance
  let cnf = Cnf::new(filename, clauses, num_variables, num_clauses);
  // dpll.add_clauses(clauses);
  Ok(cnf)
}
