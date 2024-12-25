#[deny(missing_docs)]
use clap::{Args, Parser};

#[derive(Parser)]
#[command(name = "catsay")]
#[command(before_help = super::CREDIT)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(flatten)]
  pub action: Action,

  /// The cat image to use.
  /// Use --list-cats to see all cat names.
  #[arg(display_order = 0)]
  #[arg(short, long, requires = "random_cat")]
  pub cat: Option<String>,

  /// The cat file to load from.
  /// Can be any file with ascii text but the .cat extension is standard
  #[arg(display_order = 0)]
  #[arg(short, long, requires = "random_cat", conflicts_with = "cat")]
  pub file: Option<String>,

  /// Padding in spaces before the ascii image.
  #[arg(long, display_order = 0)]
  pub padding: Option<usize>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Action {
  /// Show the credits for catsay
  #[arg(long, display_order = 10)]
  pub credits: bool,

  /// List the names of the available cat images.
  #[arg(display_order = 0)]
  #[arg(short, long)]
  pub list_cats: bool,

  /// Display a single cat image.
  #[arg(display_order = 0)]
  #[arg(long, value_name = "CAT")]
  pub show_cat: Option<String>,

  /// Display all of the available cat images.
  #[arg(display_order = 0)]
  #[arg(long)]
  pub show_cats: bool,

  /// Use input from stdin instead of command agruments.
  #[arg(display_order = 0)]
  #[arg(long, group = "random_cat")]
  pub use_stdin: bool,

  /// Text to be placed into the speech bubble.
  /// Includes spaces.
  #[arg(id = "TEXT", trailing_var_arg = true)]
  #[arg(group = "random_cat")]
  pub say: Vec<String>,
}
