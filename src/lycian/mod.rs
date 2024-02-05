use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 30] = [
    "a", "b", "g", "d", "i", "w", "z", "h", "j", "y", "k", "l", "m", "n", "u", "p", "k", "r", "s",
    "t", "e", "ã", "ẽ", "M", "N", "T", "q", "B", "x", "th",
];

const UNI_VALUES: [&str; 30] = [
    "𐊀", "𐊂", "𐊄", "𐊅", "𐊆", "𐊇", "𐊈", "𐊛", "𐊊", "𐊊", "𐊋", "𐊍", "𐊎", "𐊏", "𐊒", "𐊓", "𐊔", "𐊕", "𐊖",
    "𐊗", "𐊁", "𐊙", "𐊚", "𐊐", "𐊑", "𐊘", "𐊌", "𐊃", "𐊜", "𐊉",
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
    output = ascii_to_unicode(output);
    output
}
