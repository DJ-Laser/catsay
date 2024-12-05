use catsay::cli::{self, parser::Cli};
use clap::Parser;
use std::io::{stderr, stdin, stdout, IsTerminal};

pub fn main() -> Result<(), cli::CliError> {
  let args = Cli::parse();
  return cli::main(
    args,
    stdin().is_terminal(),
    stdin().lock(),
    stdout().lock(),
    stderr().lock(),
  );
}
