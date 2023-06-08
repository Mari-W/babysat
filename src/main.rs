use std::{process::exit, time::SystemTime};

use clap::Parser;
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

mod cstrs;
mod data;
mod dimacs;
mod dpll;
mod impls;
mod nvec;

/// propagate arbitrary errors that implement the trait `std::error::Error`
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// struct parsed based on command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
  /// Path to input file in DIMACS format
  path: Option<String>,

  /// Do not print any messages
  #[arg(short, long)]
  quite: bool,

  /// Do not print witness if satisfiable
  #[arg(short, long)]
  no_witness: bool,

  /// Print verbose messages
  #[arg(short, long)]
  verbose: bool,
}

/// set log level given the cli arguments and build logger
fn init_logging(args: &Args) -> Result<()> {
  let config = ConfigBuilder::new()
    .set_time_level(LevelFilter::Off)
    .build();
  let mut invalid_combination = false;
  let level = match (args.quite, args.verbose) {
    (true, true) => {
      invalid_combination = true;
      LevelFilter::Error
    }
    (true, false) => LevelFilter::Off,
    (false, true) => LevelFilter::Trace,
    (false, false) => LevelFilter::Info,
  };
  TermLogger::init(level, config, TerminalMode::Stdout, ColorChoice::Always)?;
  if invalid_combination {
    Err(String::from("cannot be quite and verbose at the same time").into())
  } else {
    Ok(())
  }
}

fn _main() -> Result<()> {
  // parse command line arguments
  let args = Args::parse();

  // initialize logging
  init_logging(&args)?;

  // print information
  info!("BabySAT DPLL SAT Solver");
  info!("Copyright (c) 2023, {}", env!("CARGO_PKG_AUTHORS"));
  info!("Version {}", env!("CARGO_PKG_VERSION"));

  // parse problem instance
  let cnf = dimacs::parse_file(args.path)?;

  let time = SystemTime::now();

  let sat = dpll::solve(cnf);

  info!("Solving took {:.2?} [{:.2?}]", SystemTime::now().duration_since(time).unwrap(), cpu_time::ProcessTime::now());

  match sat {
    Some(witness) => {
      if !args.no_witness {
        info!("Witness: \n{}", witness)
      }

      info!("SATISFIABLE");
      exit(10)
    }
    None => {
      info!("UNSATISFIABLE");
      exit(20)
    }
  }
}

fn main() {
  if let Err(e) = _main() {
    error!("{}", e)
  }
}
