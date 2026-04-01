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
    pub fn value(&self) -> &'static str {
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

pub fn gen_derive_str(traits: &[DeriveTraits]) -> String {
    let mut out = String::new();
    out.push_str(DERIVE_START);

    for (idx, d) in traits.iter().enumerate() {
        out.push_str(d.value());

        if idx < traits.len() - 1 {
            out.push_str(COMMA);
        }
    }

    out.push_str(CLOSE_PAREN);
    out.push_str(CLOSE_SQR);
    out.push_str(LINE_FEED);
    out
}
