use super::utils::aegean_numbers::UAegean;
use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 66] = [
    "-", "wa", "we", "wi", "wo", "za", "ga", "zo", "ja", "jo", "ka", "ke", "ki", "ko", "ku", "la",
    "le", "li", "lo", "lu", "ma", "me", "mi", "mo", "mu", "na", "ne", "ni", "no", "nu", "ksa",
    "kse", "pa", "pe", "pi", "po", "pu", "ra", "re", "ri", "ro", "ru", "sa", "se", "si", "so",
    "su", "ta", "te", "ti", "to", "tu", "a", "e", "i", "o", "u", "V", "M", "N", "T", "P", "Q", "L",
    "S", "Z",
];

const UNI_VALUES: [&str; 66] = [
    "", "𐠲", "𐠳", "𐠴", "𐠵", "𐠼", "𐠼", "𐠿", "𐠅", "𐠈", "𐠊", "𐠋", "𐠌", "𐠍", "𐠎", "𐠏", "𐠐", "𐠑", "𐠒",
    "𐠓", "𐠔", "𐠕", "𐠖", "𐠗", "𐠘", "𐠙", "𐠚", "𐠛", "𐠜", "𐠝", "𐠷", "𐠸", "𐠞", "𐠟", "𐠠", "𐠡", "𐠢", "𐠣",
    "𐠤", "𐠥", "𐠦", "𐠧", "𐠨", "𐠩", "𐠪", "𐠫", "𐠬", "𐠭", "𐠮", "𐠯", "𐠰", "𐠱", "𐠀", "𐠁", "𐠂", "𐠃", "𐠄",
    "𐄾", "𐄸", "𐄹", "𐄼", "𐄺", "𐄻", "𐄷", "𐄽", "𐄿",
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

fn numbers<T: Into<String>>(input: T) -> String {
    let output = input.into();

    let vecoutput = output
        .split(' ')
        .map(|st: &str| match st.parse::<u32>() {
            Ok(v) => UAegean::new(v).unwrap().into_aegean(),
            Err(_) => st.to_owned(),
        })
        .collect::<Vec<String>>();
    vecoutput.join(" ")
}

pub fn convert<T: Into<String>>(input: T) -> String {
    let mut output = input.into();
    output = ascii_to_unicode(output);
    output = numbers(output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion() {
        let input = "wa 98765";
        let expected = "𐠲 𐄳𐄩𐄟𐄕𐄋";
        let output = convert(input);
        assert_eq!(&output, expected);
    }
}
