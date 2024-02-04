use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 31] = [
    ",ear,",
    ",or,",
    ",uilleann,",
    ",ifin,",
    ",eam,",
    ",peith, ",
    "b",
    "l",
    "w",
    "s",
    "n",
    "j",
    "h",
    "d",
    "t",
    "kw ",
    "k",
    "cw ",
    "c",
    "m",
    "gw ",
    "g",
    "S",
    "r",
    "a",
    "o",
    "u",
    "e",
    "i",
    ">",
    "<",
];

const UNI_VALUES: [&str; 31] = [
    "ᚕ", "ᚖ", "ᚗ", "ᚘ", "ᚙ", "ᚚ", "ᚁ", "ᚂ", "ᚃ", "ᚄ", "ᚅ", "ᚆ", "ᚆ", "ᚇ", "ᚈ", "ᚊ", "ᚉ", "ᚊ", "ᚉ",
    "ᚋ", "ᚍ", "ᚌ", "ᚎ", "ᚏ", "ᚐ", "ᚑ", "ᚒ", "ᚓ", "ᚔ", "᚛", "᚜",
];

lazy_static! {
    static ref ENTRY_TO_UNI: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        for (b, u) in ENTRY_VALUES.iter().zip(UNI_VALUES.iter()) {
            m.insert(*b, *u);
        }
        m
    };
}

fn ascii_to_unicode<T: Into<String>>(input: T) -> String {
    let mut output: String = input.into();
    ENTRY_VALUES.iter().for_each(|c| {
        output = output.replace(*c, ENTRY_TO_UNI.get(c).unwrap());
    });

    output
}

pub fn convert<T: Into<String>>(input: T) -> String {
    let mut output = input.into();
    output = output.replace("/", "\u{0301}");
    output = output.nfkc().collect::<String>();
    output = ascii_to_unicode(output);
    output
}
