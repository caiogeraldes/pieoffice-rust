use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 61] = [
    "A", "aa", "a", "Á", "áá", "á", "ãã", "ã", "ææ", "æ", "Æ", "êê", "ê", "Ê", "ee", "e", "E",
    "oo", "o", "O", "i", "I", "u", "U", "k", "X", "xw", "x", "c", "j", "th", "t", "dh", "d", "T",
    "p", "f", "b", "B", "ngh", "ngw", "ng", "gh", "g", "G", "ñ", "n", "N", "m", "M", "Y", "y", "v",
    "r", "shy", "sh", "zh", "S", "s", "z", "h",
];

const UNI_VALUES: [&str; 61] = [
    "𐬁", "𐬀", "𐬀", "𐬃", "𐬂", "𐬂", "𐬅", "𐬄", "𐬆", "𐬆", "𐬇", "𐬆", "𐬆", "𐬇", "𐬈", "𐬈", "𐬉", "𐬊", "𐬊",
    "𐬋", "𐬌", "𐬍", "𐬎", "𐬏", "𐬐", "𐬒", "𐬓", "𐬑", "𐬗", "𐬘", "𐬚", "𐬙", "𐬜", "𐬛", "𐬝", "𐬞", "𐬟", "𐬠",
    "𐬡", "𐬣", "𐬤", "𐬢", "𐬖", "𐬔", "𐬕", "𐬦", "𐬥", "𐬧", "𐬨", "𐬩", "𐬪", "𐬫", "𐬬", "𐬭", "𐬳", "𐬱", "𐬲",
    "𐬴", "𐬯", "𐬰", "𐬵",
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
