/// # Oscan transliteration scheme
///
/// |      |      |        |      |      |      |      |
/// |------|------|--------|------|------|------|------|
/// | a 𐌀 | b 𐌁 | g,k 𐌂 | d 𐌃 | e 𐌄 | v 𐌅 | z 𐌆 |
/// | h 𐌇 | i 𐌉 | l   𐌋 | m 𐌌 | n 𐌍 | p 𐌐 | ś; s/ 𐌑 |
/// | r 𐌓 | s 𐌔 | t   𐌕 | u 𐌖 | f 𐌚 | ú; u/ 𐌞 | í; i/ 𐌝 |
pub mod oscan;

/// # Ogham transliteration scheme
///
/// |               |               |               |               |
/// |---------------|---------------|---------------|---------------|
/// | b           ᚁ | l           ᚂ | w           ᚃ | s           ᚄ |
/// | n           ᚅ | j           ᚆ | h           ᚆ | d           ᚇ |
/// | t           ᚈ | k           ᚉ | kw          ᚊ | c           ᚉ |
/// | cw          ᚊ | m           ᚋ | g           ᚌ | gw          ᚍ |
/// | S           ᚎ | r           ᚏ | a           ᚐ | o           ᚑ |
/// | u           ᚒ | e           ᚓ | i           ᚔ | ,ear,       ᚕ |
/// | ,or,        ᚖ | ,uilleann,  ᚗ | ,ifin,      ᚘ | ,eam,       ᚙ |
/// | ,peith,     ᚚ | >           ᚛ | <           ᚜ |               |
pub mod ogham;

pub mod armenian;

pub mod carian;

pub mod pie;

pub mod oldpersian;

pub mod lydian;

pub mod lycian;

pub mod gothic;

pub mod glagolitic;

pub mod cypriot;

pub mod linearb;

pub mod avestan;

pub mod luwian;
