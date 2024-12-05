use catsay::cli;
use std::env;
use std::io::{stderr, stdin, stdout, IsTerminal};

pub fn main() -> Result<(), cli::CliError> {
  // Discard argv0 (process path)
  let args: Vec<String> = env::args().skip(1).collect();
  return cli::main(
    args,
    stdin().is_terminal(),
    stdin().lock(),
    stdout().lock(),
    stderr().lock(),
  );
}
