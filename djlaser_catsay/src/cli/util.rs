use crate::Cat;

use super::CliError;

use std::io::Error;

pub fn try_get_cat(name: String) -> Result<&'static Cat<'static>, CliError> {
  let cat = Cat::get_cat(&name);
  if cat.is_none() {
    return Err(CliError::CatNotFound(name));
  }

  Ok(cat.unwrap())
}

pub fn try_parse_cat_file<F>(file: &str, read_to_string: F) -> Result<String, CliError>
where
  for<'f> F: Fn(&'f str) -> Result<String, Error>,
{
  let text = read_to_string(file).map_err(|error| match error {
    _ => CliError::CatFileNotFound(file.to_string(), error),
  })?;

  // TODO: Do parsing of complex catfiles else return simple_text
  Ok(text)
}
