use std::io::{self, Read, Write};
use std::string::FromUtf8Error;

use crate::{get_help_text, say};

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

pub fn main<I, O, E>(
  args: Vec<String>,
  is_terminal: bool,
  mut stdin: I,
  mut stdout: O,
  mut _stderr: E,
) -> Result<(), CliError>
where
  I: Read,
  O: Write,
  E: Write,
{
  // TODO: cli arg parsing
  let options = Default::default();

  if args.len() == 0 {
    if is_terminal {
      stdout.write(get_help_text().as_bytes())?;
      return Ok(());
    }

    let mut buf: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut buf)?;

    let text = String::from_utf8(buf)?;
    stdout.write(say(&text, &options).as_bytes())?;

    return Ok(());
  }

  let text: String = args.join(" ");
  stdout.write(say(&text, &options).as_bytes())?;

  Ok(())
}
