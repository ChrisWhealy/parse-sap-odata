use super::{CLOSE_PAREN, CLOSE_SQR, COMMA, DERIVE_START, LINE_FEED};

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
    /// Transform Enum variant name to the trait name added to the source code
    pub fn value(&self) -> Vec<u8> {
        let trait_name = match *self {
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
        };

        trait_name.as_bytes().to_vec()
    }
}

pub fn derive_str(traits: Vec<DeriveTraits>) -> Vec<u8> {
    let mut out_buffer = DERIVE_START.to_vec();

    for (idx, d) in traits.iter().enumerate() {
        out_buffer.append(&mut d.value());

        if idx < traits.len() - 1 {
            out_buffer.append(&mut COMMA.to_vec());
        }
    }

    out_buffer.append(&mut [CLOSE_PAREN, CLOSE_SQR, LINE_FEED].concat());
    out_buffer
}
