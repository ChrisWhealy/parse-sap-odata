pub mod edmx;
pub mod macros;
pub mod oasis;
pub mod property;
pub mod sap_annotations;
pub mod sap_semantics;
pub mod utils;
pub mod xml;

#[cfg(feature = "parser")]
pub mod parser;
#[cfg(feature = "parser")]
#[cfg(test)]
mod test_utils;
