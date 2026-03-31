pub mod metadata_doc;
pub mod srvc_doc;
pub mod syntax_fragments;

use crate::property::{metadata::PropertyType, Property};
use syntax_fragments::{
    serde_fragments::{gen_datetime_deserializer_fn, gen_decimal_deserializer_ref},
    *,
};

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
    out.push_str(COLON2);
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

pub fn gen_pub_getter_fn_of_type_into<T: std::fmt::Display>(
    out: &mut String,
    fn_name: &str,
    return_type: &str,
    some_type: T,
) {
    out.push_str(&gen_fn_signature(fn_name, true, false, None, Some(return_type)));
    out.push_str(OPEN_CURLY);
    out.push_str(LINE_FEED);
    out.push_str(&format!("{some_type}"));
    out.push_str(END_BLOCK);
    out.push_str(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// `from_str` implementation for each entity set struct
pub fn gen_impl_from_str_for(struct_name: &str) -> String {
    static FN_START: &str = "impl std::str::FromStr for ";
    static FN_END: &str = " {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { quick_xml::de::from_str(s) }
}";

    [LINE_FEED, FN_START, struct_name, FN_END, LINE_FEED].concat()
}

pub fn gen_comment_separator_for(something: &str) -> String {
    [LINE_FEED, SEPARATOR, COMMENT_LINE, something, SEPARATOR].concat()
}

pub fn gen_comment_separator_for_into(out: &mut String, something: &str) {
    out.push_str(LINE_FEED);
    out.push_str(SEPARATOR);
    out.push_str(COMMENT_LINE);
    out.push_str(something);
    out.push_str(SEPARATOR);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start a module declaration
pub fn gen_extern_crate(crate_name: &str) -> String {
    [EXTERN_CRATE, crate_name, SEMI_COLON, LINE_FEED].concat()
}

pub fn gen_extern_crate_into(out: &mut String, crate_name: &str) {
    out.push_str(EXTERN_CRATE);
    out.push_str(crate_name);
    out.push_str(SEMI_COLON);
    out.push_str(LINE_FEED);
}

pub fn gen_module_start(mod_name: &str) -> String {
    [PUBLIC, MOD, mod_name, SPACE, OPEN_CURLY, LINE_FEED].concat()
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
static START_PUB_STRUCT: &str = "pub struct ";
pub fn gen_start_struct(struct_name: &str) -> String {
    [START_PUB_STRUCT, struct_name, OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_struct_field(field_name: &str, rust_type: &str) -> String {
    [PUBLIC, field_name, COLON, rust_type, COMMA, LINE_FEED].concat()
}

static CALL_ITER: &str = ".iter()";
static CALL_COPIED: &str = ".copied()";
pub fn gen_end_iter_fn() -> String {
    [CLOSE_SQR, CALL_ITER, CALL_COPIED, LINE_FEED, END_BLOCK].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an implementation
pub fn gen_impl_start_for(some_name: &str) -> String {
    [IMPL, some_name, SPACE, OPEN_CURLY, LINE_FEED].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate function signature
pub fn gen_fn_signature(
    fn_name: &str,
    is_pub: bool,
    is_const: bool,
    arg_list: Option<&[&str]>,
    return_type: Option<&str>,
) -> String {
    let public = if is_pub { PUBLIC } else { "" };
    let constant = if is_const { CONST } else { "" };

    let mut out = String::new();

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

    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Output the start of the "variant_name" function within an enum implementation
//   pub const fn variant_name(&self) -> &'static str {↩︎
//       match *self {↩︎
pub fn gen_enum_impl_fn_variant_name() -> String {
    [
        &gen_fn_signature(FN_NAME_VARIANT_NAME, true, true, Some(&[SELF_REF]), Some(STATIC_STR_REF)),
        OPEN_CURLY,
        LINE_FEED,
        MATCH_SELF,
        OPEN_CURLY,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate a function that returns an instance of some type
pub fn gen_pub_getter_fn_of_type<T: std::fmt::Display>(
    fn_name: &str,
    return_type: &str,
    some_type: T,
) -> String {
    [
        &gen_fn_signature(fn_name, true, false, None, Some(return_type)),
        OPEN_CURLY,
        LINE_FEED,
        &format!("{some_type}"),
        END_BLOCK,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an enum declaration
pub fn gen_enum_start(enum_name: &str) -> String {
    [PUBLIC, ENUM, enum_name, SPACE, OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_enum_variant(variant_name: &str) -> String {
    [variant_name, COMMA, LINE_FEED].concat()
}

fn gen_fq_enum_variant_name(enum_name: &str, variant_name: &str) -> String {
    [enum_name, COLON2, variant_name].concat()
}

pub fn gen_fq_enum_variant(enum_name: &str, variant_name: &str) -> String {
    [&gen_fq_enum_variant_name(enum_name, variant_name), COMMA, LINE_FEED].concat()
}

static VARIANT_NAMES_FN_START: &str = "
pub fn variant_names() -> Vec<&'static str> {
";
static VARIANT_NAMES_FN_END: &str = "::iterator().fold(Vec::new(), |mut acc: Vec<&'static str>, es| {
  acc.push(&mut es.variant_name());
  acc
})
}
";
pub fn gen_enum_fn_variant_names(enum_name: &str) -> String {
    [VARIANT_NAMES_FN_START, enum_name, VARIANT_NAMES_FN_END].concat()
}

static FN_ITERATOR_DECL_START: &str = "pub fn iterator() -> impl Iterator<Item = ";
pub fn gen_enum_fn_iter_start(type_name: &str) -> String {
    [
        FN_ITERATOR_DECL_START,
        type_name,
        CLOSE_ANGLE,
        OPEN_CURLY,
        OPEN_SQR,
        LINE_FEED,
    ]
    .concat()
}

pub fn gen_enum_match_arm(enum_name: &str, variant_name: &str, variant_value: &str) -> String {
    [
        &gen_fq_enum_variant_name(enum_name, variant_name),
        SPACE,
        FAT_ARROW,
        SPACE,
        DOUBLE_QUOTE,
        variant_value,
        DOUBLE_QUOTE,
        COMMA,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn gen_of_type(t: &str) -> String {
    [OPEN_ANGLE, t, CLOSE_ANGLE].concat()
}
pub fn gen_option_of_type(t: &str) -> String {
    [OPTION, &gen_of_type(t)].concat()
}
pub fn gen_vector_of_type(t: &str) -> String {
    [VECTOR, &gen_of_type(t)].concat()
}

static TO_OWNED: &str = ".to_owned()";
pub fn gen_owned_string(s: &str) -> String {
    [DOUBLE_QUOTE, s, DOUBLE_QUOTE, TO_OWNED].concat()
}
pub fn gen_bool_string(b: bool) -> String {
    b.to_string()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_some_value(val: &str) -> String {
    [SOME, OPEN_PAREN, val, CLOSE_PAREN].concat()
}
pub fn gen_opt_u16_string(int_arg: Option<u16>) -> String {
    if let Some(int) = int_arg {
        gen_some_value(&int.to_string())
    } else {
        NONE.to_string()
    }
}
pub fn gen_opt_string(s_arg: &Option<String>) -> String {
    if let Some(s) = s_arg {
        gen_some_value(&gen_owned_string(s))
    } else {
        NONE.to_string()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Returns possible forward reference to a custom deserializer function in the parse-sap-atom-feed crate
pub fn gen_custom_deserializer_info(prop: &Property) -> String {
    if let PropertyType::Edm(edm_type, _) = Property::get_property_type(&prop) {
        if edm_type.eq(EDMX_DATE_TIME) || edm_type.eq(EDMX_DATE_TIME_OFFSET) {
            gen_datetime_deserializer_fn(prop.nullable)
        } else if edm_type.eq(EDMX_DECIMAL) {
            gen_decimal_deserializer_ref(prop.nullable, prop.scale)
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}
