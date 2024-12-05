use std::str::{self, from_utf8};

pub struct Cat {
  pub name: &'static str,
  pub credit: &'static str,
  art: &'static str,
}

impl Cat {
  pub const CATS: &[Cat; 7] = &CATS;
  pub fn get_art(&self) -> &'static str {
    let bytes = &self.art.as_bytes()[1..];
    return from_utf8(bytes).expect("Should be able to remove the leading newline");
  }
}

const CATS: [Cat; 7] = [
  Cat {
    name: "peeking",
    credit: "Peeking cat by Unknown Artist",
    art: r"
    |\__/,|   (`\
  _.|o o  |_   ) )
-(((---(((--------",
  },
  Cat {
    name: "sleeping",
    credit: "Sleeping cat by Felix Lee",
    art: r"
      |\      _,,,---,,_
ZZZzz /,`.-'`'    -.  ;-;;,_
     |,4-  ) )-,_. ,\ (  `'-'
    '---''(_/--'  `-'\_)  Felix Lee",
  },
  Cat {
    name: "surprised",
    credit: "Surprised cat by Unknown Artist",
    art: r#"
 _._     _,-'""`-._
(,-.`._,'(       |\`-/|
    `-.-' \ )-`( , o o)
          `-    \`_`"'-"#,
  },
  Cat {
    name: "contented",
    credit: "Contented cat by Unknown Artist",
    art: r"
 |\__/,|   (`\
 |_ _  |.--.) )
 ( T   )     /
(((^_(((/(((_/",
  },
  Cat {
    name: "playing",
    credit: "Playing cat by Julie Rhodes",
    art: r"
    /\_/\           ___
   = o_o =_______    \ \  -Julie Rhodes-
    __^      __(  \.__) )
(@)<_____>__(_____)____/",
  },
  Cat {
    name: "hissing",
    credit: "Hissing cat by Hayley Jane Wakenshaw",
    art: r#"
        ,-""""""-.
     /\j__/\  (  \`--.
hjw  \`@_@'/  _)  >--.`.
    _{.:Y:_}_{{_,'    ) )
   {_}`-^{_} ```     (_/"#,
  },
  Cat {
    name: "sitting",
    credit: "Sitting cat by Unknown Artist",
    art: r#"
   |\---/|
   | ,_, |
    \_`_/-..----.
 ___/ `   ' ,""+ \  sk
(__...'   __\    |`.___.';
  (_,...'(_,.`__)/'.....+"#,
  },
];
