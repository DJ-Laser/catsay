use clap::Parser;
use djlaser_catsay::cli::{self, parser::Cli};
use std::{
  io::{stderr, stdin, stdout},
  process::ExitCode,
};

pub fn main() -> ExitCode {
  let args = Cli::parse();
  return cli::main(args, stdin().lock(), stdout().lock(), stderr().lock());
}
