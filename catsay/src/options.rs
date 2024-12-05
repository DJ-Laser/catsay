use crate::Cat;

pub enum CatChoice {
  Choice(Cat),
  Random,
}

pub struct CatsayOptions {
  /// Maximum width of the speech bubble (does not include the +4 for padding and outline)
  pub max_bubble_width: Option<usize>,
  /// Where to place the bubble if it isn't long enough
  pub bubble_offset: usize,
  /// Number of spaces to pad the cat from the left
  pub left_padding: usize,
  /// Cat art to use
  pub cat: CatChoice,
}

impl Default for CatsayOptions {
  fn default() -> Self {
    Self {
      max_bubble_width: Some(40),
      left_padding: 8,
      bubble_offset: 0,
      cat: CatChoice::Random,
    }
  }
}

impl CatsayOptions {
  pub fn with_max_bubble_width(mut self, width: Option<usize>) -> Self {
    self.max_bubble_width = width;
    return self;
  }

  pub fn with_padding(mut self, padding: usize) -> Self {
    self.left_padding = padding;
    return self;
  }

  pub fn with_offset(mut self, offset: usize) -> Self {
    self.bubble_offset = offset;
    return self;
  }

  pub fn with_random_cat(mut self) -> Self {
    self.cat = CatChoice::Random;
    return self;
  }

  pub fn with_cat(mut self, cat: Cat) -> Self {
    self.cat = CatChoice::Choice(cat);
    return self;
  }

  pub fn with_cat_choice(mut self, choice: CatChoice) -> Self {
    self.cat = choice;
    return self;
  }
}
