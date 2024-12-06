mod io;

use clap::Parser;
use io::TerminalIo;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use xterm_js_rs::addons::fit::FitAddon;
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};

use djlaser_catsay::cli::{self, parser::Cli};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const PROMPT: &str = "$ ";

fn prompt(text: &str, term: &Terminal) {
  term.write(PROMPT);
  term.write(text);
}

fn setup_terminal() -> Result<Terminal, JsValue> {
  let terminal = Terminal::new(
    TerminalOptions::new()
      .with_rows(50)
      .with_cursor_blink(true)
      .with_cursor_width(10)
      .with_font_size(20)
      .with_draw_bold_text_in_bright_colors(true)
      .with_right_click_selects_word(true)
      .with_theme(
        Theme::new()
          .with_foreground("#98FB98")
          .with_background("#000000"),
      ),
  );

  let elem = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .get_element_by_id("terminal")
    .unwrap();

  terminal.open(elem.dyn_into()?);
  terminal.writeln("Welcome to catsay web!");
  terminal
    .writeln("Here you can use the catsay command, just like if you installed the cargo package");

  terminal.writeln("");
  prompt("", &terminal);

  return Ok(terminal);
}

fn run_command(line: &str, terminal: &Terminal) {
  // If there was no space, the whole line is the command name
  let command = line.split_once(' ').map_or(line, |s| s.0);
  match command {
    "catsay" => catsay(&terminal, &line),
    "clear" => terminal.clear(),
    _ => {
      terminal.writeln(&format!(
        "Command: {} not supported, please use the catsay or clear commands",
        command,
      ));
    }
  }
}

// Keyboard keys
// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_C: u32 = 67;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";
const CLEAR_LINE: &str = "\x1b[2K\r";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  std::panic::set_hook(Box::new(console_error_panic_hook::hook));

  let terminal = setup_terminal()?;
  let mut line = String::new();
  let mut line_len = 0;
  let mut cursor_col = 0;

  let term: Terminal = terminal.clone().dyn_into()?;

  let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
    let event = e.dom_event();
    match event.key_code() {
      KEY_ENTER => {
        term.writeln("");
        if !line.is_empty() {
          run_command(&line, &term);
        }
        line.clear();
        line_len = 0;
        cursor_col = 0;
        prompt("", &term);
      }
      KEY_LEFT_ARROW => {
        if cursor_col > 0 {
          term.write(CURSOR_LEFT);
          cursor_col -= 1;
        }
      }
      KEY_RIGHT_ARROW => {
        if cursor_col < line_len {
          term.write(CURSOR_RIGHT);
          cursor_col += 1;
        }
      }
      KEY_C if event.ctrl_key() => {
        term.writeln("^C");
        prompt("", &term);
        line.clear();
        line_len = 0;
        cursor_col = 0;
      }
      KEY_BACKSPACE => {
        let res = line.char_indices().nth(cursor_col - 1);
        if cursor_col > 0 {
          let pos = res.expect("cursor_col should be valid").0;
          line.remove(pos);
          line_len -= 1;
          cursor_col -= 1;

          term.write(CLEAR_LINE);
          prompt(
            &(line.clone() + &CURSOR_LEFT.repeat(line_len - cursor_col)),
            &term,
          );
        }
      }
      _ => {
        if event.key().len() == 1
          && !event.alt_key()
          && !event.alt_key()
          && !event.ctrl_key()
          && !event.meta_key()
        {
          let res = line.char_indices().nth(cursor_col);
          let key_len = e.key().chars().count();
          if let Some((pos, _)) = res {
            line.insert_str(pos, &e.key());
          } else {
            line.push_str(&e.key());
          }

          line_len += key_len;
          cursor_col += key_len;

          term.write(CLEAR_LINE);
          prompt(
            &(line.clone() + &CURSOR_LEFT.repeat(line_len - cursor_col)),
            &term,
          );
        }
      }
    }
  }) as Box<dyn FnMut(_)>);

  terminal.on_key(callback.as_ref().unchecked_ref());

  callback.forget();

  let addon = FitAddon::new();
  terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
  addon.fit();
  terminal.focus();

  Ok(())
}

fn catsay(terminal: &Terminal, line: &str) {
  let args = line.split(' ');
  let terminal_io = TerminalIo::new(terminal);

  let options = match Cli::try_parse_from(args) {
    Ok(cli) => cli,
    Err(error) => {
      terminal_io.write_str(&format!("{}", error));
      return;
    }
  };

  match cli::main(
    options,
    terminal_io.clone(),
    terminal_io.clone(),
    terminal_io,
  ) {
    Ok(()) => return,
    Err(_) => return,
  };
}
