pub mod derive_traits;
pub mod serde_fragments;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Single characters
pub static LINE_FEED: &str = "\n";
pub static SPACE: &str = " ";
pub static DOUBLE_QUOTE: &str = "\"";
pub static OPEN_PAREN: &str = "(";
pub static CLOSE_PAREN: &str = ")";
pub static COMMA: &str = ",";
pub static COLON: &str = ":";
pub static SEMI_COLON: &str = ";";
pub static OPEN_ANGLE: &str = "<";
pub static CLOSE_ANGLE: &str = ">";
pub static OPEN_SQR: &str = "[";
pub static CLOSE_SQR: &str = "]";
pub static OPEN_CURLY: &str = "{";
pub static CLOSE_CURLY: &str = "}";
pub static UNDERSCORE: &str = "_";
pub static ONE: &str = "1";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Comment separators
pub static COMMENT_LINE: &str = "// ";
pub static SEPARATOR: &str = "
// -----------------------------------------------------------------------------
";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Syntactical separators
pub static THIN_ARROW: &str = "->";
pub static COLON2: &str = "::";
pub static FAT_ARROW: &str = "=>";
pub static END_BLOCK: &str = "}\n\n";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Primitive type names
pub static BOOLEAN: &str = "bool";
pub static F32: &str = "f32";
pub static F64: &str = "f64";
pub static I8: &str = "i8";
pub static I16: &str = "i16";
pub static I32: &str = "i32";
pub static I64: &str = "i64";
pub static U8: &str = "u8";
pub static UNIT: &str = "()";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Keywords
pub static ENUM: &str = "enum ";
pub static EXTERN_CRATE: &str = "extern crate ";
pub static FN: &str = "fn ";
pub static IMPL: &str = "impl ";
pub static MOD: &str = "mod ";
pub static PUBLIC: &str = "pub ";
pub static CONST: &str = "const ";
pub static USE: &str = "use ";
pub static KEY: &str = "key";
pub static MATCH_SELF: &str = "match *self ";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Primitive values and basic types
pub static OPTION: &str = "Option";
pub static SOME: &str = "Some";
pub static NONE: &str = "None";
pub static STRING: &str = "String";
pub static VECTOR: &str = "Vec";
pub static SELF_REF: &str = "&self";
pub static STATIC_STR_REF: &str = "&'static str";
pub static ATOM: &str = "atom";
pub static JSON: &str = "JSON";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Macro calls
pub static VEC_BANG: &str = "vec![";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// External types
pub static EDMX_DATE_TIME: &str = "DateTime";
pub static EDMX_DATE_TIME_OFFSET: &str = "DateTimeOffset";
pub static EDMX_DECIMAL: &str = "Decimal";
pub static RUST_DECIMAL: &str = "rust_decimal::Decimal";
pub static NAIVE_DATE_TIME: &str = "chrono::NaiveDateTime";
pub static STD_TIME_SYSTEMTIME: &str = "std::time::SystemTime";
pub static UUID: &str = "uuid::Uuid";
pub static PROPERTY: &str = "Property";
pub static PROPERTYREF: &str = "PropertyRef";
pub static COMPLEX_TYPE: &str = "ComplexType";
pub static METADATA: &str = "Metadata";
pub static ASSOCIATION_SETS: &str = "AssociationSets";
pub static ASSOCIATION_SET: &str = "AssociationSet";
pub static ASSOCIATIONS: &str = "Associations";
pub static ASSOCIATION: &str = "Association";
pub static COMPLEX_TYPES: &str = "ComplexTypes";
pub static ENTITY_TYPES: &str = "EntityTypes";
pub static SUFFIX_SNAKE_METADATA: &str = "_metadata";
pub static PREFIX_SNAKE_GET: &str = "get_";
pub static FIELD_NAME_KEY: &str = "key";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Function names used in enum implementations
pub static FN_NAME_VARIANT_NAME: &str = "variant_name";
pub static FN_NAME_VARIANT_NAMES: &str = "variant_names";
pub static FN_VARIANT_NAME_START: &str = "pub const fn variant_name(&self) -> &'static str {
    match *self {
";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// External crate dependencies
pub static CRATE_PARSE_SAP_ATOM_FEED: &str = "parse_sap_atom_feed";
pub static CRATE_SERDE: &str = "serde";
pub static CRATE_QUICK_XML: &str = "quick_xml";
pub static CRATE_CHRONO: &str = "chrono";
pub static CRATE_RUST_DECIMAL: &str = "rust_decimal";
pub static CRATE_GUID: &str = "uuid";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Compiler attributes and paths to used types
pub static DERIVE_START: &str = "#[derive(";
pub static RUSTC_ALLOW_DEAD_CODE: &str = "#[allow(dead_code)]\n";

/// Paths to used types
pub fn gen_use_path(out: &mut String, path: &str) {
    out.push_str(USE);
    out.push_str(path);
    out.push_str(SEMI_COLON);
    out.push_str(LINE_FEED);
}

pub fn gen_use_path_into(out: &mut String, path: &str) {
    out.push_str(USE);
    out.push_str(path);
    out.push_str(SEMI_COLON);
    out.push_str(LINE_FEED);
}

pub static PATH_TO_EDMX_COMPLEX_TYPE: &str =
    "parse_sap_odata::edmx::data_services::schema::complex_type::ComplexType";
pub static PATH_TO_EDMX_SCHEMA_ASSOCIATION_SETS: &str =
    "parse_sap_odata::edmx::data_services::schema::entity_container::association_set::AssociationSet";
pub static PATH_TO_EDMX_SCHEMA_ASSOCIATION_TYPES: &str = "parse_sap_odata::edmx::data_services::schema::association::{
    end::End,
    referential_constraint::{ReferentialConstraint, principal::Principal, dependent::Dependent},
    Association,
}";
pub static PATH_TO_SAP_AGGREGATION_PROPERTY: &str =
    "parse_sap_odata::sap_annotations::SAPAggregationProperty";
pub static PATH_TO_SAP_ANNOTATIONS_ASSOCIATION_SET: &str =
    "parse_sap_odata::sap_annotations::association_set::SAPAnnotationsAssociationSet";
pub static PATH_TO_SAP_ANNOTATIONS_DISPLAY_FORMAT_PROPERTY: &str =
    "parse_sap_odata::sap_annotations::display_format::SAPDisplayFormatProperty";
pub static PATH_TO_SAP_ANNOTATIONS_FIELD_CONTROL_PROPERTY: &str =
    "parse_sap_odata::sap_annotations::field_control::SAPFieldControlProperty";
pub static PATH_TO_SAP_ANNOTATIONS_FILTER_RESTRICTION_PROPERTY: &str =
    "parse_sap_odata::sap_annotations::filter_restriction::SAPFilterRestrictionProperty";
pub static PATH_TO_SAP_ANNOTATIONS_PARAMETER_PROPERTY: &str =
    "parse_sap_odata::sap_annotations::property::SAPParameterProperty";
pub static PATH_TO_SAP_ANNOTATIONS_PROPERTY: &str =
    "parse_sap_odata::sap_annotations::property::SAPAnnotationsProperty";
pub static PATH_TO_SAP_ODATA_PROPERTIES: &str =
    "parse_sap_odata::property::{Property, property_ref::PropertyRef}";
pub static PATH_TO_SAP_SEMANTICS_PROPERTY: &str =
    "parse_sap_odata::sap_semantics::property::SAPSemanticsProperty";
pub static PATH_TO_SERDE_SERIALIZE_DESERIALIZE: &str = "serde::{Deserialize, Serialize}";
