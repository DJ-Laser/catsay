pub struct CatsayOptions {
  pub max_bubble_width: Option<usize>,
}

impl Default for CatsayOptions {
  fn default() -> Self {
    Self {
      max_bubble_width: Some(40),
    }
  }
}

impl CatsayOptions {
  pub fn with_max_bubble_width(mut self, width: Option<usize>) -> Self {
    self.max_bubble_width = width;
    return self;
  }
}
