pub mod metadata_doc;
pub mod srvc_doc;
pub mod syntax_fragments;

use crate::property::{edm_primitive::EdmPrimitive, edm_type::EdmType, Property};
use syntax_fragments::{
    serde_fragments::{gen_datetime_deserializer_fn, gen_decimal_deserializer_ref},
    *,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Utility functions that insert some fragment of source code into a mutable string
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_struct_field_into(out: &mut String, field_name: &str, rust_type: &str) {
    out.push_str(PUBLIC);
    out.push_str(field_name);
    out.push_str(COLON);
    out.push_str(rust_type);
    out.push_str(COMMA);
    out.push_str(LINE_FEED);
}

pub fn gen_enum_variant_into(out: &mut String, variant_name: &str) {
    out.push_str(variant_name);
    out.push_str(COMMA);
    out.push_str(LINE_FEED);
}

fn gen_fq_enum_variant_name_into(out: &mut String, enum_name: &str, variant_name: &str) {
    out.push_str(enum_name);
    out.push_str(DOUBLE_COLON);
    out.push_str(variant_name);
}

pub fn gen_fq_enum_variant_into(out: &mut String, enum_name: &str, variant_name: &str) {
    gen_fq_enum_variant_name_into(out, enum_name, variant_name);
    out.push_str(COMMA);
    out.push_str(LINE_FEED);
}

pub fn gen_enum_match_arm_into(out: &mut String, enum_name: &str, variant_name: &str, variant_value: &str) {
    gen_fq_enum_variant_name_into(out, enum_name, variant_name);
    out.push_str(SPACE);
    out.push_str(FAT_ARROW);
    out.push_str(SPACE);
    out.push_str(DOUBLE_QUOTE);
    out.push_str(variant_value);
    out.push_str(DOUBLE_QUOTE);
    out.push_str(COMMA);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// `from_str` implementation for each entity set struct
pub fn gen_impl_from_str_for_into(out: &mut String, struct_name: &str) {
    out.push_str("\nimpl std::str::FromStr for ");
    out.push_str(struct_name);
    out.push_str(" {\ntype Err = quick_xml::de::DeError;\nfn from_str(s: &str) -> Result<Self, Self::Err> { quick_xml::de::from_str(s) }\n}\n");
}

pub fn gen_comment_separator_for_into(out: &mut String, comment_text: &str) {
    out.push_str(LINE_FEED);
    out.push_str(SEPARATOR);
    out.push_str(COMMENT_LINE);
    out.push_str(comment_text);
    out.push_str(SEPARATOR);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start a module declaration
pub fn gen_extern_crate_into(out: &mut String, crate_name: &str) {
    out.push_str(EXTERN_CRATE);
    out.push_str(crate_name);
    out.push_str(SEMI_COLON);
    out.push_str(LINE_FEED);
}

pub fn gen_module_start_into(out: &mut String, mod_name: &str) {
    out.push_str(PUBLIC);
    out.push_str(MOD);
    out.push_str(mod_name);
    out.push_str(SPACE);
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start and end of a struct declaration
pub fn gen_start_struct_into(out: &mut String, struct_name: &str) {
    out.push_str("pub struct ");
    out.push_str(struct_name);
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
}

pub fn gen_end_iter_fn_into(out: &mut String) {
    out.push_str(CLOSE_SQR);
    out.push_str(".iter().copied()");
    out.push_str(LINE_FEED);
    out.push_str(END_BLOCK);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an implementation
pub fn gen_impl_start_for_into(out: &mut String, some_name: &str) {
    out.push_str(IMPL);
    out.push_str(some_name);
    out.push_str(SPACE);
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a function signature
pub fn gen_fn_signature_into(
    out: &mut String,
    fn_name: &str,
    is_pub: bool,
    is_const: bool,
    arg_list: Option<&[&str]>,
    return_type: Option<&str>,
) {
    let public = if is_pub { PUBLIC } else { "" };
    let constant = if is_const { CONST } else { "" };

    out.push_str(public);
    out.push_str(constant);
    out.push_str(FN);
    out.push_str(fn_name);
    out.push_str(OPEN_PAREN);

    if let Some(args) = arg_list {
        for arg in args {
            out.push_str(arg);
            out.push_str(COMMA);
        }
    }

    out.push_str(CLOSE_PAREN);

    if let Some(ret_type) = return_type {
        out.push_str(THIN_ARROW);
        out.push_str(ret_type);
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Output the start of the "variant_name" function within an enum implementation
//   pub const fn variant_name(&self) -> &'static str {↩︎
//       match *self {↩︎
pub fn gen_enum_impl_fn_variant_name_into(out: &mut String) {
    gen_fn_signature_into(out, FN_NAME_VARIANT_NAME, true, true, Some(&[SELF_REF]), Some(STATIC_STR_REF));
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
    out.push_str(MATCH_DEREF_SELF);
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a function that returns an instance of some type
pub fn gen_pub_getter_fn_of_type_into<T: std::fmt::Display>(
    out: &mut String,
    fn_name: &str,
    return_type: &str,
    some_type: T,
) {
    gen_fn_signature_into(out, fn_name, true, false, None, Some(return_type));
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
    out.push_str(&some_type.to_string());
    out.push_str(END_BLOCK);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an enum declaration
pub fn gen_enum_start_into(out: &mut String, enum_name: &str) {
    out.push_str(PUBLIC);
    out.push_str(ENUM);
    out.push_str(enum_name);
    out.push_str(SPACE);
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
}

pub fn gen_enum_fn_variant_names_into(out: &mut String, enum_variant_names: &str) {
    // Generate VARIANT_NAMES const
    out.push_str("\npub const VARIANT_NAMES: &[& str] = &[\n");
    out.push_str(enum_variant_names);
    out.push_str("];\n");

    out.push_str("pub fn variant_names() -> &'static [&'static str] {\n");
    out.push_str("Self::VARIANT_NAMES\n");
    out.push_str("}\n");
}

pub fn gen_enum_fn_iter_start_into(out: &mut String, type_name: &str) {
    out.push_str("pub fn iterator() -> impl Iterator<Item = ");
    out.push_str(type_name);
    out.push_str(CLOSE_ANGLE);
    out.push_str(OPEN_CURLY);
    out.push_str(OPEN_SQR);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_vector_of_type_into(out: &mut String, t: &str) {
    out.push_str(VECTOR);
    out.push_str(OPEN_ANGLE);
    out.push_str(t);
    out.push_str(CLOSE_ANGLE);
}
pub fn gen_vector_of_type_src(t: &str) -> String {
    let mut out = String::new();
    gen_vector_of_type_into(&mut out, t);
    out
}

static TO_OWNED: &str = ".to_owned()";
pub fn gen_owned_string_src(s: &str) -> String {
    let mut out = String::new();
    out.push_str(DOUBLE_QUOTE);
    out.push_str(s);
    out.push_str(DOUBLE_QUOTE);
    out.push_str(TO_OWNED);
    out
}
pub fn gen_bool_string(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_some_value_into(out: &mut String, val: &str) {
    out.push_str(SOME);
    out.push_str(OPEN_PAREN);
    out.push_str(val);
    out.push_str(CLOSE_PAREN);
}
pub fn gen_opt_u16_string_src(int_arg: Option<u16>) -> String {
    let mut out = String::new();
    match int_arg {
        Some(n) => {
            out.push_str(SOME);
            out.push_str(OPEN_PAREN);
            out.push_str(itoa::Buffer::new().format(n));
            out.push_str(CLOSE_PAREN);
        },
        None => out.push_str("None"),
    }
    out
}
pub fn gen_opt_string_src(s_arg: &Option<String>) -> String {
    let mut out = String::new();
    match s_arg {
        Some(s) => {
            out.push_str(SOME);
            out.push_str(OPEN_PAREN);
            out.push_str(DOUBLE_QUOTE);
            out.push_str(s);
            out.push_str(DOUBLE_QUOTE);
            out.push_str(TO_OWNED);
            out.push_str(CLOSE_PAREN);
        },
        None => out.push_str(NONE),
    }
    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Returns possible forward reference to a custom deserializer function in the parse-sap-atom-feed crate
pub fn gen_custom_deserializer_info(prop: &Property) -> String {
    if let EdmType::Primitive(prim) = &prop.edm_type {
        match prim {
            EdmPrimitive::DateTime | EdmPrimitive::DateTimeOffset => gen_datetime_deserializer_fn(prop.nullable),
            EdmPrimitive::Decimal => gen_decimal_deserializer_ref(prop.nullable, prop.scale),
            _ => String::new(),
        }
    } else {
        String::new()
    }
}
