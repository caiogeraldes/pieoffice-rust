use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 30] = [
    "ee", "EE", "e'", "E'", "zh", "ch.", "c'h", "C'h", "ch", "sh", "jh", "ZH", "CH.", "SH", "CH",
    "JH", "o'", "O'", "g.", "l.", "r.", "R.", "G.", "L.", "C", "J", "Ow", "c", "j", "ow",
];

const UNI_VALUES: [&str; 30] = [
    "ē", "Ē", "ë", "Ë", "ž", "č̣", "č̣", "Č̣", "č", "š", "ǰ", "Ž", "Č̣", "Š", "Č", "ǰ", "ò", "Ò", "ġ",
    "ġ", "ṙ", "Ṙ", "Ġ", "Ġ", "Ç", "Dz", "U", "ç", "dz", "u",
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
    output = output.nfkc().collect::<String>();
    output
}
