use std::env;
use std::io::{self, IsTerminal, Read, Write};
use std::string::FromUtf8Error;

use catsay::{get_help_text, say};

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

pub fn main() -> Result<(), CliError> {
  // Discard argv0 (process path)
  let args = env::args().skip(1);
  // TODO: cli arg parsing
  let options = Default::default();

  let mut stdout = io::stdout().lock();

  if args.len() == 0 {
    if io::stdin().is_terminal() {
      stdout.write(get_help_text().as_bytes())?;
      return Ok(());
    }

    let mut buf: Vec<u8> = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;

    let text = String::from_utf8(buf)?;
    stdout.write(say(&text, &options).as_bytes())?;

    return Ok(());
  }

  let text: String = args.collect::<Vec<String>>().join(" ");
  stdout.write(say(&text, &options).as_bytes())?;

  Ok(())
}
