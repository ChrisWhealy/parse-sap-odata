use super::{COMMA, SPACE};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Derivable traits.
///
/// The Serde traits `Serialize` and `Deserialize` are also included
pub enum DeriveTraits {
    CLONE,
    COPY,
    DEBUG,
    DEFAULT,
    DESERIALIZE,
    EQ,
    HASH,
    ITERATOR,
    ORD,
    PARTIALEQ,
    PARTIALORD,
    SERIALIZE,
}

impl DeriveTraits {
    /// Helper function to return the enum variant name as a static string slice
    pub const fn value(&self) -> &'static str {
        match *self {
            DeriveTraits::CLONE => "Clone",
            DeriveTraits::COPY => "Copy",
            DeriveTraits::DEBUG => "Debug",
            DeriveTraits::DEFAULT => "Default",
            DeriveTraits::DESERIALIZE => "Deserialize",
            DeriveTraits::EQ => "Eq",
            DeriveTraits::HASH => "Hash",
            DeriveTraits::ITERATOR => "Iterator",
            DeriveTraits::ORD => "Ord",
            DeriveTraits::PARTIALEQ => "PartialEq",
            DeriveTraits::PARTIALORD => "PartialOrd",
            DeriveTraits::SERIALIZE => "Serialize",
        }
    }
}

pub fn derive_str(traits: Vec<DeriveTraits>) -> Vec<u8> {
    let mut d_str = Vec::new();
    d_str.extend_from_slice(DERIVE_START);

    for (idx, d) in traits.iter().enumerate() {
        d_str.extend_from_slice(d.value().as_bytes());

        if idx < traits.len() - 1 {
            d_str.extend([COMMA, SPACE].concat());
        }
    }

    d_str.extend(DERIVE_END);
    d_str
}

pub static DERIVE_START: &[u8] = "#[derive(".as_bytes();
pub static DERIVE_END: &[u8] = &[0x29, 0x5D, 0x0A];
