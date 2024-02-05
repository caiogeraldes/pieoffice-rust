use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 45] = [
    "A/", "A", "I/", "I", "U/", "U", "lRR/", "lRR", "lR/", "lR", "LL/", "LL", "L/", "L", "RR/",
    "RR", "R/", "R", "cw", "c", "k'", "kw", "k'", "jh", "jw", "j", "bh", "dh", "gwh", "gh", "gw",
    "h1", "h2", "h3", "y", "w", "u̯'", "E/", "O/", "O", "É", "E", "Ó", "M", "N",
];

const UNI_VALUES: [&str; 45] = [
    "ā́", "ā", "ī́", "ī", "ū́", "ū", "l̥̄́", "l̥̄", "ĺ̥", "l̥", "l̥̄́", "l̥̄", "ĺ̥", "l̥", "r̥̄́", "r̥̄", "ŕ̥", "r̥", "k̑ʷ",
    "k̑", "ḱ", "kʷ", "ǵ", "ĝʰ", "ĝʷ", "ĝ", "bʰ", "dʰ", "gʷʰ", "gʰ", "gʷ", "h₁", "h₂", "h₃", "i̯",
    "u̯", "w", "ḗ", "ṓ", "ō", "ḗ", "ē", "ṓ", "m̥", "n̥",
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
