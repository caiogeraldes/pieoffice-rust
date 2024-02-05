use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 33] = [
    "a", "b", "p", "g", "d", "e", "v", "w", "i", "y", "k", "l", "m", "n", "o", "r", "S", "ś", "t",
    "u", "f", "q", "s", "sh", "T", "ã", "A", "ẽ", "E", "L", "N", "c", ".",
];

const UNI_VALUES: [&str; 33] = [
    "𐤠", "𐤡", "𐤡", "𐤢", "𐤣", "𐤤", "𐤥", "𐤥", "𐤦", "𐤧", "𐤨", "𐤩", "𐤪", "𐤫", "𐤬", "𐤭", "𐤮", "𐤮", "𐤯",
    "𐤰", "𐤱", "𐤲", "𐤳", "𐤳", "𐤴", "𐤵", "𐤵", "𐤶", "𐤶", "𐤷", "𐤸", "𐤹", "",
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
