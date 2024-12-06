use std::str::{self, from_utf8};

pub struct Cat<'a> {
  pub name: &'a str,
  pub credit: &'a str,
  /// Contains a leading newline, use get_art for the trimmed version
  pub art: &'a str,
}

impl<'a> Cat<'a> {
  pub const CATS: &'static [Cat<'static>; 6] = &CATS;
  pub fn get_art(&self) -> &str {
    let bytes = &self.art.as_bytes()[1..];
    return from_utf8(bytes).expect("Should be able to remove the leading newline");
  }

  pub fn get_cat(name: &str) -> Option<&'static Cat<'static>> {
    for cat in Self::CATS {
      if cat.name == name {
        return Some(cat);
      }
    }

    None
  }
}

const CATS: [Cat; 6] = [
  Cat {
    name: "peeking",
    credit: "Peeking cat by Unknown Artist",
    art: r"
\ 
 \  |\__/,|   (`\
  _.|o o  |_   ) )
-(((---(((--------",
  },
  Cat {
    name: "sleeping",
    credit: "Sleeping cat by Felix Lee",
    art: r"
Z
 z  |\      _,,,---,,_
  z /,`.-'`'    -.  ;-;;,_
   |,4-  ) )-,_. ,\ (  `'-'
  '---''(_/--'  `-'\_)  Felix Lee",
  },
  Cat {
    name: "contented",
    credit: "Contented cat by Unknown Artist",
    art: r"
\   |\__/,|   (`\
 \  |_ _  |.--.) )
    ( T   )     /
   (((^_(((/(((_/",
  },
  Cat {
    name: "playing",
    credit: "Playing cat by Julie Rhodes",
    art: r"
\    /\_/\           ___
 \  = o_o =_______    \ \  -Julie Rhodes-
     __^      __(  \.__) )
 (@)<_____>__(_____)____/",
  },
  Cat {
    name: "hissing",
    credit: "Hissing cat by Hayley Jane Wakenshaw",
    art: r#"
 \      ,-""""""-.
  \  /\j__/\  (  \`--.
hjw  \`@_@'/  _)  >--.`.
    _{.:Y:_}_{{_,'    ) )
   {_}`-^{_} ```     (_/"#,
  },
  Cat {
    name: "sitting",
    credit: "Sitting cat by Unknown Artist",
    art: r#"
\  |\---/|
 \ | ,_, |
    \_`_/-..----.
 ___/ `   ' ,""+ \  sk
(__...'   __\    |`.___.';
  (_,...'(_,.`__)/'.....+"#,
  },
];
