pub mod parser;

use std::io::{self, Read, Write};
use std::process::{self, ExitCode};

use crate::{say, Cat, CatChoice, CatsayOptions};
use parser::Cli;

#[derive(Debug)]
enum CliError {
  IoError(io::Error),
}

impl From<io::Error> for CliError {
  fn from(error: io::Error) -> CliError {
    CliError::IoError(error)
  }
}

const CREDIT: &str = "\
Art sourced from ASCII Art Archive https://www.asciiart.eu/animals/cats
Catsay programmed by DJ_Laser";

pub fn get_credits() -> String {
  let mut credits = String::from(CREDIT);
  for cat in Cat::CATS {
    credits.push('\n');
    credits.push_str(&cat.credit);
  }

  return credits;
}

fn try_get_cat<B: Write>(name: &str, stderr: &mut B) -> Result<&'static Cat<'static>, CliError> {
  let cat = Cat::get_cat(name);
  if cat.is_none() {
    write!(stderr, r#"No cat "{name}" available, sorry"#)?;
    process::exit(1);
  }

  Ok(cat.unwrap())
}

fn catsay<I, O, E>(args: Cli, mut stdin: I, mut stdout: O, mut stderr: E) -> Result<(), CliError>
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
    write!(stdout, "{}", get_credits())?;
  } else if args.action.list_cats {
    for cat in Cat::CATS {
      writeln!(stdout, "--cat {}: {}", cat.name, cat.credit)?;
    }
  } else if args.action.show_cats {
    for cat in Cat::CATS {
      let text = format!("--cat {}:\n{}", cat.name, cat.credit);
      options.set_cat(cat);
      writeln!(stdout, "{}", say(&text, &options))?;
    }
  } else if let Some(name) = args.action.show_cat {
    let cat = try_get_cat(&name, &mut stderr)?;
    options.set_cat(cat);
    write!(stdout, "{}", say(&cat.credit, &options))?;
  } else {
    let text = if args.action.use_stdin {
      let mut buf: String = String::new();
      stdin.read_to_string(&mut buf)?;
      buf
    } else {
      args.action.say.join(" ")
    };

    write!(stdout, "{}", say(&text, &options))?;
  }

  Ok(())
}

// Error handling wrapper so platforms don't have to
pub fn main<I, O, E>(args: Cli, mut stdin: I, mut stdout: O, mut stderr: E) -> ExitCode
where
  I: Read,
  O: Write,
  E: Write,
{
  let error = match catsay(args, stdin, stdout, stderr) {
    Ok(()) => return ExitCode::SUCCESS,
    Err(err) => err,
  };
  panic!()
}
