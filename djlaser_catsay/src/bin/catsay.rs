use clap::Parser;
use djlaser_catsay::cli::{self, parser::Cli};
use std::{
  fs,
  io::{stderr, stdin, stdout},
  process::ExitCode,
};

pub fn main() -> ExitCode {
  let args = Cli::parse();
  return cli::main(
    args,
    &mut stdin().lock(),
    &mut stdout().lock(),
    &mut stderr().lock(),
    |path| fs::read_to_string(path),
  );
}
