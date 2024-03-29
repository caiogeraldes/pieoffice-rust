use lazy_static::lazy_static;
use std::collections::HashMap;

const ENTRY_VALUES: [&str; 725] = [
    "(DEUS)MONS.MENSA",
    "(DEUS)MONS.SARPA",
    "(DEUS)VIA+TERRA",
    "*003",
    "*005",
    "*011",
    "*013",
    "*020",
    "*023",
    "*030",
    "*033",
    "*037",
    "*038",
    "*040",
    "*044",
    "*047",
    "*048",
    "*050",
    "*051",
    "*054",
    "*060",
    "*061",
    "*063",
    "*064",
    "*067",
    "*068",
    "*069",
    "*071",
    "*072",
    "*074",
    "*075",
    "*076",
    "*077",
    "*087",
    "*088",
    "*092",
    "*094",
    "*106",
    "*113",
    "*116",
    "*117",
    "*118",
    "*119",
    "*122",
    "*123",
    "*124",
    "*126",
    "*127",
    "*129",
    "*135A",
    "*136",
    "*139",
    "*140",
    "*141",
    "*142",
    "*143",
    "*144",
    "*145",
    "*146",
    "*147",
    "*149",
    "*150",
    "*152",
    "*154",
    "*155",
    "*156",
    "*157",
    "*158",
    "*159",
    "*161",
    "*162",
    "*163",
    "*164",
    "*167",
    "*168",
    "*169",
    "*170",
    "*171",
    "*180",
    "*183",
    "*184",
    "*185",
    "*186",
    "*187",
    "*188",
    "*189",
    "*194",
    "*195",
    "*198",
    "*203",
    "*205",
    "*206",
    "*208",
    "*211",
    "*213",
    "*217",
    "*218",
    "*219",
    "*220",
    "*222",
    "*224",
    "*227",
    "*230",
    "*232",
    "*233",
    "*234",
    "*235",
    "*236",
    "*238",
    "*240",
    "*242",
    "*253",
    "*256",
    "*258",
    "*259",
    "*260",
    "*261",
    "*262",
    "*263",
    "*265",
    "*270",
    "*271",
    "*273",
    "*274",
    "*275",
    "*279",
    "*282",
    "*284",
    "*285",
    "*287",
    "*293",
    "*295",
    "*297",
    "*302",
    "*308",
    "*310",
    "*311",
    "*312",
    "*317",
    "*320",
    "*321",
    "*323",
    "*324",
    "*329A",
    "*333",
    "*339",
    "*342",
    "*348",
    "*349",
    "*350",
    "*351",
    "*352",
    "*353",
    "*354",
    "*356",
    "*357",
    "*359A",
    "*359",
    "*361",
    "*365",
    "*373",
    "*374",
    "*375",
    "*394",
    "*396",
    "*398",
    "*401",
    "*403",
    "*405",
    "*406",
    "*407",
    "*408",
    "*409",
    "*414",
    "*416",
    "*418",
    "*420",
    "*424",
    "*425",
    "*426",
    "*427",
    "*428",
    "*431",
    "*432",
    "*436",
    "*437",
    "*440",
    "*441",
    "*442",
    "*443",
    "*444",
    "*448",
    "*449",
    "*452",
    "*453",
    "*454",
    "*457A",
    "*457",
    "*458",
    "*459",
    "*460",
    "*462",
    "*463",
    "*464",
    "*465",
    "*466",
    "*467",
    "*468",
    "*469",
    "*471",
    "*472",
    "*473",
    "*475",
    "*476",
    "*477",
    "*478",
    "*479",
    "*480",
    "*481",
    "*482",
    "*483",
    "*484",
    "*485",
    "*486",
    "*487",
    "*489",
    "*490",
    "*491",
    "*492",
    "*493",
    "*494",
    "*495",
    "*496",
    "*497",
    "*501",
    "*502",
    "*503",
    "*504",
    "*505",
    "*507",
    "*509",
    "*510",
    "*511",
    "*512",
    "*513",
    "*514",
    "*515",
    "*516",
    "*517",
    "*518",
    "*519",
    "*520",
    "*521",
    "*522",
    "*523",
    "*526",
    "*530",
    "ADORARE",
    "AEDIFICARE",
    "AEDIFICIUM+MINUS",
    "AEDIFICIUM.PONERE",
    "AEDIFICIUM",
    "ALA",
    "AMPLECTI",
    "ANIMAL",
    "ANNUS+ANNUS",
    "ANNUS",
    "APER",
    "AQUILA",
    "ARGENTUM",
    "ASCIA",
    "ASINUS",
    "ASINUS2",
    "ASINUS2A",
    "AUDIRE+tu+mi",
    "AURIGA",
    "AURIGA2",
    "AURIS+tu+mi",
    "AVIS2",
    "AVIS3",
    "AVIS4",
    "AVIS5",
    "AVIS-x",
    "AVIS",
    "AVUS",
    "BESTIA",
    "BIBERE",
    "BONUS2",
    "BONUS",
    "BOS+MI",
    "BOS.MI",
    "BOS.",
    "BOS",
    "BOS2.MI",
    "BOS2",
    "BRACCHIUM",
    "CAELUM",
    "CANIS2",
    "CANIS",
    "CAPERE+SCALPRUM",
    "CAPERE2.CAPERE2",
    "CAPERE2",
    "CAPERE",
    "CAPRA2A",
    "CAPRA2",
    "CAPRA",
    "CAPUT+SCALPRUM",
    "CAPUT",
    "CASTRUM",
    "CENTUM",
    "CERVUS3",
    "CERVUS2",
    "CERVUS",
    "CONTRACTUS",
    "CORNU+CAPUT",
    "CORNU",
    "CRUS+FLUMEN",
    "CRUS.CRUS",
    "CRUS2",
    "CRUS",
    "CRUX2",
    "CRUX",
    "CUBITUM",
    "CUM",
    "CURRERE",
    "CURRUS",
    "DARE.DARE",
    "DARE",
    "DECEM",
    "DELERE",
    "DEUS.DOMUS",
    "DEUS",
    "DIES",
    "MAGNUS.DOMINA",
    "MAGNUS.DOMUS",
    "MAGNUS.FILIA",
    "MAGNUS.REX",
    "REX.FILIA",
    "REX.FILIUS",
    "REX.INFANS.FILIUS",
    "MAGNUS",
    "DOMINA",
    "DOMINUS",
    "FEMINA",
    "FILIA",
    "FILIUS",
    "DOMUS+MINUS",
    "DOMUS+SCALA",
    "DOMUS+x",
    "DOMUS",
    "EDERE",
    "EGO2",
    "EGO",
    "ENSIS",
    "EQUUS",
    "EUNUCHUS2",
    "EUNUCHUS",
    "EXERCITUS",
    "FINES+ha",
    "FINES",
    "FLUMEN",
    "FONS",
    "FORTIS",
    "FRATER2",
    "FRATER",
    "FRONS",
    "FULGUR",
    "FUSUS",
    "GENUFLECTERE",
    "GRYLLUS",
    "HASTARIUS",
    "HATTI+LI",
    "HATTUSILI",
    "HATTI",
    "HEROS",
    "HORDEUM",
    "HORREUM",
    "IACULUM",
    "INFANS",
    "INFRA",
    "ISHUWA",
    "IUDEX+la",
    "IUDEX+ra",
    "IUDEX+ri",
    "IUDEX+tara",
    "IUDEX+tari",
    "IUDEX.la",
    "IUDEX",
    "IUSTITIA",
    "JANUS",
    "LAPIS+SCALPRUM",
    "LAPIS",
    "LECTUS",
    "LEO+MONS.tu+LEO",
    "LEO",
    "LEO2",
    "LEPUS2",
    "LEPUS",
    "LIBARE",
    "LIBATIO",
    "LIGARE",
    "LINGERE",
    "LINGUA+CLAVUS",
    "LINGUA-x",
    "LINGUA",
    "LIS",
    "LITUS+na",
    "LITUUS+U",
    "LITUUS",
    "LOCUS",
    "LONGUS",
    "LOQUI",
    "LUNA",
    "MALLEUS",
    "MALUS2",
    "MALUS",
    "MANDARE2",
    "MANDARE",
    "MANUS+CULTER",
    "MANUS+MINUS",
    "MANUS.CULTER",
    "CULTER",
    "MANUS",
    "MATER",
    "MENSA2",
    "MILLE",
    "MINUS",
    "MORI",
    "MONS2",
    "MONS",
    "MURSILI",
    "NEG2",
    "NEG3",
    "NEG",
    "NEPOS",
    "OCCIDENS",
    "OCULUS",
    "OMNIS2",
    "OMNIS(+mi)",
    "OMNIS",
    "ORIENS",
    "OVIS2",
    "OVIS3",
    "OVIS",
    "PANIS.SCUTELLA",
    "PANIS",
    "PASTOR",
    "PES.REGIO",
    "mí.REGIO",
    "PES.SCALA.ROTAE",
    "PES2.PES2",
    "PES2.PES",
    "PES2",
    "PES",
    "PISCIS",
    "PITHOS.SCUTELLA",
    "PITHOS..",
    "PITHOS.",
    "PITHOS",
    "POCULUM",
    "PODIUM",
    "PONERE",
    "PORTA2",
    "PORTA",
    "POST",
    "PRAE",
    "PRINCEPS",
    "PROPHETA",
    "PUGNUS+PUGNUS",
    "PUGNUS+x",
    "PUGNUS",
    "PURUS",
    "REGIO",
    "REL",
    "REX",
    "ROTA",
    "SACERDOS2",
    "SACERDOS",
    "SARA",
    "SARI",
    "SARMA2",
    "SARMA",
    "SARPA",
    "SCRIBA",
    "SCUTELLA",
    "SCUTUM",
    "SERVUS",
    "SIGILLUM",
    "SOL",
    "SOL2.THRONUS/MENSA",
    "MENSA",
    "SOL2",
    "SOLIUM",
    "SPHINX",
    "STATUA",
    "STELE",
    "SUB",
    "SUPER",
    "TELIPINU",
    "TESHUB",
    "THRONUS..",
    "THRONUS.",
    "THRONUS2",
    "THRONUS",
    "TONITRUS",
    "UNGULA",
    "UNUS",
    "URBS+li",
    "URBS-li",
    "URBS",
    "URCEUS",
    "VACUUS",
    "VAS",
    "VERSUS",
    "VIR2.MINUS",
    "VIR2A",
    "VIR2",
    "VIR",
    "VIA+TERRA+SCALPRUM",
    "VIA+TERRA.SCALPRUM",
    "VIA",
    "TERRA",
    "SCALPRUM",
    "VITA",
    "VITELLUS",
    "VITIS",
    "WD",
    "WE",
    "zuwa",
    "ha-x",
    "hala",
    "hali",
    "hana",
    "hara",
    "hari",
    "huru",
    "hu",
    "hwi-x",
    "hwa.",
    "hwi.",
    "há-li",
    "há",
    "hí",
    "hú",
    "kar",
    "ka",
    "ki-x",
    "ki",
    "ku",
    "kwa",
    "kwi",
    "ká.",
    "ká",
    "la..",
    "la.",
    "la+la",
    "la+ra+a",
    "la-x",
    "li..",
    "li.",
    "li-x",
    "lignum",
    "lu",
    "lá",
    "lì",
    "lí.",
    "lí",
    "ma..",
    "ma.",
    "ma",
    "ma-x.",
    "ma-x",
    "mi",
    "muwa...",
    "muwa..",
    "muwa.",
    "muwa",
    "mu....",
    "mu...",
    "mu..",
    "mu.",
    "mu",
    "mà",
    "má",
    "mì",
    "mí",
    "ni-x",
    "nu",
    "nà",
    "ná",
    "nì",
    "ní",
    "nú",
    "pa-x",
    "pari",
    "pi",
    "pi.",
    "pu",
    "pú",
    "ru",
    "rú..",
    "rú.",
    "rú",
    "sa-x",
    "sa4",
    "sa5",
    "sa6",
    "sa7",
    "sa8",
    "sara",
    "sari",
    "sa",
    "si",
    "su",
    "sà...",
    "sà..",
    "sà.",
    "sà",
    "sá",
    "sí-x",
    "sú",
    "ta-x",
    "ta.",
    "ta4",
    "ta5",
    "ta6",
    "tala",
    "tana",
    "tapa.",
    "tapa",
    "tara.",
    "tara",
    "tari.",
    "tari",
    "ta",
    "ti4",
    "ti5",
    "ti",
    "tu4",
    "tuzzi",
    "tu",
    "tà.",
    "tà",
    "tá",
    "tì",
    "tí",
    "tù",
    "tú",
    "u...",
    "u..",
    "u.",
    "urhi",
    "ur",
    "us",
    "u",
    "wa5",
    "wa5.",
    "wa6",
    "wa7",
    "wa9",
    "wi(ya)",
    "wi.",
    "wi5",
    "wi5.",
    "wi6",
    "wi7",
    "wi9",
    "wi",
    "wà",
    "wá",
    "wì",
    "wí",
    "za-x",
    "za.",
    "za4",
    "za",
    "zi4",
    "zi",
    "zà",
    "zá",
    "zì.",
    "zì",
    "zí",
    "wa",
    "hi",
    "li",
    "pa",
    "la",
    "arha.",
    "arha",
    "ha",
    "a+ra",
    "a+ri",
    "a+tá",
    "a-x",
    "ara",
    "ara.",
    "ari",
    "ari.",
    "i(a)",
    "i+ra",
    "i+ri",
    "ia",
    "ra",
    "ri",
    "na",
    "ni",
    "ià",
    "iá",
    "a",
    "i",
    "á",
    "í",
    "12",
    "1",
    "2.",
    "2",
    "3",
    "4",
    "5",
    "8",
    "9",
    ".",
    "<",
    ">",
];

const UNI_VALUES: [&str; 725] = [
    "𔕍", "𔕍", "𔓧", "𔐂", "𔐄", "𔐋", "𔐍", "𔐔", "𔐗", "𔐟", "𔐢", "𔐦", "𔐧", "𔐪", "𔐯", "𔐵", "𔐶", "𔐸", "𔐹",
    "𔐼", "𔑂", "𔑃", "𔑅", "𔑆", "𔑌", "𔑍", "𔑎", "𔑐", "𔑑", "𔑓", "𔑔", "𔑕", "𔑖", "𔑠", "𔑡", "𔑥", "𔑧", "𔑽",
    "𔒉", "𔒍", "𔒎", "𔒏", "𔒐", "𔒓", "𔒔", "𔒕", "𔒘", "𔒙", "𔒛", "𔒢", "𔒣", "𔒦", "𔒧", "𔒨", "𔒩", "𔒪", "𔒫",
    "𔒬", "𔒭", "𔒮", "𔒰", "𔒱", "𔒳", "𔒵", "𔒶", "𔒷", "𔒸", "𔒹", "𔒺", "𔒼", "𔒽", "𔒾", "𔒿", "𔓂", "𔓃", "𔓄",
    "𔓅", "𔓆", "𔓏", "𔓒", "𔓓", "𔓔", "𔓕", "𔓖", "𔓗", "𔓘", "𔓝", "𔓞", "𔓡", "𔓨", "𔓪", "𔓫", "𔓮", "𔓲", "𔓴",
    "𔓺", "𔓻", "𔓼", "𔓽", "𔓿", "𔔁", "𔔄", "𔔈", "𔔊", "𔔋", "𔔌", "𔔍", "𔔎", "𔔐", "𔔒", "𔔔", "𔔟", "𔔢", "𔔤",
    "𔔥", "𔔦", "𔔧", "𔔨", "𔔩", "𔔫", "𔔱", "𔔲", "𔔴", "𔔵", "𔔶", "𔔺", "𔔽", "𔔿", "𔕀", "𔕂", "𔕉", "𔕌", "𔕎",
    "𔕔", "𔕚", "𔕝", "𔕞", "𔕟", "𔕤", "𔕧", "𔕨", "𔕪", "𔕫", "𔕱", "𔕷", "𔖀", "𔖃", "𔖉", "𔖊", "𔖋", "𔖌", "𔖍",
    "𔖎", "𔖏", "𔖑", "𔖒", "𔖕", "𔖔", "𔖗", "𔖜", "𔖦", "𔖧", "𔖨", "𔖾", "𔗀", "𔗂", "𔗅", "𔗇", "𔗉", "𔗊", "𔗋",
    "𔗌", "𔗍", "𔗓", "𔗕", "𔗗", "𔗙", "𔗝", "𔗞", "𔗟", "𔗠", "𔗡", "𔗤", "𔗥", "𔗩", "𔗪", "𔗭", "𔗮", "𔗯", "𔗰",
    "𔗱", "𔗵", "𔗶", "𔗺", "𔗻", "𔗼", "𔘀", "𔗿", "𔘁", "𔘂", "𔘃", "𔘅", "𔘆", "𔘇", "𔘈", "𔘉", "𔘊", "𔘋", "𔘌",
    "𔘎", "𔘏", "𔘐", "𔘒", "𔘓", "𔘔", "𔘕", "𔘖", "𔘗", "𔘘", "𔘙", "𔘚", "𔘛", "𔘜", "𔘝", "𔘞", "𔘠", "𔘡", "𔘢",
    "𔘣", "𔘤", "𔘥", "𔘦", "𔘧", "𔘨", "𔘩", "𔘪", "𔘫", "𔘬", "𔘭", "𔘯", "𔘱", "𔘲", "𔘳", "𔘴", "𔘵", "𔘶", "𔘷",
    "𔘸", "𔘹", "𔘺", "𔘻", "𔘼", "𔘽", "𔘾", "𔘿", "𔙂", "𔙆", "𔐅", "𔔘", "𔔗", "𔔘", "𔔖", "𔑗", "𔐈", "𔗈", "𔖁",
    "𔕺", "𔙃", "𔒟", "𔔣", "𔔼", "𔑯", "𔑱", "𔑲", "𔑒", "𔕄", "𔕅", "𔑒", "𔒞", "𔒜", "𔒟", "𔒝", "𔒡", "𔒚", "𔕳",
    "𔑪", "𔐇", "𔖢", "𔓀", "𔑾", "𔒀", "𔑻", "𔑺", "𔒁", "𔑼", "𔐡", "𔓑", "𔑭", "𔑬", "𔕲", "𔐭", "𔐮", "𔐫", "𔑹",
    "𔑸", "𔑶", "𔐊", "𔐉", "𔔉", "𔗃", "𔑵", "𔑴", "𔑳", "𔖅", "𔙀", "𔒂", "𔑜", "𔑟", "𔑝", "𔑛", "𔕜", "𔕛", "𔔕",
    "𔑀", "𔘰", "𔕃", "𔑊", "𔑈", "𔗁", "𔔚", "𔔛", "𔖖", "𔖓", "𔐐", "𔔜", "𔐴", "𔐒", "𔐳", "𔐲", "𔐲", "𔖙", "𔐏",
    "𔖺", "𔑘", "𔐱", "𔐰", "𔔚", "𔔞", "𔔝", "𔔙", "𔐆", "𔐁", "𔐀", "𔐻", "𔑮", "𔔠", "𔘑", "𔔰", "𔓹", "𔓸", "𔓳",
    "𔓶", "𔐝", "𔔷", "𔐰", "𔐚", "𔓣", "𔕗", "𔑞", "𔒑", "𔓈", "𔓠", "𔓠", "𔓟", "𔐕", "𔓎", "𔔡", "𔕀", "𔐰", "𔐿",
    "𔔃", "𔔸", "𔖤", "𔖤", "𔖤", "𔖤", "𔔸", "𔖣", "𔖣", "𔒯", "𔔭", "𔔮", "𔕓", "𔓭", "𔑪", "𔑫", "𔒌", "𔒋", "𔐜",
    "𔒤", "𔐠", "𔒈", "𔓌", "𔙅", "𔓊", "𔐘", "𔐥", "𔒊", "𔖫", "𔓤", "𔑄", "𔐖", "𔓜", "𔔻", "𔖠", "𔖟", "𔑋", "𔑊",
    "𔐻", "𔑄", "𔐺", "𔕿", "𔑁", "𔑘", "𔕋", "𔗄", "𔖮", "𔖯", "𔐃", "𔓬", "𔔅", "𔕵", "𔕶", "𔕴", "𔕒", "𔖬", "𔐙",
    "𔗣", "𔖝", "𔖝", "𔓛", "𔒆", "𔒇", "𔒄", "𔗛", "𔓐", "𔗫", "𔔬", "𔔇", "𔑤", "𔑨", "𔑩", "𔑦", "𔑣", "𔒥", "𔕺",
    "𔖄", "𔕾", "𔕺", "𔖇", "𔔪", "𔑇", "𔔑", "𔔏", "𔐣", "𔐎", "𔙁", "𔙀", "𔐠", "𔐩", "𔐨", "𔕩", "𔔆", "𔕰", "𔐑",
    "𔕈", "𔖥", "𔖐", "𔕕", "𔕕", "𔑚", "𔑙", "𔕋", "𔕭", "𔗆", "𔔳", "𔖷", "𔕮", "𔓚", "𔕌", "𔕊", "𔓙", "𔕐", "𔒒",
    "𔐌", "𔔭", "𔐿", "𔑏", "𔒲", "𔕥", "𔕍", "𔕋", "𔕏", "𔕊", "𔓢", "𔒗", "𔖭", "𔔅", "𔔅", "𔔂", "𔖆", "𔔗", "𔖂",
    "𔐛", "𔖯", "𔖶", "𔖵", "𔕠", "𔓦", "𔓥", "𔓾", "𔓤", "𔔯", "𔖡", "𔒃", "𔒻", "𔖵", "𔗷", "𔕀", "𔕡", "𔕈", "𔕈",
    "𔘮", "𔕆", "𔕆", "𔗹", "𔕙", "𔓎", "𔘰", "𔘰", "𔓠", "𔓟", "𔕘", "𔖈", "𔕢", "𔗧", "𔔓", "𔗳", "𔗜", "𔕰", "𔕰",
    "𔐿", "𔐾", "𔗲", "𔕦", "𔓋", "𔓍", "𔗽", "𔗲", "𔕦", "𔒗", "𔖰", "𔗲", "𔓇", "𔕇", "𔒖", "𔓇", "𔒆", "𔒅", "𔒄",
    "𔘄", "𔒃", "𔖻", "𔒁", "𔒀", "𔑿", "𔑾", "𔖛", "𔒁", "𔒀", "𔑿", "𔑾", "𔕖", "𔖘", "𔖷", "𔗘", "𔗴", "𔒴", "𔑝",
    "𔕵", "𔐽", "𔓵", "𔖿", "𔓐", "𔐎", "𔑈", "𔑉", "𔕯", "𔗣", "𔗑", "𔑵", "𔑴", "𔑳", "𔗖", "𔗆", "𔕮", "𔔀", "𔕣",
    "𔖭", "𔑏", "𔑏", "𔗔", "𔓉", "𔖢", "𔑹", "𔑸", "𔑷", "𔑶", "𔗦", "𔗾", "𔒂", "𔐭", "𔑰", "𔕦", "𔓇", "𔑛", "𔖞",
    "𔗢", "𔒌", "𔒋", "𔖸", "𔖹", "𔖸", "𔖹", "𔑯", "𔕦", "𔓇", "𔑣", "𔔆", "𔔾", "𔑢", "𔐬", "𔐫", "𔐞", "𔙄", "𔘟",
    "𔕭", "𔕬", "𔖚", "𔑼", "𔑻", "𔗘", "𔖙", "𔗚", "𔑺", "𔓩", "𔓬", "𔓤", "𔕁", "𔔻", "𔒻", "𔒻", "𔓩", "𔓬", "𔓤",
    "𔕁", "𔔻", "𔗬", "𔓀", "𔓁", "𔓀", "𔓁", "𔕽", "𔖩", "𔒈", "𔖪", "𔒚", "𔖩", "𔕼", "𔕹", "𔕻", "𔕺", "𔕠", "𔗬",
    "𔗒", "𔔹", "𔕸", "𔓊", "𔓹", "𔓸", "𔓷", "𔗸", "𔗸", "𔐷", "𔗨", "𔒟", "𔒠", "𔒟", "𔒠", "𔓯", "𔓰", "𔓰", "𔓱",
    "𔖱", "𔖱", "𔐤", "𔗐", "𔖬", "𔕑", "𔗷", "𔓯", "𔐓", "𔕐", "𔘍", "𔖭", "𔖴", "𔖳", "𔖸", "𔖻", "𔖼", "𔖽", "𔖿",
    "𔖲", "𔗎", "𔗏",
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
