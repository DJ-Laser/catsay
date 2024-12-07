use crate::Cat;

#[derive(Clone)]
pub enum CatChoice<'a> {
  Choice(&'a Cat<'a>),
  Random,
}

#[derive(Clone)]
pub struct CatsayOptions<'a> {
  /// Maximum width of the speech bubble (does not include the +4 for padding and outline)
  pub max_bubble_width: Option<usize>,
  /// Where to place the bubble if it isn't long enough
  pub bubble_offset: usize,
  /// Number of spaces to pad the cat from the left
  pub left_padding: usize,
  /// Cat art to use
  pub cat: CatChoice<'a>,
}

impl<'a> Default for CatsayOptions<'a> {
  fn default() -> Self {
    Self {
      max_bubble_width: Some(40),
      left_padding: 8,
      bubble_offset: 0,
      cat: CatChoice::Random,
    }
  }
}

impl<'a> CatsayOptions<'a> {
  pub fn set_random_cat(&mut self) {
    self.cat = CatChoice::Random;
  }

  pub fn set_cat(&mut self, cat: &'a Cat<'a>) {
    self.cat = CatChoice::Choice(cat);
  }

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
    self.set_random_cat();
    return self;
  }

  pub fn with_cat(mut self, cat: &'a Cat<'a>) -> Self {
    self.set_cat(cat);
    return self;
  }

  pub fn with_cat_choice(mut self, choice: CatChoice<'a>) -> Self {
    self.cat = choice;
    return self;
  }
}
