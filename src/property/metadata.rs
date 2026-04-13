use std::fmt::Formatter;

use crate::property::edm_primitive::EdmPrimitive;
use crate::property::edm_type::EdmType;
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
    utils::{odata_name_to_rust_safe_name, to_pascal_case},
};

static MY_NAME: &str = "Property";

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
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_into(f: &mut Formatter<'_>, prop_md: PropertyFieldNames, val: &str) -> std::fmt::Result {
    write!(f, "{}{COLON}{val}{COMMA}{LINE_FEED}", PropertyFieldNames::value(prop_md))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Renders an `EdmType` value as a Rust source code expression for use in generated metadata modules
fn edm_type_to_rust_src(edm_type: &EdmType) -> String {
    match edm_type {
        EdmType::Primitive(prim) => {
            let prim_src = match prim {
                EdmPrimitive::Unknown(s) => format!("EdmPrimitive::Unknown({})", gen_owned_string_src(s)),
                _ => format!("EdmPrimitive::{prim:?}"),
            };
            format!("EdmType::Primitive({prim_src})")
        },
        EdmType::Complex(s) => format!("EdmType::Complex({})", gen_owned_string_src(s)),
        EdmType::Unknown(s) => format!("EdmType::Unknown({})", gen_owned_string_src(s)),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Metadata Module Generation
/// Generate the source code that declares an instance of this Property
impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_into(f, PropertyFieldNames::ODataName, &gen_owned_string_src(&self.odata_name))?;
        line_into(f, PropertyFieldNames::EdmType, &edm_type_to_rust_src(&self.edm_type))?;
        line_into(f, PropertyFieldNames::Nullable, gen_bool_string(self.nullable))?;
        line_into(f, PropertyFieldNames::MaxLength, &gen_opt_u16_string_src(self.max_length))?;
        line_into(f, PropertyFieldNames::Precision, &gen_opt_u16_string_src(self.precision))?;
        line_into(f, PropertyFieldNames::Scale, &gen_opt_u16_string_src(self.scale))?;
        line_into(
            f,
            PropertyFieldNames::ConcurrencyMode,
            &gen_opt_string_src(&self.concurrency_mode),
        )?;
        line_into(f, PropertyFieldNames::FcKeepInContent, gen_bool_string(self.fc_keep_in_content))?;
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

        let (resolved_prop_type, crate_ref) = match self.edm_type.clone() {
            EdmType::Primitive(prim) => {
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
                let src = match prim {
                    EdmPrimitive::Binary => self.maybe_optional(&gen_vector_of_type_src(U8)),
                    EdmPrimitive::Boolean => self.maybe_optional(BOOLEAN),
                    EdmPrimitive::Byte => U8.to_string(),
                    EdmPrimitive::DateTime | EdmPrimitive::DateTimeOffset => self.maybe_optional(NAIVE_DATE_TIME),
                    EdmPrimitive::Decimal => self.maybe_optional(RUST_DECIMAL),
                    EdmPrimitive::Double => F64.to_string(),
                    EdmPrimitive::Guid => UUID.to_string(),
                    EdmPrimitive::Int16 => self.maybe_optional(I16),
                    EdmPrimitive::Int32 => self.maybe_optional(I32),
                    EdmPrimitive::Int64 => self.maybe_optional(I64),
                    // EDM allows for null which is intentionally excluded by Rust
                    EdmPrimitive::Null => UNIT.to_string(),
                    EdmPrimitive::SByte => self.maybe_optional(I8),
                    EdmPrimitive::Single => F32.to_string(),
                    EdmPrimitive::Time => self.maybe_optional(STD_TIME_SYSTEMTIME),

                    // Catch-all for both Unknown and String
                    _ => self.maybe_optional(STRING),
                };

                (src, prim.get_crate_ref())
            },

            // EdmType::Complex always holds a ready-to-use Rust type name (UpperCamelCase, no namespace prefix)
            EdmType::Complex(cmplx_type) => (cmplx_type, ""),

            // Truly unrecognised type: emit as-is and hope for the best
            EdmType::Unknown(unknown_type) => (unknown_type, ""),
        };

        gen_struct_field_into(
            &mut out_buffer,
            &odata_name_to_rust_safe_name(&self.odata_name),
            &resolved_prop_type,
        );

        (out_buffer, crate_ref.to_owned())
    }
}
