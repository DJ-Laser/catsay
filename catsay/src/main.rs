use std::env;
use std::io::{self, IsTerminal, Read, Write};
use std::string::FromUtf8Error;

use catsay::say;

#[derive(Debug)]
pub enum CliError {
  IoError(io::Error),
  FromUtf8Error(FromUtf8Error),
}

impl From<io::Error> for CliError {
  fn from(error: io::Error) -> CliError {
    CliError::IoError(error)
  }
}

impl From<FromUtf8Error> for CliError {
  fn from(error: FromUtf8Error) -> CliError {
    CliError::FromUtf8Error(error)
  }
}

const HELP_TEXT: &str = "\
Catsay written by DJ_Laser
Usage: catsay <text>
";

pub fn main() -> Result<(), CliError> {
  // Discard argv0 (process path)
  let args = env::args().skip(1);
  let mut stdout = io::stdout().lock();

  if args.len() == 0 {
    if io::stdin().is_terminal() {
      stdout.write(HELP_TEXT.as_bytes())?;
      return Ok(());
    }

    let mut buf: Vec<u8> = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;

    let text = String::from_utf8(buf)?;
    stdout.write(say(text).as_bytes())?;

    return Ok(());
  }

  let text: String = args.collect::<Vec<String>>().join(" ");
  stdout.write(say(text).as_bytes())?;

  Ok(())
}
