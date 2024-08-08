pub mod derive_traits;
pub mod fragment_generators;
pub mod serde_fragments;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Single characters
pub static LINE_FEED: &[u8] = &[0x0a];
pub static SPACE: &[u8] = &[0x20];
pub static DOUBLE_QUOTE: &[u8] = &[0x22];
pub static OPEN_PAREN: &[u8] = &[0x28];
pub static CLOSE_PAREN: &[u8] = &[0x29];
pub static COMMA: &[u8] = &[0x2C];
pub static COLON: &[u8] = &[0x3A];
pub static SEMI_COLON: &[u8] = &[0x3B];
pub static OPEN_ANGLE: &[u8] = &[0x3C];
pub static CLOSE_ANGLE: &[u8] = &[0x3E];
pub static OPEN_SQR: &[u8] = &[0x5B];
pub static CLOSE_SQR: &[u8] = &[0x5D];
pub static OPEN_CURLY: &[u8] = &[0x7B];
pub static CLOSE_CURLY: &[u8] = &[0x7D];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Comment separators
pub static COMMMENT_LINE: &[u8] = "// ".as_bytes();
pub static SEPARATOR: &[u8] = "// -----------------------------------------------------------------------------
"
.as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Syntactical separators
pub static THIN_ARROW: &[u8] = &[0x2D, 0x3E];
pub static COLON2: &[u8] = &[0x3A, 0x3A];
pub static FAT_ARROW: &[u8] = &[0x3D, 0x3E];
pub static END_BLOCK: &[u8] = &[0x7D, 0x0A, 0x0A];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Primitive type names
pub static BOOLEAN: &[u8] = "bool".as_bytes();
pub static F32: &[u8] = "f32".as_bytes();
pub static F64: &[u8] = "f64".as_bytes();
pub static I8: &[u8] = "i8".as_bytes();
pub static I16: &[u8] = "i16".as_bytes();
pub static I32: &[u8] = "i32".as_bytes();
pub static I64: &[u8] = "i64".as_bytes();
pub static U8: &[u8] = "u8".as_bytes();
pub static UNIT: &[u8] = "()".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Keywords or keyword fragments
pub static ENUM: &[u8] = "enum ".as_bytes();
pub static FN: &[u8] = "fn ".as_bytes();
pub static IMPL: &[u8] = "impl ".as_bytes();
pub static MOD: &[u8] = "mod ".as_bytes();
pub static PUBLIC: &[u8] = "pub ".as_bytes();
pub static CONST: &[u8] = "const ".as_bytes();
pub static USE: &[u8] = "use ".as_bytes();
pub static KEY: &[u8] = "key".as_bytes();
pub static MATCH_SELF: &[u8] = "match *self ".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Primitive values and basic types
pub static OPTION: &[u8] = "Option".as_bytes();
pub static SOME: &[u8] = "Some".as_bytes();
pub static NONE: &[u8] = "None".as_bytes();
pub static STRING: &[u8] = "String".as_bytes();
pub static VECTOR: &[u8] = "Vec".as_bytes();
pub static SELF_REF: &[u8] = "&self".as_bytes();
pub static STATIC_STR_REF: &[u8] = "&'static str".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Macro calls
pub static VEC_BANG: &[u8] = "vec![".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// External types
pub static DECIMAL: &[u8] = "rust_decimal::Decimal".as_bytes();
pub static NAIVE_DATE_TIME: &[u8] = "chrono::NaiveDateTime".as_bytes();
pub static STD_TIME_SYSTEMTIME: &[u8] = "std::time::SystemTime".as_bytes();
pub static UUID: &[u8] = "uuid::Uuid".as_bytes();
pub static PROPERTY: &[u8] = "Property".as_bytes();
pub static PROPERTYREF: &[u8] = "PropertyRef".as_bytes();
pub static COMPLEX_TYPE: &[u8] = "ComplexType".as_bytes();
pub static METADATA: &'static str = "Metadata";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Function names used in enum implementations
pub static FN_NAME_VARIANT_NAME: &[u8] = "variant_name".as_bytes();
pub static FN_NAME_VARIANT_NAMES: &[u8] = "variant_names".as_bytes();
pub static FN_VARIANT_NAME_START: &[u8] = "pub const fn variant_name(&self) -> &'static str {
    match *self {
"
.as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Compiler attributes and paths to used types
pub static RUSTC_ALLOW_DEAD_CODE: &[u8] = "#[allow(dead_code)]
"
.as_bytes();
pub fn gen_use_path(path: &[u8]) -> Vec<u8> {
    [USE, path, SEMI_COLON, LINE_FEED].concat()
}
pub static PATH_TO_EDMX_COMPLEX_TYPE: &[u8] =
    "parse_sap_odata::edmx::data_services::schema::complex_type::ComplexType".as_bytes();
pub static PATH_TO_EDMX_SCHEMA_ASSOCIATION_SETS: &[u8] =
    "parse_sap_odata::edmx::data_services::schema::entity_container::association_set::AssociationSet".as_bytes();
pub static PATH_TO_EDMX_SCHEMA_ASSOCIATION_TYPES: &[u8] = "parse_sap_odata::edmx::data_services::schema::association::{
    end::End,
    referential_constraint::{ReferentialConstraint, principal::Principal, dependent::Dependent},
    Association,
}"
.as_bytes();
pub static PATH_TO_SAP_AGGREGATION_PROPERTY: &[u8] =
    "parse_sap_odata::sap_annotations::SAPAggregationProperty".as_bytes();
pub static PATH_TO_SAP_ANNOTATIONS_ASSOCIATION_SET: &[u8] =
    "parse_sap_odata::sap_annotations::association_set::SAPAnnotationsAssociationSet".as_bytes();
pub static PATH_TO_SAP_ANNOTATIONS_DISPLAY_FORMAT_PROPERTY: &[u8] =
    "parse_sap_odata::sap_annotations::display_format::SAPDisplayFormatProperty".as_bytes();
pub static PATH_TO_SAP_ANNOTATIONS_FIELD_CONTROL_PROPERTY: &[u8] =
    "parse_sap_odata::sap_annotations::field_control::SAPFieldControlProperty".as_bytes();
pub static PATH_TO_SAP_ANNOTATIONS_FILTER_RESTRICTION_PROPERTY: &[u8] =
    "parse_sap_odata::sap_annotations::filter_restriction::SAPFilterRestrictionProperty".as_bytes();
pub static PATH_TO_SAP_ANNOTATIONS_PARAMETER_PROPERTY: &[u8] =
    "parse_sap_odata::sap_annotations::property::SAPParameterProperty".as_bytes();
pub static PATH_TO_SAP_ANNOTATIONS_PROPERTY: &[u8] =
    "parse_sap_odata::sap_annotations::property::SAPAnnotationsProperty".as_bytes();
pub static PATH_TO_SAP_ODATA_PROPERTIES: &[u8] =
    "parse_sap_odata::property::{Property, property_ref::PropertyRef}".as_bytes();
pub static PATH_TO_SAP_SEMANTICS_PROPERTY: &[u8] =
    "parse_sap_odata::sap_semantics::property::SAPSemanticsProperty".as_bytes();
