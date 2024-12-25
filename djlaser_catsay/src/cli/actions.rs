use crate::{get_credits, say, Cat, CatsayOptions};
use std::io::Write;

use super::CliError;

pub fn credits<O: Write>(mut stdout: O) -> Result<(), CliError> {
  writeln!(stdout, "{}", get_credits())?;
  Ok(())
}

pub fn list_cats<O: Write>(mut stdout: O) -> Result<(), CliError> {
  for cat in Cat::CATS {
    writeln!(stdout, "--cat {}: {}", cat.name, cat.credit)?;
  }
  Ok(())
}

pub fn show_cat<O: Write>(
  credit: &str,
  options: CatsayOptions,
  mut stdout: O,
) -> Result<(), CliError> {
  writeln!(stdout, "{}", say(&credit, &options))?;
  Ok(())
}

pub fn show_cats<O: Write>(options: CatsayOptions, mut stdout: O) -> Result<(), CliError> {
  let mut options = options.clone();
  for cat in Cat::CATS {
    let text = format!("--cat {}:\n{}", cat.name, cat.credit);
    options.set_cat(cat);
    writeln!(stdout, "{}", say(&text, &options))?;
  }
  Ok(())
}

pub fn catsay<O: Write>(text: &str, options: CatsayOptions, mut stdout: O) -> Result<(), CliError> {
  writeln!(stdout, "{}", say(text, &options))?;
  Ok(())
}
