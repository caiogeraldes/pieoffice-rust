use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 42] = [
    "A", "aa", "Á", "áá", "á", "Ã", "ãã", "ã", "ææ", "æ", "Æ", "êê", "ê", "Ê", "ee", "E", "oo",
    "O", "I", "U", "X", "xw", "c", "j", "th", "dh", "T", "B", "ngH", "ngW", "ng", "gh", "G", "ñ",
    "N", "m", "M", "Y", "shy", "sh", "zh", "S",
];

const UNI_VALUES: [&str; 42] = [
    "ā", "ā", "ā̊", "ā̊", "å", "ą̇", "ą̇", "ą", "ə̄", "ə", "ə̄", "ə̄", "ə", "ə̄", "ē", "ē", "ō", "ō", "ī",
    "ū", "x́", "xᵛ", "č", "ǰ", "ϑ", "δ", "t̰", "β", "ŋ́", "ŋᵛ", "ŋ", "γ", "ġ", "ń", "ṇ", "m", "m̨",
    "ẏ", "š́", "š", "ž", "ṣ̌",
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
