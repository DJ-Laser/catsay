#[deny(missing_docs)]
use clap::{Args, Parser};

#[derive(Parser)]
#[command(name = "catsay")]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(flatten)]
  pub action: Action,

  /// The cat image to use.
  /// Use --list-cats to see all cat names
  #[arg(display_order = 0)]
  #[arg(short, long, requires = "TextSource")]
  pub cat: Option<String>,

  /// Padding in spaces before the ascii image
  #[arg(display_order = 0)]
  #[arg(long, requires = "TextSource")]
  pub padding: Option<usize>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct TextSource {
  #[arg(id = "TEXT", trailing_var_arg = true)]
  pub args: Vec<String>,

  /// Use input from stdin instead of command agruments
  #[arg(display_order = 0)]
  #[arg(long)]
  pub use_stdin: bool,
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Action {
  #[command(flatten)]
  pub say: TextSource,

  /// List the names of the available cat images
  #[arg(display_order = 0)]
  #[arg(short, long)]
  pub list_cats: bool,

  /// Display a single cat image
  #[arg(display_order = 0)]
  #[arg(long, value_name = "CAT")]
  pub show_cat: Option<String>,

  /// Display all of the available cat images
  #[arg(display_order = 0)]
  #[arg(long)]
  pub show_cats: bool,
}
