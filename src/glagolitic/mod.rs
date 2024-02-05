use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 47] = [
    "yõ", "yo~", "ya", "yo", "yu", "yẽ", "ye~", "ẽ", "e~", "õ", "o~", "a", "b", "v", "g", "d", "e",
    "zh", "dz", "z", "ii", "iy", "i", "j", "k", "l", "m", "n", "p", "r", "s", "t", "u", "f", "x",
    "oo", "o", "w", "sht", "ts", "ch", "sh", "''", "'i", "'", "th", "v",
];

const UNI_VALUES: [&str; 47] = [
    "Ⱙ", "Ⱙ", "Ⱑ", "Ⱖ", "Ⱓ", "Ⱗ", "Ⱗ", "Ⱔ", "Ⱔ", "Ⱘ", "Ⱘ", "Ⰰ", "Ⰱ", "Ⰲ", "Ⰳ", "Ⰴ", "Ⰵ", "Ⰶ", "Ⰷ",
    "Ⰸ", "Ⰹ", "Ⰺ", "Ⰻ", "Ⰼ", "Ⰽ", "Ⰾ", "Ⰿ", "Ⱀ", "ⱂ", "Ⱃ", "Ⱄ", "Ⱅ", "Ⱆ", "Ⱇ", "Ⱈ", "Ⱉ", "Ⱁ", "Ⱉ",
    "Ⱋ", "Ⱌ", "Ⱍ", "Ⱎ", "Ⱏ", "Ⰺ", "Ⱐ", "Ⱚ", "Ⱛ",
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
