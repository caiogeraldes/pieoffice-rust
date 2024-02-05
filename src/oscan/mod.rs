use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 22] = [
    "a", "b", "g", "k", "d", "e", "v", "z", "h", "i", "l", "m", "n", "p", "ś", "r", "s", "t", "u",
    "f", "ú", "í",
];

const UNI_VALUES: [&str; 22] = [
    "𐌀", "𐌁", "𐌂", "𐌂", "𐌃", "𐌄", "𐌅", "𐌆", "𐌇", "𐌉", "𐌋", "𐌌", "𐌍", "𐌐", "𐌑", "𐌓", "𐌔", "𐌕", "𐌖",
    "𐌚", "𐌞", "𐌝",
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
    output = output.replace('/', "\u{0301}");
    output = output.nfkc().collect::<String>();
    output = ascii_to_unicode(output);
    output
}
