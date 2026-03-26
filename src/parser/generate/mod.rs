pub mod metadata_doc;
pub mod srvc_doc;
pub mod syntax_fragments;

use crate::property::{metadata::PropertyType, Property};
use syntax_fragments::{
    serde_fragments::{gen_datetime_deserializer_fn, gen_decimal_deserializer_ref},
    *,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_struct_field_into(out: &mut Vec<u8>, field_name: &str, rust_type: &[u8]) {
    out.extend_from_slice(PUBLIC);
    out.extend_from_slice(field_name.as_bytes());
    out.extend_from_slice(COLON);
    out.extend_from_slice(rust_type);
    out.extend_from_slice(COMMA);
    out.extend_from_slice(LINE_FEED);
}

pub fn gen_enum_variant_into(out: &mut Vec<u8>, variant_name: &str) {
    out.extend_from_slice(variant_name.as_bytes());
    out.extend_from_slice(COMMA);
    out.extend_from_slice(LINE_FEED);
}

fn gen_fq_enum_variant_name_into(out: &mut Vec<u8>, enum_name: &str, variant_name: &str) {
    out.extend_from_slice(enum_name.as_bytes());
    out.extend_from_slice(COLON2);
    out.extend_from_slice(variant_name.as_bytes());
}

pub fn gen_fq_enum_variant_into(out: &mut Vec<u8>, enum_name: &str, variant_name: &str) {
    gen_fq_enum_variant_name_into(out, enum_name, variant_name);
    out.extend_from_slice(COMMA);
    out.extend_from_slice(LINE_FEED);
}

pub fn gen_enum_match_arm_into(out: &mut Vec<u8>, enum_name: &str, variant_name: &str, variant_value: &str) {
    gen_fq_enum_variant_name_into(out, enum_name, variant_name);
    out.extend_from_slice(SPACE);
    out.extend_from_slice(FAT_ARROW);
    out.extend_from_slice(SPACE);
    out.extend_from_slice(DOUBLE_QUOTE);
    out.extend_from_slice(variant_value.as_bytes());
    out.extend_from_slice(DOUBLE_QUOTE);
    out.extend_from_slice(COMMA);
    out.extend_from_slice(LINE_FEED);
}

pub fn gen_pub_getter_fn_of_type_into<T: std::fmt::Display>(
    out: &mut Vec<u8>,
    fn_name: &[u8],
    return_type: &[u8],
    some_type: T,
) {
    out.extend_from_slice(&gen_fn_signature(fn_name, true, false, None, Some(return_type)));
    out.extend_from_slice(OPEN_CURLY);
    out.extend_from_slice(LINE_FEED);

    let rendered = format!("{some_type}");
    out.extend_from_slice(rendered.as_bytes());

    out.extend_from_slice(END_BLOCK);
    out.extend_from_slice(LINE_FEED);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// `from_str` implementation for each entity set struct
pub fn gen_impl_from_str_for(struct_name: &str) -> Vec<u8> {
    static FN_START: &[u8] = "impl std::str::FromStr for ".as_bytes();
    static FN_END: &[u8] = " {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { quick_xml::de::from_str(s) }
}"
    .as_bytes();

    [LINE_FEED, FN_START, struct_name.as_bytes(), FN_END, LINE_FEED].concat()
}

pub fn gen_comment_separator_for(something: &str) -> Vec<u8> {
    [LINE_FEED, SEPARATOR, COMMENT_LINE, something.as_bytes(), SEPARATOR].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start a module declaration
pub fn gen_extern_crate(crate_name: &str) -> Vec<u8> {
    [EXTERN_CRATE, crate_name.as_bytes(), SEMI_COLON, LINE_FEED].concat()
}

pub fn gen_module_start(mod_name: &str) -> Vec<u8> {
    [PUBLIC, MOD, mod_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start and end of a struct declaration
static START_PUB_STRUCT: &[u8] = &"pub struct ".as_bytes();
pub fn gen_start_struct(struct_name: &str) -> Vec<u8> {
    [START_PUB_STRUCT, struct_name.as_bytes(), OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_struct_field(field_name: &str, rust_type: &[u8]) -> Vec<u8> {
    [PUBLIC, field_name.as_bytes(), COLON, rust_type, COMMA, LINE_FEED].concat()
}

static CALL_ITER: &[u8] = ".iter()".as_bytes();
static CALL_COPIED: &[u8] = ".copied()".as_bytes();
pub fn gen_end_iter_fn() -> Vec<u8> {
    [CLOSE_SQR, CALL_ITER, CALL_COPIED, LINE_FEED, END_BLOCK].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an implementation
pub fn gen_impl_start_for(some_name: &str) -> Vec<u8> {
    [IMPL, some_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate function signature
pub fn gen_fn_signature(
    fn_name: &[u8],
    is_pub: bool,
    is_const: bool,
    arg_list: Option<&[&[u8]]>,
    return_type: Option<&[u8]>,
) -> Vec<u8> {
    let public = if is_pub { PUBLIC } else { &[] };
    let constant = if is_const { CONST } else { &[] };

    let mut out = Vec::new();

    out.extend_from_slice(public);
    out.extend_from_slice(constant);
    out.extend_from_slice(FN);
    out.extend_from_slice(fn_name);
    out.extend_from_slice(OPEN_PAREN);

    if let Some(args) = arg_list {
        for arg in args {
            out.extend_from_slice(arg);
            out.extend_from_slice(COMMA);
        }
    }

    out.extend_from_slice(CLOSE_PAREN);

    if let Some(ret_type) = return_type {
        out.extend_from_slice(THIN_ARROW);
        out.extend_from_slice(ret_type);
    }

    out
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Output the start of the "variant_name" function within an enum implementation
//   pub const fn variant_name(&self) -> &'static str {↩︎
//       match *self {↩︎
pub fn gen_enum_impl_fn_variant_name() -> Vec<u8> {
    [
        &*gen_fn_signature(&FN_NAME_VARIANT_NAME, true, true, Some(&[SELF_REF]), Some(STATIC_STR_REF)),
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
    fn_name: &[u8],
    return_type: &[u8],
    some_type: T,
) -> Vec<u8> {
    [
        &*gen_fn_signature(fn_name, true, false, None, Some(return_type)),
        OPEN_CURLY,
        LINE_FEED,
        format!("{some_type}").as_bytes(),
        END_BLOCK,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an enum declaration
pub fn gen_enum_start(enum_name: &str) -> Vec<u8> {
    [PUBLIC, ENUM, enum_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_enum_variant(variant_name: &str) -> Vec<u8> {
    [variant_name.as_bytes(), COMMA, LINE_FEED].concat()
}

fn gen_fq_enum_variant_name(enum_name: &str, variant_name: &str) -> Vec<u8> {
    [enum_name.as_bytes(), COLON2, variant_name.as_bytes()].concat()
}

pub fn gen_fq_enum_variant(enum_name: &str, variant_name: &str) -> Vec<u8> {
    [&*gen_fq_enum_variant_name(&enum_name, &variant_name), COMMA, LINE_FEED].concat()
}

static VARIANT_NAMES_FN_START: &[u8] = "
pub fn variant_names() -> Vec<&'static str> {
"
.as_bytes();
static VARIANT_NAMES_FN_END: &[u8] = "::iterator().fold(Vec::new(), |mut acc: Vec<&'static str>, es| {
  acc.push(&mut es.variant_name());
  acc
})
}
"
.as_bytes();
pub fn gen_enum_fn_variant_names(enum_name: &str) -> Vec<u8> {
    [VARIANT_NAMES_FN_START, enum_name.as_bytes(), VARIANT_NAMES_FN_END].concat()
}

static FN_ITERATOR_DECL_START: &[u8] = "pub fn iterator() -> impl Iterator<Item = ".as_bytes();
pub fn gen_enum_fn_iter_start(type_name: &str) -> Vec<u8> {
    [
        FN_ITERATOR_DECL_START,
        type_name.as_bytes(),
        CLOSE_ANGLE,
        OPEN_CURLY,
        OPEN_SQR,
        LINE_FEED,
    ]
    .concat()
}

pub fn gen_enum_match_arm(enum_name: &str, variant_name: &str, variant_value: &str) -> Vec<u8> {
    [
        &*gen_fq_enum_variant_name(enum_name, variant_name),
        SPACE,
        FAT_ARROW,
        SPACE,
        DOUBLE_QUOTE,
        variant_value.as_bytes(),
        DOUBLE_QUOTE,
        COMMA,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn gen_of_type(t: &[u8]) -> Vec<u8> {
    [OPEN_ANGLE, t, CLOSE_ANGLE].concat()
}
pub fn gen_option_of_type(t: &[u8]) -> Vec<u8> {
    [OPTION, &*gen_of_type(t)].concat()
}
pub fn gen_vector_of_type(t: &[u8]) -> Vec<u8> {
    [VECTOR, &*gen_of_type(t)].concat()
}

static TO_OWNED: &[u8] = ".to_owned()".as_bytes();
pub fn gen_owned_string(s: &str) -> Vec<u8> {
    [DOUBLE_QUOTE, s.as_bytes(), DOUBLE_QUOTE, TO_OWNED].concat()
}
pub fn gen_bool_string(b: bool) -> Vec<u8> {
    bool::to_string(&b).into_bytes()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_some_value(val: &[u8]) -> Vec<u8> {
    [SOME, OPEN_PAREN, &*val, CLOSE_PAREN].concat()
}
pub fn gen_opt_u16_string(int_arg: Option<u16>) -> Vec<u8> {
    if let Some(int) = int_arg {
        let s = int.to_string();
        gen_some_value(s.as_bytes())
    } else {
        NONE.to_vec()
    }
}
pub fn gen_opt_string(s_arg: &Option<String>) -> Vec<u8> {
    if let Some(s) = s_arg {
        let owned = gen_owned_string(s);
        gen_some_value(&owned)
    } else {
        NONE.to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Returns possible forward reference to a custom deserializer function in the parse-sap-atom-feed crate
pub fn gen_custom_deserializer_info(prop: &Property) -> String {
    let empty_str = "".to_string();

    if let PropertyType::Edm(edm_type, _) = Property::get_property_type(&prop) {
        if edm_type.eq(EDMX_DATE_TIME) || edm_type.eq(EDMX_DATE_TIME_OFFSET) {
            gen_datetime_deserializer_fn(prop.nullable)
        } else if edm_type.eq(EDMX_DECIMAL) {
            gen_decimal_deserializer_ref(prop.nullable, prop.scale)
        } else {
            empty_str
        }
    } else {
        empty_str
    }
}
