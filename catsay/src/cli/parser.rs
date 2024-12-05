use clap::Parser;

#[derive(Parser)]
#[command(name = "catsay")]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// The name of the cat to use
  /// Use --list-cats to see all cat names
  #[arg(short, long)]
  pub cat: Option<String>,

  /// Padding in spaces before the ascii image
  #[arg(long)]
  pub padding: Option<usize>,

  pub text: Vec<String>,
}
