// pub mod property_type;

use check_keyword::CheckKeyword;
use convert_case::Case;
use serde::{Deserialize, Serialize};

use crate::{
    parser::syntax_fragments::*,
    sap_annotations::SAPAnnotations,
    utils::{de_str_to_bool, default_false, default_true},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    pub name: String,

    #[serde(rename = "Type")]
    pub edm_type: String,

    #[serde(default = "default_true")]
    pub nullable: bool,

    pub max_length: Option<u16>,

    pub precision: Option<u16>,
    pub scale: Option<u16>,
    pub concurrency_mode: Option<String>,

    // Microsoft Annotations
    #[serde(
        rename = "m:FC_KeepInContent",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub fc_keep_in_content: bool,

    #[serde(rename = "m:FC_TargetPath")]
    pub fc_target_path: Option<String>,

    // SAP Annotations
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotations,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// The `Cargo.toml` of the application consuming the code generated by this tool must declare at least the following
/// dependencies:
///
/// ```toml
/// [dependencies]
/// chrono = { version = "0.4", features = ["serde"]}
/// rust_decimal = "1.30"
/// serde = { version = "1.0", features = ["derive"] }
/// uuid = { version = "1.4", features = ["serde"]}
/// ```
impl Property {
    fn trim_prefix<'a>(some_str: &'a str, some_prefix: &str) -> &'a str {
        if let Some(suffix) = some_str.strip_prefix(some_prefix) {
            suffix
        } else {
            some_str
        }
    }

    fn maybe_optional(rust_type: &[u8], is_optional: bool) -> Vec<u8> {
        if is_optional {
            [OPTION_DECLARATION, rust_type, TYPE_TERMINATOR].concat()
        } else {
            [rust_type].concat()
        }
    }

    fn to_rust_safe_name(&self) -> Vec<u8> {
        CheckKeyword::into_safe(convert_case::Casing::to_case(&self.name, Case::Snake))
            .as_bytes()
            .to_vec()
    }

    // For complex types, the type struct will already have been generated using the <ct_name> part extracted
    // from the OData type name conforming to one of these patterns:
    //   <namespace>.CT_<ct_name>
    //   <namespace>.<ct_name>
    //   <ct_name>
    pub fn trim_complex_type_name(type_name: &str, namespace: &str) -> Vec<u8> {
        let trimmed = Property::trim_prefix(type_name, namespace);
        let trimmed = Property::trim_prefix(trimmed, ".");
        let trimmed = Property::trim_prefix(trimmed, "CT_");

        convert_case::Casing::to_case(&trimmed, convert_case::Case::Pascal)
            .as_bytes()
            .to_vec()
    }

    fn to_rust_type(&self, namespace: &str) -> Vec<u8> {
        match self.edm_type.as_ref() {
            "Edm.Binary" => Property::maybe_optional(VECTOR_U8, self.nullable),
            "Edm.Boolean" => Property::maybe_optional(BOOLEAN, self.nullable),
            "Edm.Byte" => U8.to_vec(),
            "Edm.DateTime" => Property::maybe_optional(NAIVE_DATE_TIME, self.nullable),
            // TODO I suspect that this may not be the correct Rust datatype for Edm.DateTimeOffset...
            "Edm.DateTimeOffset" => Property::maybe_optional(NAIVE_DATE_TIME, self.nullable),
            "Edm.Decimal" => DECIMAL.to_vec(),
            "Edm.Double" => F64.to_vec(),
            "Edm.Single" => F32.to_vec(),
            "Edm.Guid" => UUID.to_vec(),
            "Edm.SByte" => Property::maybe_optional(I8, self.nullable),
            "Edm.Int16" => Property::maybe_optional(I16, self.nullable),
            "Edm.Int32" => Property::maybe_optional(I32, self.nullable),
            "Edm.Int64" => Property::maybe_optional(I64, self.nullable),
            "Edm.String" => Property::maybe_optional(STRING, self.nullable),
            "Edm.Time" => Property::maybe_optional(STD_TIME_SYSTEMTIME, self.nullable),

            type_name => Property::trim_complex_type_name(type_name, namespace),
        }
    }

    pub fn to_rust(&self, namespace: &str) -> Vec<u8> {
        [
            LINE_FEED.to_vec(),
            PUBLIC.to_vec(),
            SPACE.to_vec(),
            self.to_rust_safe_name(),
            COLON.to_vec(),
            self.to_rust_type(namespace),
            COMMA.to_vec(),
        ]
        .concat()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    pub name: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
