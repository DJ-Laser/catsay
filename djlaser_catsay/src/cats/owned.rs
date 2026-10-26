use super::Cat;

#[derive(Clone)]
pub struct OwnedCat {
  pub name: String,
  pub credit: String,
  /// Contains a leading newline, use get_art for the trimmed version
  pub art: String,
}

impl OwnedCat {
  pub fn get_art(&self) -> &str {
    if matches!(self.art.chars().next(), Some('\n')) {
      &self.art[1..]
    } else {
      &self.art
    }
  }

  pub fn as_ref_cat(&self) -> Cat {
    Cat {
      name: &self.name,
      credit: &self.credit,
      art: &self.art,
    }
  }

  pub fn as_ref_cat_mut(&mut self) -> Cat {
    Cat {
      name: &mut self.name,
      credit: &mut self.credit,
      art: &mut self.art,
    }
  }
}

impl<'a> Cat<'a> {
  pub fn to_owned_cat(&self) -> OwnedCat {
    OwnedCat {
      name: self.name.to_owned(),
      credit: self.credit.to_owned(),
      art: self.art.to_owned(),
    }
  }
}
