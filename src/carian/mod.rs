use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 51] = [
    "a", "e2", "r2", "z2", "b", "d", "l", "y3", "y2", "ý", "'y", "y", "r", "L2", "L", "A2", "q",
    "b", "m", "o", "D2", "t", "sh2", "sh", "'s", "ś", "s", "18", "u", "N", "c", "n", "T2", "p",
    "i", "e", "k2", "k", "dh", "w", "G2", "G", "z", "ng", "j", "39", "T", "mb2", "mb3", "mb4",
    "mb",
];

const UNI_VALUES: [&str; 51] = [
    "𐊠", "𐋏", "𐋉", "𐋂", "𐊡", "𐊢", "𐊣", "𐋈", "𐋐", "𐊻", "𐊻", "𐊤", "𐊥", "𐋎", "𐊦", "𐊧", "𐊨", "𐊩", "𐊪",
    "𐊫", "𐊬", "𐊭", "𐊯", "𐊮", "𐊸", "𐊸", "𐊰", "𐊱", "𐊲", "𐊳", "𐊴", "𐊵", "𐊶", "𐊷", "𐊹", "𐊺", "𐊽", "𐊼",
    "𐊾", "𐊿", "𐋁", "𐋀", "𐋃", "𐋄", "𐋅", "𐋆", "𐋇", "𐋋", "𐋌", "𐋍", "𐋊",
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
