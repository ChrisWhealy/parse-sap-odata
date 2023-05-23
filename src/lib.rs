pub mod atom;
pub mod edmx;
pub mod ms_annotations;
pub mod oasis;
pub mod property;
pub mod sap_annotations;
pub mod utils;
pub mod xml;

pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod unit_tests;
