pub mod parser;

use std::io::{self, Read, Write};
use std::string::FromUtf8Error;

use crate::say;
use parser::Cli;

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

pub fn main<I, O, E>(args: Cli, mut stdin: I, mut stdout: O, mut _stderr: E) -> Result<(), CliError>
where
  I: Read,
  O: Write,
  E: Write,
{
  let options = Default::default();

  if args.action.say.use_stdin {
    let mut buf: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut buf)?;

    let text = String::from_utf8(buf)?;
    stdout.write(say(&text, &options).as_bytes())?;

    return Ok(());
  }

  let text: String = args.action.say.args.join(" ");
  stdout.write(say(&text, &options).as_bytes())?;

  Ok(())
}
