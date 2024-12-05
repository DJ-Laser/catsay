mod cats;
mod options;
mod web_term;

pub use cats::*;
pub use options::*;

use std::{cmp::min, iter};

pub const CREDITS: &str = "\
Art sourced from ASCII Art Archive https://www.asciiart.eu/animals/cats
";

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

pub fn say(text: &str, options: &CatsayOptions) -> String {
  let text_len = text.chars().count();
  let bubble_width = match options.max_bubble_width {
    Some(width) => min(text_len, width),
    None => text_len,
  };

  // Add one line if text / width has remainder, meaning it overflows using floor division
  let bubble_num_lines = text_len / bubble_width + min(text_len % bubble_width, 1);

  let mut bubble = String::from(' ');
  // Top line
  bubble.push_str(&"_".repeat(bubble_width + 2));
  bubble.push(' ');
  bubble.push('\n');

  // We want to use spaces to pad out the bubble if there are not enough chars
  let mut chars = text.chars().chain(iter::repeat(' '));
  for i in 0..bubble_num_lines {
    let line_type = match i {
      0 if bubble_num_lines == 1 => BubbleVerticalLine::Single,
      0 => BubbleVerticalLine::Top,
      n if n == bubble_num_lines - 1 => BubbleVerticalLine::Bottom,
      _ => BubbleVerticalLine::Middle,
    };

    bubble.push_str(line_type.get_start());
    let line: String = chars.by_ref().take(bubble_width).collect();
    bubble.push_str(&line);
    bubble.push_str(line_type.get_end());
    bubble.push('\n');
  }

  // Bottom "line"
  bubble.push(' ');
  bubble.push_str(&"-".repeat(bubble_width + 2));
  bubble.push(' ');

  for car in Cat::CATS {
    println!("{}", car.get_art());
  }
  return bubble;
}

pub fn say_default(text: &str) -> String {
  return say(text, &Default::default());
}
