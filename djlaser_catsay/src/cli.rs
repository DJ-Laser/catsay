pub mod parser;

use std::io::{Error, ErrorKind, Read, Write};
use std::process::ExitCode;

use crate::{say, Cat, CatChoice, CatsayOptions};
use parser::Cli;

enum CliError {
  IoError(Error),
  CatNotFound(String),
  CatFileNotFound(String, Error),
}

impl From<Error> for CliError {
  fn from(error: Error) -> CliError {
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

fn try_get_cat(name: String) -> Result<&'static Cat<'static>, CliError> {
  let cat = Cat::get_cat(&name);
  if cat.is_none() {
    return Err(CliError::CatNotFound(name));
  }

  Ok(cat.unwrap())
}

fn try_parse_cat_file<F>(file: &str, read_to_string: F) -> Result<String, CliError>
where
  for<'f> F: Fn(&'f str) -> Result<String, Error>,
{
  let text = read_to_string(file).map_err(|error| match error {
    _ => CliError::CatFileNotFound(file.to_string(), error),
  })?;

  // get_art removes a leading newline
  let simple_text = " ".to_string() + &text;
  // TODO: Do parsing of complex catfiles else return simple_text
  Ok(simple_text)
}

fn catsay<I, O, E, F>(
  args: Cli,
  stdin: &mut I,
  stdout: &mut O,
  _stderr: &mut E,
  read_to_string: F,
) -> Result<(), CliError>
where
  I: Read,
  O: Write,
  E: Write,
  for<'f> F: Fn(&'f str) -> Result<String, Error>,
{
  let mut options: CatsayOptions = Default::default();

  if let Some(padding) = args.padding {
    options.left_padding = padding;
  }

  if let Some(name) = args.cat {
    options.cat = CatChoice::Choice(try_get_cat(name)?);
  }

  let cat_text;
  let mut file_cat = Cat {
    name: "Unknown",
    credit: "Unknown Cat by Unknown Artist",
    art: "",
  };

  if let Some(path) = args.file {
    cat_text = try_parse_cat_file(&path, read_to_string)?;
    file_cat.art = &cat_text;

    options.cat = CatChoice::Choice(&file_cat);
  }

  if args.action.credits {
    writeln!(stdout, "{}", get_credits())?;
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
    let cat = try_get_cat(name)?;
    options.set_cat(cat);
    writeln!(stdout, "{}", say(&cat.credit, &options))?;
  } else {
    let text = if args.action.use_stdin {
      let mut buf: String = String::new();
      stdin.read_to_string(&mut buf)?;
      buf
    } else {
      args.action.say.join(" ")
    };

    writeln!(stdout, "{}", say(&text, &options))?;
  }

  Ok(())
}

// Error handling wrapper so platforms don't have to
pub fn main<I, O, E, F>(
  args: Cli,
  stdin: &mut I,
  stdout: &mut O,
  stderr: &mut E,
  read_to_string: F,
) -> ExitCode
where
  I: Read,
  O: Write,
  E: Write,
  for<'f> F: Fn(&'f str) -> Result<String, Error>,
{
  let error = match catsay(args, stdin, stdout, stderr, read_to_string) {
    Ok(()) => return ExitCode::SUCCESS,
    Err(err) => err,
  };

  let io_error = match error {
    CliError::IoError(err) => err,
    CliError::CatNotFound(name) => {
      let res = writeln!(
        stderr,
        r#"No cat "{name}" available. Use --list-cats or --show-cats to see available cats"#
      );
      match res {
        Ok(()) => return ExitCode::from(2),
        Err(io_error) => io_error,
      }
    }
    CliError::CatFileNotFound(file, error) => {
      let res = writeln!(stderr, r#"Could not get catfile "{file}": {error}"#);
      match res {
        Ok(()) => return ExitCode::from(2),
        Err(io_error) => io_error,
      }
    }
  };

  // Ignore failure to write, that might be what caused the error
  let _ = writeln!(stderr, "Error: {}", io_error);
  match io_error.kind() {
    ErrorKind::InvalidData => ExitCode::from(65),
    _ => ExitCode::from(1),
  }
}
