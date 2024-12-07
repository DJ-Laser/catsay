use std::fmt::{Debug, Display};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::str;
use xterm_js_rs::Terminal;

#[derive(Clone)]
pub struct TerminalIo<'a>(&'a Terminal);

impl<'a> TerminalIo<'a> {
  pub fn new(terminal: &'a Terminal) -> Self {
    TerminalIo(terminal)
  }

  pub fn write_str(&mut self, string: &str) {
    for line in string.split('\n') {
      if line.len() == 0 {
        continue;
      }

      self.0.writeln(line);
    }
  }
}

impl<'a> Write for TerminalIo<'a> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match str::from_utf8(buf) {
      Ok(string) => {
        self.write_str(string);
        io::Result::Ok(buf.len())
      }
      Err(error) => io::Result::Err(Error::new(ErrorKind::InvalidData, error)),
    }
  }

  fn flush(&mut self) -> io::Result<()> {
    io::Result::Ok(())
  }
}

impl<'a> Read for TerminalIo<'a> {
  fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
    self.0.writeln(&format!("{}", StdinNotSupported));
    io::Result::Err(Error::new(ErrorKind::BrokenPipe, StdinNotSupported))
  }
}

#[derive(Debug)]
pub struct StdinNotSupported;

impl Display for StdinNotSupported {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Standard input not supported in web mode, sorry")
  }
}

impl std::error::Error for StdinNotSupported {}
