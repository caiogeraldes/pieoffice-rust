use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 50] = [
    "-",
    "ahuramazda1",
    "ahuramazda2",
    "ahuramazda3",
    "xshayathia",
    "dahyaus1",
    "dahyaus2",
    "baga",
    "bumis",
    "mi",
    "mu",
    "ku",
    "xi",
    "gu",
    "xu",
    "ji",
    "ti",
    "tu",
    "th",
    "di",
    "du",
    "ni",
    "nu",
    "vi",
    "ri",
    "sh",
    "a",
    "i",
    "u",
    "k",
    "x",
    "g",
    "c",
    "ç",
    "j",
    "t",
    "d",
    "p",
    "f",
    "b",
    "n",
    "m",
    "y",
    "v",
    "r",
    "l",
    "s",
    "z",
    "š",
    "h",
];

const UNI_VALUES: [&str; 50] = [
    "", "𐏈", "𐏉", "𐏊", "𐏋", "𐏌", "𐏌", "𐏎", "𐏏", "𐎷", "𐎸", "𐎤", "𐎧", "𐎦", "𐎧", "𐎪", "𐎫", "𐎬", "𐎰",
    "𐎮", "𐎯", "𐎴", "𐎵", "𐎻", "𐎽", "𐏁", "𐎠", "𐎡", "𐎢", "𐎣", "𐎧", "𐎥", "𐎨", "𐏂", "𐎩", "𐎫", "𐎭", "𐎱",
    "𐎳", "𐎲", "𐎴", "𐎶", "𐎹", "𐎺", "𐎼", "𐎾", "𐎿", "𐏀", "𐏁", "𐏃",
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
