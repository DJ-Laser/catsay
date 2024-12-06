mod cats;
#[cfg(feature = "cli")]
pub mod cli;
mod options;

pub use cats::*;
pub use options::*;

use fastrand;
use std::{cmp::max, usize};

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
  let mut lines: Vec<(String, usize)> = vec![(String::new(), 0)];
  let mut word = String::new();

  let mut word_len = 0;

  loop {
    let line = lines.last_mut().unwrap();
    match chars.next() {
      Some(w) if w.is_whitespace() => {
        line.0.push_str(&word);
        line.1 += word_len;
        word.clear();
        word_len = 0;

        if w != '\n' && line.1 + 1 < max_bubble_width {
          line.0.push(w);
          line.1 += 1;
        } else {
          lines.push((String::new(), 0));
        }
      }
      Some(c) => {
        word.push(c);
        word_len += 1;
        if word_len == max_bubble_width {
          line.0.push_str(&word);
          line.1 += word_len;
          lines.push((String::new(), 0));
          word.clear();
          word_len = 0;
        } else if line.1 + word_len == max_bubble_width {
          lines.push((String::new(), 0));
          let line = lines.last_mut().unwrap();
          line.0.push_str(&word);
          line.1 = word_len;
          word.clear();
          word_len = 0;
        }
      }
      None => {
        if line.1 == 0 && word_len == 0 {
          break;
        }

        line.0.push_str(&word);
        line.1 += word_len;
        word.clear();
        break;
      }
    }
  }

  let bubble_width = lines.iter().fold(0usize, |a, (_, b)| max(a, *b));
  for line in &mut lines {
    line.0.push_str(&" ".repeat(bubble_width - line.1));
  }

  // Top line
  buf.push(' ');
  buf.push_str(&"_".repeat(bubble_width + 2));
  buf.push(' ');
  buf.push('\n');

  let mut i = 0;
  for (line, _) in &lines {
    let line_type = match i {
      0 if lines.len() == 1 => BubbleVerticalLine::Single,
      0 => BubbleVerticalLine::Top,
      n if n == lines.len() - 1 => BubbleVerticalLine::Bottom,
      _ => BubbleVerticalLine::Middle,
    };

    buf.push_str(line_type.get_start());
    buf.push_str(&line);
    buf.push_str(line_type.get_end());
    buf.push('\n');
    i += 1;
  }

  // Bottom "line"
  buf.push(' ');
  buf.push_str(&"-".repeat(bubble_width + 2));
  buf.push(' ');
}

pub fn say(text: &str, options: &CatsayOptions) -> String {
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
