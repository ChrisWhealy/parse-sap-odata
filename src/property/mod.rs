use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[cfg(feature = "parser")]
use crate::{
    parser::{
        syntax_fragments::{
            fragment_generators::{
                gen_bool_string, gen_opt_string, gen_opt_u16_string, gen_option_of_type, gen_owned_string,
                gen_struct_field, gen_type_name, gen_vector_of,
            },
            serde_fragments::*,
            *,
        },
        AsRustSrc,
    },
    sap_annotations::property::SAPAnnotationsProperty,
    utils::{de_str_to_bool, default_false, default_true, odata_name_to_rust_safe_name, to_pascal_case},
};

pub mod property_ref;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Property type flags
///
/// A `<Property>` within an `<EntityType>` can be one of three types:
/// * **`PropertyType::Edm()`**<br>An entity data model type such as `String`, `DateTime`, `Decimal` etc
/// * **`PropertyType::Complex()`**<br>A Complex Type defined within the Schema's namespace containing multiple fields
/// * **`PropertyType::Unqualified`**<br>The type name is missing its namespace qualifier.  Need to decide if this is an error condition
pub enum PropertyType {
    Edm(String),
    Complex(String),
    Unqualified,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub enum PropertyMetadata {
    ODataName,
    EdmType,
    Nullable,
    MaxLength,
    Precision,
    Scale,
    ConcurrencyMode,
    FcKeepInContent,
    FcTargetPath,
    SAPAnnotations,
    DeserializerFn,
    DeserializerModule,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl PropertyMetadata {
    pub fn get_field_name(prop_name: PropertyMetadata) -> Vec<u8> {
        let member = match prop_name {
            PropertyMetadata::ODataName => "odata_name",
            PropertyMetadata::EdmType => "edm_type",
            PropertyMetadata::Nullable => "nullable",
            PropertyMetadata::MaxLength => "max_length",
            PropertyMetadata::Precision => "precision",
            PropertyMetadata::Scale => "scale",
            PropertyMetadata::ConcurrencyMode => "concurrency_mode",
            PropertyMetadata::FcKeepInContent => "fc_keep_in_content",
            PropertyMetadata::FcTargetPath => "fc_target_path",
            PropertyMetadata::SAPAnnotations => "sap_annotations",
            PropertyMetadata::DeserializerFn => "deserializer_fn",
            PropertyMetadata::DeserializerModule => "deserializer_module",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `edm:Property` element
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    #[serde(rename = "@Name")]
    pub odata_name: String,

    #[serde(rename = "@Type")]
    pub edm_type: String,

    #[serde(rename = "@Nullable", default = "default_true")]
    pub nullable: bool,

    #[serde(rename = "@MaxLength")]
    pub max_length: Option<u16>,

    #[serde(rename = "@Precision")]
    pub precision: Option<u16>,

    #[serde(rename = "@Scale")]
    pub scale: Option<u16>,

    #[serde(rename = "@ConcurrencyMode")]
    pub concurrency_mode: Option<String>,

    // Microsoft Annotations
    #[serde(
        rename = "@FC_KeepInContent",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub fc_keep_in_content: bool,

    #[serde(rename = "@FC_TargetPath")]
    pub fc_target_path: Option<String>,

    // SAP Annotations
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsProperty,

    #[serde(skip, default)]
    pub deserializer_fn: &'static str,

    #[serde(skip, default)]
    pub deserializer_module: &'static str,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl Property {
    fn maybe_optional(&self, rust_type: &[u8]) -> Vec<u8> {
        if self.nullable {
            gen_option_of_type(rust_type)
        } else {
            rust_type.to_vec()
        }
    }

    pub fn get_property_type(&self) -> PropertyType {
        let type_name_parts: Vec<&str> = self.edm_type.split('.').collect();

        // The type name should contain exactly two parts
        if type_name_parts.len() == 2 {
            if type_name_parts[0].eq("Edm") {
                PropertyType::Edm(type_name_parts[1].to_owned())
            } else {
                PropertyType::Complex(type_name_parts[1].to_owned())
            }
        } else {
            PropertyType::Unqualified
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(prop_md: PropertyMetadata, val: Vec<u8>) -> Vec<u8> {
    [&*PropertyMetadata::get_field_name(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut src_str: Vec<u8> = vec![];
        let deser_fn = [DOUBLE_QUOTE, self.deserializer_fn.as_bytes(), DOUBLE_QUOTE].concat();
        let deser_mod = [DOUBLE_QUOTE, self.deserializer_module.as_bytes(), DOUBLE_QUOTE].concat();

        src_str.append(&mut [PROPERTY, OPEN_CURLY].concat());
        src_str.append(&mut line_from(
            PropertyMetadata::ODataName,
            gen_owned_string(&self.odata_name.clone()),
        ));
        src_str.append(&mut line_from(
            PropertyMetadata::EdmType,
            gen_owned_string(&self.edm_type.clone()),
        ));
        src_str.append(&mut line_from(PropertyMetadata::Nullable, gen_bool_string(self.nullable)));
        src_str.append(&mut line_from(PropertyMetadata::MaxLength, gen_opt_u16_string(self.max_length)));
        src_str.append(&mut line_from(PropertyMetadata::Precision, gen_opt_u16_string(self.precision)));
        src_str.append(&mut line_from(PropertyMetadata::Scale, gen_opt_u16_string(self.scale)));
        src_str.append(&mut line_from(
            PropertyMetadata::ConcurrencyMode,
            gen_opt_string(&self.concurrency_mode),
        ));
        src_str.append(&mut line_from(
            PropertyMetadata::FcKeepInContent,
            gen_bool_string(self.fc_keep_in_content),
        ));
        src_str.append(&mut line_from(
            PropertyMetadata::FcTargetPath,
            gen_opt_string(&self.fc_target_path),
        ));
        src_str.append(&mut line_from(
            PropertyMetadata::SAPAnnotations,
            format!("{}", self.sap_annotations).as_bytes().to_vec(),
        ));
        src_str.append(&mut line_from(PropertyMetadata::DeserializerFn, deser_fn));
        src_str.append(&mut line_from(PropertyMetadata::DeserializerModule, deser_mod));
        src_str.append(&mut [CLOSE_CURLY].concat());

        write!(f, "{}", String::from_utf8(src_str).unwrap())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
impl AsRustSrc for Property {
    fn to_rust(&self) -> Vec<u8> {
        let mut out_buffer: Vec<u8> = Vec::new();

        let resolved_prop_type: Vec<u8> = match Self::get_property_type(&self) {
            PropertyType::Edm(edm_type) => {
                // Since SAP does not use strict PascalCase formatting for abbreviations such as "ID", we need to catch
                // those cases here and issue a serde_rename attribute.
                // E.G. SAP outputs fields called "BusinessPartnerID" when you would expect "BusinessPartnerId"
                if !to_pascal_case(&self.odata_name).eq(&self.odata_name) {
                    out_buffer.extend(gen_serde_rename(&self.odata_name))
                }

                // Add an attribute pointing either to a custom deserializer function or a deserializer module.
                // Only one of these deserializers should ever be populated at any one time!
                if !self.deserializer_fn.is_empty() {
                    out_buffer.extend(deserialize_with(self.deserializer_fn, true))
                }
                if !self.deserializer_module.is_empty() {
                    out_buffer.extend(deserialize_with(self.deserializer_module, false))
                }

                // Convert EDM type to Rust type
                match edm_type.as_str() {
                    "Binary" => self.maybe_optional(&*gen_vector_of(U8)),
                    "Boolean" => self.maybe_optional(BOOLEAN),
                    "Byte" => U8.to_vec(),
                    "DateTime" | "DateTimeOffset" => self.maybe_optional(NAIVE_DATE_TIME),
                    "Decimal" => self.maybe_optional(DECIMAL),
                    "Double" => F64.to_vec(),
                    "Guid" => UUID.to_vec(),
                    "Int16" => self.maybe_optional(I16),
                    "Int32" => self.maybe_optional(I32),
                    "Int64" => self.maybe_optional(I64),
                    // EDM allows for null which is intentionally excluded by Rust
                    "Null" => UNIT.to_vec(),
                    "SByte" => self.maybe_optional(I8),
                    "Single" => F32.to_vec(),
                    "Time" => self.maybe_optional(STD_TIME_SYSTEMTIME),

                    // Use String as the catch-all case
                    _ => self.maybe_optional(STRING),
                }
            },

            PropertyType::Complex(cmplx_type) => gen_type_name(&cmplx_type).into_bytes(),

            // TODO Need to decide what to do with an unqualified property type
            // Simply writing it out in the hope that the source code compiles is probably not a good idea...
            PropertyType::Unqualified => self.edm_type.clone().into_bytes(),
        };

        out_buffer.extend(gen_struct_field(
            &odata_name_to_rust_safe_name(&self.odata_name),
            &resolved_prop_type,
        ));

        out_buffer
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
