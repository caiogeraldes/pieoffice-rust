use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 27] = [
    "th", "q'", "z'", "hw", "a", "b", "g", "d", "e", "q", "z", "h", "i", "k", "l", "m", "n", "j",
    "u", "p", "r", "s", "t", "w", "f", "x", "o",
];

const UNI_VALUES: [&str; 27] = [
    "𐌸", "𐍁", "𐍊", "𐍈", "𐌰", "𐌱", "𐌲", "𐌳", "𐌴", "𐌵", "𐌶", "𐌷", "𐌹", "𐌺", "𐌻", "𐌼", "𐌽", "𐌾", "𐌿",
    "𐍀", "𐍂", "𐍃", "𐍄", "𐍅", "𐍆", "𐍇", "𐍉",
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
