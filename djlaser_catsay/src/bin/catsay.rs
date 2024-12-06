use clap::Parser;
use djlaser_catsay::cli::{self, parser::Cli};
use std::io::{stderr, stdin, stdout};

pub fn main() -> Result<(), cli::CliError> {
  let args = Cli::parse();
  return cli::main(args, stdin().lock(), stdout().lock(), stderr().lock());
}
