use lazy_static::lazy_static;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

const ENTRY_VALUES: [&str; 91] = [
    "ee", "EE", "ew", "e'", "E'", "e", "E", "zh", "ch.", "c'h", "C'h", "ch", "sh", "jh", "ZH",
    "CH.", "SH", "CH", "JH", "t'", "p'", "k'", "o'", "c'", "P'", "C'", "K'", "O'", "T'", "g.",
    "l.", "r.", "V.", "R.", "G.", "L.", "I", "L", "X", "C", "K", "J", "M", "Y", "N", "O", "P", "S",
    "T", "R", "W", "F", "U", "a", "b", "g", "d", "z", "i", "l", "x", "c", "k", "j", "m", "y", "n",
    "o", "p", "s", "v", "t", "r", "w", "f", "u", "A", "B", "G", "D", "Z", "h", "H", "?", ".'", ".",
    ";", ";'", "!", "``", "''",
];

const UNI_VALUES: [&str; 91] = [
    "է", "Է", "և", "ը", "Ը", "ե", "Ե", "ժ", "ճ", "ճ", "ճ", "չ", "շ", "ջ", "Ժ", "Ճ", "Շ", "Չ", "Ջ",
    "թ", "փ", "ք", "օ", "ց", "Փ", "Ց", "Ք", "Օ", "Թ", "ղ", "ղ", "ռ", "Վ", "Ռ", "Ղ", "Ղ", "Ի", "Լ",
    "Խ", "Ծ", "Կ", "Ձ", "Մ", "Յ", "Ն", "Ո", "Պ", "Ս", "Տ", "Ր", "Ւ", "Ֆ", "Սւ", "ա", "բ", "գ", "դ",
    "զ", "ի", "լ", "խ", "ծ", "կ", "ձ", "մ", "յ", "ն", "ո", "պ", "ս", "վ", "տ", "ր", "ւ", "ֆ", "ու",
    "Ա", "Բ", "Գ", "Դ", "Զ", "հ", "Հ", "՞", "։", "՝", "՟", "՛", "՜", "«", "»",
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
