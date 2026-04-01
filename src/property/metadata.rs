use std::fmt::Formatter;

use crate::{
    parser::{
        generate::{
            gen_bool_string, gen_custom_deserializer_info, gen_opt_string_src, gen_opt_u16_string_src,
            gen_owned_string_src, gen_struct_field_into, gen_vector_of_type_src,
            syntax_fragments::{
                serde_fragments::{gen_deserialize_with, gen_serde_rename},
                *,
            },
        },
        AsRustSrc,
    },
    property::Property,
    utils::{odata_name_to_rust_safe_name, to_pascal_case, to_upper_camel_case},
};

static MY_NAME: &str = "Property";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Property type flags
///
/// A `<Property>` within an `<EntityType>` can be one of three types:
/// * **`PropertyType::Edm(String, String)`**
///
///    An entity data model type such as `String`, `DateTime` or `Decimal` followed by a possible external crate reference
/// * **`PropertyType::Complex(String)`**
///
///   A Complex Type defined within the Schema's namespace containing multiple fields
/// * **`PropertyType::Unqualified`**
///
///    The type name is missing its namespace qualifier.  Need to decide if this is an error condition
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyType {
    Edm(String, String),
    Complex(String),
    Unqualified,
}

impl std::fmt::Display for PropertyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyType::Edm(t, cr) => write!(f, "Edm({t}, {cr})"),
            PropertyType::Complex(ct) => write!(f, "Complex({ct})"),
            PropertyType::Unqualified => write!(f, "Unqualified"),
        }
    }
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
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl PropertyFieldNames {
    pub fn value(prop_name: PropertyFieldNames) -> &'static str {
        match prop_name {
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
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl Property {
    fn maybe_optional(&self, rust_type: &str) -> String {
        if self.nullable {
            format!("Option<{rust_type}>")
        } else {
            rust_type.to_string()
        }
    }

    pub fn get_property_type(&self) -> PropertyType {
        // The type name should contain exactly two parts
        if let Some((part1, part2)) = self.edm_type.split_once('.') {
            if !part1.is_empty() && !part2.is_empty() {
                if part1.eq("Edm") {
                    let crate_ref = match part2 {
                        "DateTime" | "DateTimeOffset" => CRATE_CHRONO,
                        "Decimal" => CRATE_RUST_DECIMAL,
                        "Guid" => CRATE_GUID,
                        _ => "",
                    };

                    PropertyType::Edm(part2.to_owned(), crate_ref.to_owned())
                } else {
                    PropertyType::Complex(part2.to_owned())
                }
            } else {
                // TODO This is likely an error condition. Need to decide what to do here...
                PropertyType::Unqualified
            }
        } else {
            // TODO This is likely an error condition. Need to decide what to do here...
            PropertyType::Unqualified
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_into(f: &mut Formatter<'_>, prop_md: PropertyFieldNames, val: &str) -> std::fmt::Result {
    write!(f, "{}{COLON}{val}{COMMA}{LINE_FEED}", PropertyFieldNames::value(prop_md))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Metadata Module Generation
/// Generate the source code that declares an instance of this Property
impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_into(f, PropertyFieldNames::ODataName, &gen_owned_string_src(&self.odata_name))?;
        line_into(f, PropertyFieldNames::EdmType, &gen_owned_string_src(&self.edm_type))?;
        line_into(f, PropertyFieldNames::Nullable, gen_bool_string(self.nullable))?;
        line_into(f, PropertyFieldNames::MaxLength, &gen_opt_u16_string_src(self.max_length))?;
        line_into(f, PropertyFieldNames::Precision, &gen_opt_u16_string_src(self.precision))?;
        line_into(f, PropertyFieldNames::Scale, &gen_opt_u16_string_src(self.scale))?;
        line_into(
            f,
            PropertyFieldNames::ConcurrencyMode,
            &gen_opt_string_src(&self.concurrency_mode),
        )?;
        line_into(
            f,
            PropertyFieldNames::FcKeepInContent,
            gen_bool_string(self.fc_keep_in_content),
        )?;
        line_into(f, PropertyFieldNames::FcTargetPath, &gen_opt_string_src(&self.fc_target_path))?;
        line_into(f, PropertyFieldNames::SAPAnnotations, &self.sap_annotations.to_string())?;
        line_into(
            f,
            PropertyFieldNames::DeserializerFn,
            &gen_owned_string_src(&self.deserializer_fn),
        )?;
        write!(f, "{CLOSE_CURLY}")
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Service Document Module Generation
/// Generate the source code that declares an instance of the runtime data stored in this Property
impl AsRustSrc for Property {
    type CrateRef = String;

    fn to_rust(&self) -> (String, Self::CrateRef) {
        let mut out_buffer = String::new();

        let (resolved_prop_type, crate_ref) = match self.get_property_type() {
            PropertyType::Edm(edm_type, crate_ref) => {
                // It is assumed that the OData field name always starts with a capital letter
                //
                // WARNING: Field names coming out of SAP do not always use strict PascalCase formatting.
                // For example, you will often see field names containing the abbreviation "ID" when you would expect
                // "Id" as in "BusinessPartnerID" instead of "BusinessPartnerId"
                if !to_pascal_case(&self.odata_name).eq(&self.odata_name) {
                    out_buffer.push_str(&gen_serde_rename(&self.odata_name));
                }

                // Output the serde attribute for a custom deserializer
                let deserializer_fn = gen_custom_deserializer_info(self);
                if !deserializer_fn.is_empty() {
                    out_buffer.push_str(&gen_deserialize_with(&deserializer_fn));
                }

                // Generate source code for Rust type
                let src = match edm_type.as_str() {
                    "Binary" => self.maybe_optional(&gen_vector_of_type_src(U8)),
                    "Boolean" => self.maybe_optional(BOOLEAN),
                    "Byte" => U8.to_string(),
                    "DateTime" | "DateTimeOffset" => self.maybe_optional(NAIVE_DATE_TIME),
                    "Decimal" => self.maybe_optional(RUST_DECIMAL),
                    "Double" => F64.to_string(),
                    "Guid" => UUID.to_string(),
                    "Int16" => self.maybe_optional(I16),
                    "Int32" => self.maybe_optional(I32),
                    "Int64" => self.maybe_optional(I64),
                    // EDM allows for null which is intentionally excluded by Rust
                    "Null" => UNIT.to_string(),
                    "SByte" => self.maybe_optional(I8),
                    "Single" => F32.to_string(),
                    "Time" => self.maybe_optional(STD_TIME_SYSTEMTIME),

                    // Use String as the catch-all case
                    _ => self.maybe_optional(STRING),
                };

                (src, crate_ref)
            },

            PropertyType::Complex(cmplx_type) => (to_upper_camel_case(&cmplx_type), "".to_string()),

            // TODO Need to decide what to do with an unqualified property type
            // Simply writing it out in the hope that the source code compiles is probably not a good idea...
            PropertyType::Unqualified => (self.edm_type.clone(), "".to_string()),
        };

        gen_struct_field_into(
            &mut out_buffer,
            &odata_name_to_rust_safe_name(&self.odata_name),
            &resolved_prop_type,
        );

        (out_buffer, crate_ref)
    }
}
