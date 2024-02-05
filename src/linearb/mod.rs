use aegean_numerals_rs::UAegean;
use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 222] = [
    "-", "*132", "*142", "*146", "*150", "*152", "*153", "*154", "*155", "*157", "*158", "*160",
    "*161", "*164", "*165", "*166", "*167", "*168", "*169", "*170", "*171", "*172", "*174", "*177",
    "*178", "*179", "*180", "*18", "*181", "*182", "*183", "*184", "*185", "*189", "*190", "*19",
    "*200", "*201", "*202", "*203", "*204", "*205", "*206", "*207", "*208", "*209", "*210", "*211",
    "*212", "*213", "*214", "*215", "*216", "*217", "*218", "*219", "*220", "*221", "*222", "*226",
    "*227", "*228", "*229", "*22", "*232", "*234", "*236", "*245", "*246", "*248", "*249", "*250",
    "*251", "*252", "*253", "*255", "*256", "*257", "*258", "*259", "*305", "*34", "*47", "*49",
    "*56", "*63", "*64", "*65", "*79", "*82", "*83", "*86", "*89", "AES", "ALVEUS", "ARBOR",
    "AREPA", "ARMA", "AROM", "AUR", "BIGAE", "BOSf", "BOSm", "CAPSUS", "CAPf", "CAPm", "CERV",
    "CORNU", "CURRUS", "CYP", "DIPTE", "EQUf", "EQUm", "EQU", "GALEA", "GRA", "HASTA", "HORD",
    "JACULUM", "KANAKO", "KAPO", "LANA", "LUNA", "MERI", "MUL", "OLE", "OLIV", "OVISf", "OVISm",
    "PUGIO", "ROTA", "SAGITTA", "SUSf", "SUSm", "TELA", "TUNICA", "TURO2", "VIN", "VIR", "da",
    "de", "di", "do", "du", "dwe", "dwo", "je", "jo", "ju2", "ju", "ka", "ke", "ki", "ko", "ku",
    "ma", "me", "mi", "mo", "mu", "na", "ne", "ni", "no", "nu", "nwa", "ja", "pa", "pe", "pi",
    "po", "pte", "pu2", "pu", "qa", "qe", "qi", "qo", "ra2", "ra3", "ra", "re", "ri", "ro2", "ro",
    "ru", "sa", "se", "si", "so", "su", "ta2", "ta", "te", "ti", "to", "tu", "two", "u", "wa",
    "we", "wi", "wo", "za", "ze", "zo", "e", "a2", "a3", "a", "i", "o", "g,", "V", "M", "N", "T",
    "P", "Q", "L", "S", "Z",
];

const UNI_VALUES: [&str; 222] = [
    "", "𐂗", "𐂜", "𐂞", "𐂟", "𐂡", "𐂢", "𐂣", "𐃞", "𐂥", "𐂦", "𐂨", "𐂩", "𐂬", "𐂭", "𐂮", "𐂯", "𐂰", "𐂱",
    "𐂲", "𐂳", "𐂴", "𐂶", "𐂸", "𐂹", "𐂺", "𐂻", "𐁐", "𐂼", "𐂽", "𐂾", "𐂿", "𐃀", "𐃁", "𐃂", "𐁑", "𐃟", "𐃠",
    "𐃡", "𐃢", "𐃣", "𐃤", "𐃥", "𐃦", "𐃧", "𐃨", "𐃩", "𐃪", "𐃫", "𐃬", "𐃭", "𐃮", "𐃯", "𐃰", "𐃱", "𐃲", "𐃄",
    "𐃳", "𐃴", "𐃵", "𐃶", "𐃷", "𐃸", "𐁒", "𐃈", "𐃊", "𐃋", "𐃐", "𐃑", "𐃓", "𐃔", "𐃹", "𐃕", "𐃖", "𐃗", "𐃙",
    "𐃚", "𐃛", "𐃜", "𐃝", "𐃺", "𐁓", "𐁔", "𐁕", "𐁖", "𐁗", "𐁘", "𐀎", "𐁙", "𐁚", "𐁛", "𐁜", "𐁝", "𐂚", "𐃅",
    "𐂷", "𐂘", "𐂫", "𐂑", "𐂛", "𐃌", "𐂌", "𐂍", "𐃎", "𐂈", "𐂉", "𐂂", "𐂠", "𐃍", "𐂒", "𐃒", "𐂄", "𐂅", "𐂃",
    "𐃃", "𐂎", "𐃆", "𐂏", "𐃘", "𐂔", "𐂓", "𐂝", "𐂵", "𐂙", "𐂁", "𐂕", "𐂐", "𐂆", "𐂇", "𐃉", "𐃏", "𐃇", "𐂊",
    "𐂋", "𐂧", "𐂪", "𐂤", "𐂖", "𐂀", "𐀅", "𐀆", "𐀇", "𐀈", "𐀉", "𐁃", "𐁄", "𐀋", "𐀍", "𐀎", "𐀎", "𐀏", "𐀐",
    "𐀑", "𐀒", "𐀓", "𐀔", "𐀕", "𐀖", "𐀗", "𐀘", "𐀙", "𐀚", "𐀛", "𐀜", "𐀝", "𐁅", "𐀊", "𐀞", "𐀟", "𐀠", "𐀡",
    "𐁇", "𐁆", "𐀢", "𐀣", "𐀤", "𐀥", "𐀦", "𐁈", "𐁉", "𐀨", "𐀩", "𐀪", "𐁊", "𐀫", "𐀬", "𐀭", "𐀮", "𐀯", "𐀰",
    "𐀱", "𐁌", "𐀲", "𐀳", "𐀴", "𐀵", "𐀶", "𐁍", "𐀄", "𐀷", "𐀸", "𐀹", "𐀺", "𐀼", "𐀽", "𐀿", "𐀁", "𐁀", "𐁁",
    "𐀀", "𐀂", "𐀃", "𐄀", "𐄾", "𐄸", "𐄹", "𐄼", "𐄺", "𐄻", "𐄷", "𐄽", "𐄿",
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
