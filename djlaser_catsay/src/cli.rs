mod actions;
pub mod parser;
mod util;

use std::io::{Error, ErrorKind, Read, Write};
use std::process::ExitCode;

use crate::{Cat, CatsayOptions};
use parser::Cli;
use util::{try_get_cat, try_parse_cat_file};

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

  if let Some(ref path) = args.action.show_cat_file {
    let cat_text = try_parse_cat_file(&path, read_to_string)?;
    let file_cat = Cat {
      name: "Unknown",
      credit: "Unknown Cat by Unknown Artist",
      art: &cat_text,
    };

    options.set_cat(&file_cat);
    return actions::show_cat(&file_cat.credit, options, stdout);
  }

  if let Some(name) = args.action.show_cat {
    let cat = try_get_cat(name)?;
    options.set_cat(cat);
    return actions::show_cat(&cat.credit, options, stdout);
  }

  if args.action.list_cats {
    return actions::list_cats(stdout);
  }

  if args.action.show_cats {
    return actions::show_cats(options, stdout);
  }

  if args.action.credits {
    return actions::credits(stdout);
  }

  let cat_text;
  let mut file_cat = Cat {
    name: "Unknown",
    credit: "Unknown Cat by Unknown Artist",
    art: "",
  };

  if let Some(name) = args.cat {
    options.set_cat(try_get_cat(name)?);
  } else if let Some(path) = args.file {
    cat_text = try_parse_cat_file(&path, read_to_string)?;
    file_cat.art = &cat_text;

    options.set_cat(&file_cat);
  }

  // All other actions were false, default to catsaying
  let text = if args.action.use_stdin {
    let mut buf: String = String::new();
    stdin.read_to_string(&mut buf)?;
    buf
  } else {
    args.action.say.join(" ")
  };

  return actions::catsay(&text, options, stdout);
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
