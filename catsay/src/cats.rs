use std::str::{self, from_utf8};

pub struct Cat {
  pub name: &'static str,
  pub desc: &'static str,
  pub art: &'static str,
}

pub const CATS: [Cat; 7] = [
  Cat {
    name: "peeking",
    desc: "Peeking cat: Unknown Artist",
    art: r"
    |\__/,|   (`\
  _.|o o  |_   ) )
-(((---(((--------",
  },
  Cat {
    name: "sleeping",
    desc: "Sleeping cat: Felix Lee",
    art: r"
      |\      _,,,---,,_
ZZZzz /,`.-'`'    -.  ;-;;,_
     |,4-  ) )-,_. ,\ (  `'-'
    '---''(_/--'  `-'\_)  Felix Lee",
  },
  Cat {
    name: "surprised",
    desc: "Surprised cat: Unknown Artist",
    art: r#"
 _._     _,-'""`-._
(,-.`._,'(       |\`-/|
    `-.-' \ )-`( , o o)
          `-    \`_`"'-"#,
  },
  Cat {
    name: "contented",
    desc: "Contented cat: Unknown Artist",
    art: r"
 |\__/,|   (`\
 |_ _  |.--.) )
 ( T   )     /
(((^_(((/(((_/",
  },
  Cat {
    name: "playing",
    desc: "Playing cat: Julie Rhodes",
    art: r"
    /\_/\           ___
   = o_o =_______    \ \  -Julie Rhodes-
    __^      __(  \.__) )
(@)<_____>__(_____)____/",
  },
  Cat {
    name: "hissing",
    desc: "Hissing cat: Hayley Jane Wakenshaw",
    art: r#"
        ,-""""""-.
     /\j__/\  (  \`--.
hjw  \`@_@'/  _)  >--.`.
    _{.:Y:_}_{{_,'    ) )
   {_}`-^{_} ```     (_/"#,
  },
  Cat {
    name: "sitting",
    desc: "Sitting cat: Unknown Artist",
    art: r#"
   |\---/|
   | ,_, |
    \_`_/-..----.
 ___/ `   ' ,""+ \  sk
(__...'   __\    |`.___.';
  (_,...'(_,.`__)/'.....+"#,
  },
];

pub fn get_cat_art(index: usize) -> &'static str {
  let art = CATS[index].art;
  // Remove leading newline
  let bytes = &art.as_bytes()[1..];
  return str::from_utf8(bytes).expect("Should be able to remove the leading newline");
}
