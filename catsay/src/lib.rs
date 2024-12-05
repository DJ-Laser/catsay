mod cats;
mod options;

pub use cats::*;
pub use options::*;

use rand::{seq::SliceRandom, thread_rng};
use std::{cmp::min, iter};

pub fn get_credits() -> String {
  let mut credits =
    String::from("Art sourced from ASCII Art Archive https://www.asciiart.eu/animals/cats");
  for cat in Cat::CATS {
    credits.push('\n');
    credits.push_str(&cat.credit);
  }

  return credits;
}

pub fn get_help_text() -> String {
  let mut help_text = get_credits();
  help_text.insert_str(
    0,
    "\
Catsay written by DJ_Laser
Usage: catsay <text>
\n",
  );
  return help_text;
}

enum BubbleVerticalLine {
  Single,
  Top,
  Middle,
  Bottom,
}

impl BubbleVerticalLine {
  fn get_start(&self) -> &'static str {
    match self {
      Self::Single => "< ",
      Self::Top => "/ ",
      Self::Middle => "| ",
      Self::Bottom => r"\ ",
    }
  }

  fn get_end(&self) -> &'static str {
    match self {
      Self::Single => " >",
      Self::Top => r" \",
      Self::Middle => " |",
      Self::Bottom => " /",
    }
  }
}

fn write_bubble(text: &str, buf: &mut String, bubble_width: usize) {
  let text_len = text.chars().count();

  // Add one line if text / width has remainder, meaning it overflows using floor division
  let bubble_num_lines = text_len / bubble_width + min(text_len % bubble_width, 1);

  // Top line
  buf.push(' ');
  buf.push_str(&"_".repeat(bubble_width + 2));
  buf.push(' ');
  buf.push('\n');

  // We want to use spaces to pad out the bubble if there are not enough chars
  let mut chars = text.chars().chain(iter::repeat(' '));
  for i in 0..bubble_num_lines {
    let line_type = match i {
      0 if bubble_num_lines == 1 => BubbleVerticalLine::Single,
      0 => BubbleVerticalLine::Top,
      n if n == bubble_num_lines - 1 => BubbleVerticalLine::Bottom,
      _ => BubbleVerticalLine::Middle,
    };

    buf.push_str(line_type.get_start());
    let line: String = chars.by_ref().take(bubble_width).collect();
    buf.push_str(&line);
    buf.push_str(line_type.get_end());
    buf.push('\n');
  }

  // Bottom "line"
  buf.push(' ');
  buf.push_str(&"-".repeat(bubble_width + 2));
  buf.push(' ');
}

pub fn say(text: &str, options: &CatsayOptions) -> String {
  let text_len = text.chars().count();
  let bubble_width = match options.max_bubble_width {
    Some(width) => min(text_len, width),
    None => text_len,
  };

  let mut bubble = String::new();
  write_bubble(text, &mut bubble, bubble_width);

  let cat = match &options.cat {
    CatChoice::Choice(cat) => &cat,
    CatChoice::Random => {
      let mut rng = thread_rng();
      Cat::CATS.choose(&mut rng).expect("")
    }
  };

  for line in cat.get_art().lines() {
    bubble.push('\n');
    bubble.push_str(&" ".repeat(options.left_padding));
    bubble.push_str(line);
  }

  return bubble;
}

pub fn say_default(text: &str) -> String {
  return say(text, &Default::default());
}
