use std::env;
use std::io::{self, stdin, stdout, Write};

use catsay::say;

const HELP_TEXT: &str = "\
Catsay written by DJ_Laser
Usage: catsay <text>
";

pub fn main() -> Result<(), io::Error> {
  // Discard argv0 (process path)
  let args = env::args().skip(1);
  let mut handle = stdout().lock();

  if args.len() == 0 {
    handle.write(HELP_TEXT.as_bytes())?;
    return Ok(());
  }

  let text: String = args.collect::<Vec<String>>().join(" ");
  handle.write(say(text).as_bytes())?;

  Ok(())
}
