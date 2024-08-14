use std::fmt::Formatter;

use crate::{
    parser::{
        generate::{
            gen_bool_string, gen_opt_string, gen_opt_u16_string, gen_option_of_type, gen_owned_string,
            gen_struct_field, gen_vector_of_type,
            syntax_fragments::{serde_fragments::*, *},
        },
        AsRustSrc,
    },
    property::Property,
    utils::{odata_name_to_rust_safe_name, to_pascal_case, to_upper_camel_case},
};

static MY_NAME: &[u8] = "Property".as_bytes();

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
enum PropertyFieldNames {
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
impl PropertyFieldNames {
    pub fn value(prop_name: PropertyFieldNames) -> Vec<u8> {
        let member = match prop_name {
            PropertyFieldNames::ODataName => "odata_name",
            PropertyFieldNames::EdmType => "edm_type",
            PropertyFieldNames::Nullable => "nullable",
            PropertyFieldNames::MaxLength => "max_length",
            PropertyFieldNames::Precision => "precision",
            PropertyFieldNames::Scale => "scale",
            PropertyFieldNames::ConcurrencyMode => "concurrency_mode",
            PropertyFieldNames::FcKeepInContent => "fc_keep_in_content",
            PropertyFieldNames::FcTargetPath => "fc_target_path",
            PropertyFieldNames::SAPAnnotations => "sap_annotations",
            PropertyFieldNames::DeserializerFn => "deserializer_fn",
            PropertyFieldNames::DeserializerModule => "deserializer_module",
        };

        member.as_bytes().to_vec()
    }
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
        let type_name_parts = self.edm_type.split('.').collect::<Vec<&str>>();

        // The type name should contain exactly two parts
        if type_name_parts.len() == 2 {
            if type_name_parts[0].eq("Edm") {
                PropertyType::Edm(type_name_parts[1].to_owned())
            } else {
                PropertyType::Complex(type_name_parts[1].to_owned())
            }
        } else {
            // TODO This is likely an error condition. Need to decide what to do here...
            PropertyType::Unqualified
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from(prop_md: PropertyFieldNames, val: Vec<u8>) -> Vec<u8> {
    [&*PropertyFieldNames::value(prop_md), COLON, &val, COMMA, LINE_FEED].concat()
}

impl std::fmt::Display for Property {
    // Output the source code to declare this Property instance
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let deser_fn = [DOUBLE_QUOTE, self.deserializer_fn.as_bytes(), DOUBLE_QUOTE].concat();
        let deser_mod = [DOUBLE_QUOTE, self.deserializer_module.as_bytes(), DOUBLE_QUOTE].concat();

        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from(PropertyFieldNames::ODataName, gen_owned_string(&self.odata_name.clone())),
            &*line_from(PropertyFieldNames::EdmType, gen_owned_string(&self.edm_type.clone())),
            &*line_from(PropertyFieldNames::Nullable, gen_bool_string(self.nullable)),
            &*line_from(PropertyFieldNames::MaxLength, gen_opt_u16_string(self.max_length)),
            &*line_from(PropertyFieldNames::Precision, gen_opt_u16_string(self.precision)),
            &*line_from(PropertyFieldNames::Scale, gen_opt_u16_string(self.scale)),
            &*line_from(PropertyFieldNames::ConcurrencyMode, gen_opt_string(&self.concurrency_mode)),
            &*line_from(PropertyFieldNames::FcKeepInContent, gen_bool_string(self.fc_keep_in_content)),
            &*line_from(PropertyFieldNames::FcTargetPath, gen_opt_string(&self.fc_target_path)),
            &*line_from(
                PropertyFieldNames::SAPAnnotations,
                format!("{}", self.sap_annotations).as_bytes().to_vec(),
            ),
            &*line_from(PropertyFieldNames::DeserializerFn, deser_fn),
            &*line_from(PropertyFieldNames::DeserializerModule, deser_mod),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl AsRustSrc for Property {
    fn to_rust(&self) -> Vec<u8> {
        let mut out_buffer: Vec<u8> = Vec::new();

        let resolved_prop_type: Vec<u8> = match Self::get_property_type(&self) {
            PropertyType::Edm(edm_type) => {
                // Since the field names coming out of SAP do not always use strict PascalCase formatting.
                // The abbreviation "ID" is often used when you would expect "Id"
                // E.G. SAP outputs a field called "BusinessPartnerID" when you would expect "BusinessPartnerId"
                // It is assumed that the OData field name always starts with a capital letter
                if !to_pascal_case(&self.odata_name).eq(&self.odata_name) {
                    out_buffer.append(&mut gen_serde_rename(&self.odata_name))
                }

                // Add an attribute pointing either to a custom deserializer function or a deserializer module.
                // Only one of these deserializers should ever be populated at any one time!
                if !self.deserializer_fn.is_empty() {
                    out_buffer.append(&mut deserialize_with(self.deserializer_fn, true))
                }
                if !self.deserializer_module.is_empty() {
                    out_buffer.append(&mut deserialize_with(self.deserializer_module, false))
                }

                // Convert EDM type to Rust type
                match edm_type.as_str() {
                    "Binary" => self.maybe_optional(&*gen_vector_of_type(U8)),
                    "Boolean" => self.maybe_optional(BOOLEAN),
                    "Byte" => U8.to_vec(),
                    "DateTime" | "DateTimeOffset" => self.maybe_optional(NAIVE_DATE_TIME),
                    "Decimal" => self.maybe_optional(RUST_DECIMAL),
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

            PropertyType::Complex(cmplx_type) => to_upper_camel_case(&cmplx_type).into_bytes(),

            // TODO Need to decide what to do with an unqualified property type
            // Simply writing it out in the hope that the source code compiles is probably not a good idea...
            PropertyType::Unqualified => self.edm_type.clone().into_bytes(),
        };

        out_buffer.append(&mut gen_struct_field(
            &odata_name_to_rust_safe_name(&self.odata_name),
            &resolved_prop_type,
        ));

        out_buffer
    }
}
