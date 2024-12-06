mod cats;
#[cfg(feature = "cli")]
pub mod cli;
mod options;

pub use cats::*;
pub use options::*;

use fastrand;
use std::{cmp::min, usize};

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

fn write_bubble(text: &str, buf: &mut String, options: &CatsayOptions) {
  let max_bubble_width = options.max_bubble_width.unwrap_or(usize::MAX);
  let mut chars = text.chars();
  let mut lines: Vec<String> = vec![String::new()];
  let mut word = String::new();

  let mut line_len = 0;
  let mut word_len = 0;

  loop {
    match chars.next() {
      Some(w) if w.is_whitespace() => {
        lines.last_mut().unwrap().push_str(&word);
        line_len += word_len;
        word.clear();
        word_len = 0;

        if w != '\n' && line_len + 1 < max_bubble_width {
          lines.last_mut().unwrap().push(w);
          line_len += 1;
        } else {
          lines.push(String::new());
          line_len = 0;
        }
      }
      Some(c) => {
        word.push(c);
        word_len += 1;
        if word_len == max_bubble_width {
          lines.last_mut().unwrap().push_str(&word);
          lines.push(String::new());
          word.clear();
          line_len = 0;
          word_len = 0;
        } else if line_len + word_len == max_bubble_width {
          lines.push(String::new());
          lines.last_mut().unwrap().push_str(&word);
          word.clear();
          line_len = word_len;
          word_len = 0;
        }
      }
      None => {
        if line_len == 0 && word_len == 0 {
          break;
        }

        lines.last_mut().unwrap().push_str(&word);
        word.clear();
        line_len += word_len;
        word_len = 0;
        break;
      }
    }
  }

  for line in lines {
    buf.push_str(&line);
    buf.push('\n');
  }

  // Top line
  /*buf.push(' ');
  buf.push_str(&"_".repeat(max_bubble_width + 2));
  buf.push(' ');
  buf.push('\n');

  // Bottom "line"
  buf.push(' ');
  buf.push_str(&"-".repeat(max_bubble_width + 2));
  buf.push(' ');*/
}

pub fn say(text: &str, options: &CatsayOptions) -> String {
  let text_len = text.chars().count();
  let bubble_width = match options.max_bubble_width {
    Some(width) => min(text_len, width),
    None => text_len,
  };

  let mut bubble = String::new();
  write_bubble(text, &mut bubble, options);

  let cat = match &options.cat {
    CatChoice::Choice(cat) => cat,
    CatChoice::Random => {
      let i = fastrand::usize(..Cat::CATS.len());
      &Cat::CATS[i]
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
