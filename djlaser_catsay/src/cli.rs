pub mod parser;

use std::io::{self, Read, Write};
use std::process;
use std::string::FromUtf8Error;

use crate::{say, Cat, CatChoice, CatsayOptions};
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

const CREDIT: &str = "\
Art sourced from ASCII Art Archive https://www.asciiart.eu/animals/cats
Catsay Programmed by DJ_Laser";

pub fn get_credits() -> String {
  let mut credits = String::from(CREDIT);
  for cat in Cat::CATS {
    credits.push('\n');
    credits.push_str(&cat.credit);
  }

  return credits;
}

pub fn try_get_cat<B: Write>(
  name: &str,
  stderr: &mut B,
) -> Result<&'static Cat<'static>, CliError> {
  let cat = Cat::get_cat(name);
  if cat.is_none() {
    stderr.write(&format!(r#"No cat "{name}"" available, sorry"#).as_bytes())?;
    process::exit(1);
  }

  Ok(cat.unwrap())
}

pub fn main<I, O, E>(args: Cli, mut stdin: I, mut stdout: O, mut stderr: E) -> Result<(), CliError>
where
  I: Read,
  O: Write,
  E: Write,
{
  let mut options: CatsayOptions = Default::default();

  if let Some(padding) = args.padding {
    options.left_padding = padding;
  }

  if let Some(name) = args.cat {
    options.cat = CatChoice::Choice(try_get_cat(&name, &mut stderr)?);
  }

  if args.action.credits {
    stdout.write(get_credits().as_bytes())?;
  } else if args.action.list_cats {
    for cat in Cat::CATS {
      write!(stdout, "--cat {}: {}", cat.name, cat.credit)?;
      write!(stdout, "\n")?;
    }
  } else if args.action.show_cats {
    for cat in Cat::CATS {
      let text = format!("--cat {}:\n{}", cat.name, cat.credit);
      stdout.write(say(&text, &options.clone().with_cat(cat)).as_bytes())?;
      stdout.write("\n".as_bytes())?;
    }
  } else if let Some(name) = args.action.show_cat {
    let cat = try_get_cat(&name, &mut stderr)?;
    stdout.write(say(&cat.credit, &options.clone().with_cat(cat)).as_bytes())?;
  } else {
    let text = if args.action.use_stdin {
      let mut buf: Vec<u8> = Vec::new();
      stdin.read_to_end(&mut buf)?;
      String::from_utf8(buf)?
    } else {
      args.action.say.join(" ")
    };

    stdout.write(say(&text, &options).as_bytes())?;
  }

  Ok(())
}
