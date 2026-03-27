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
    pub fn value(&self) -> &'static [u8] {
        match *self {
            DeriveTraits::CLONE => b"Clone",
            DeriveTraits::COPY => b"Copy",
            DeriveTraits::DEBUG => b"Debug",
            DeriveTraits::DEFAULT => b"Default",
            DeriveTraits::DESERIALIZE => b"Deserialize",
            DeriveTraits::EQ => b"Eq",
            DeriveTraits::HASH => b"Hash",
            DeriveTraits::ITERATOR => b"Iterator",
            DeriveTraits::ORD => b"Ord",
            DeriveTraits::PARTIALEQ => b"PartialEq",
            DeriveTraits::PARTIALORD => b"PartialOrd",
            DeriveTraits::SERIALIZE => b"Serialize",
        }
    }
}

pub fn gen_derive_str(traits: &[DeriveTraits]) -> Vec<u8> {
    let mut out_buffer = traits.iter().enumerate().fold(DERIVE_START.to_vec(), |mut acc, (idx, d)| {
        acc.extend_from_slice(d.value());

        if idx < traits.len() - 1 {
            acc.extend_from_slice(COMMA);
        }

        acc
    });

    out_buffer.extend_from_slice(CLOSE_PAREN);
    out_buffer.extend_from_slice(CLOSE_SQR);
    out_buffer.extend_from_slice(LINE_FEED);
    out_buffer
}
