use super::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// `from_str` implementation for each entity set struct
pub fn impl_from_str_for(struct_name: &str) -> Vec<u8> {
    static FN_START: &[u8] = "
  impl std::str::FromStr for "
        .as_bytes();
    static FN_END: &[u8] = " {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

"
    .as_bytes();

    [FN_START, struct_name.as_bytes(), FN_END].concat()
}

pub fn comment_for(something: &str) -> Vec<u8> {
    [SEPARATOR, COMMMENT_LINE, something.as_bytes(), LINE_FEED, SEPARATOR].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start a module declaration
pub fn gen_mod_start(mod_name: &str) -> Vec<u8> {
    [PUBLIC, MOD, mod_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start and end of a struct declaration
pub fn start_struct(struct_name: &str) -> Vec<u8> {
    [START_PUB_STRUCT, SPACE, struct_name.as_bytes(), OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_struct_field(field_name: &str, rust_type: &Vec<u8>) -> Vec<u8> {
    [PUBLIC, field_name.as_bytes(), COLON, rust_type, COMMA, LINE_FEED].concat()
}

pub fn end_iter_fn() -> Vec<u8> {
    [CLOSE_SQR, CALL_ITER, CALL_COPIED, LINE_FEED, END_BLOCK].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an implementation
pub fn gen_impl_start(some_name: &str) -> Vec<u8> {
    [IMPL, some_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Start of an enum declaration
pub fn gen_enum_start(enum_name: &str) -> Vec<u8> {
    [PUBLIC, ENUM, enum_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat()
}

pub fn gen_fq_enum_variant_name(enum_name: &str, variant_name: &str) -> Vec<u8> {
    [enum_name.as_bytes(), COLON2, variant_name.as_bytes()].concat()
}

pub fn gen_enum_variant(variant_name: &str) -> Vec<u8> {
    [variant_name.as_bytes(), COMMA, LINE_FEED].concat()
}

pub fn gen_fq_enum_variant(enum_name: &str, variant_name: &str) -> Vec<u8> {
    [&*gen_fq_enum_variant_name(&enum_name, &variant_name), COMMA, LINE_FEED].concat()
}

pub fn gen_enum_variant_names_fn(enum_name: &str) -> Vec<u8> {
    [VARIANT_NAMES_FN_START, enum_name.as_bytes(), VARIANT_NAMES_FN_END].concat()
}

pub fn gen_enum_variant_name_fn_start() -> Vec<u8> {
    [FN_VARIANT_NAME_DECL, MATCH_SELF].concat()
}

pub fn gen_enum_iter_fn_start(type_name: &str) -> Vec<u8> {
    [
        FN_ITERATOR_DECL_START,
        type_name.as_bytes(),
        FN_ITERATOR_DECL_END,
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
/// EntityType public trait declaration and implementation
pub fn gen_entity_type_trait_for(struct_name: &str) -> Vec<u8> {
    [
        RUSTC_ALLOW_DEAD_CODE,
        PUBLIC,
        SPACE,
        TRAIT,
        struct_name.as_bytes(),
        OPEN_CURLY,
        &gen_getter_fn_for_property_of_type(KEY, &*gen_vector_of(STATIC_STR.as_bytes())),
        SEMI_COLON,
        END_BLOCK,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_option_of_type(ty: &[u8]) -> Vec<u8> {
    [OPTION, OPEN_ANGLE, ty, CLOSE_ANGLE].concat()
}

pub fn gen_owned_string(s: &str) -> Vec<u8> {
    [DOUBLE_QUOTE, s.as_bytes(), DOUBLE_QUOTE, TO_OWNED].concat()
}

pub fn gen_bool_string(b: bool) -> Vec<u8> {
    bool::to_string(&b).into_bytes()
}

pub fn gen_vector_of(t: &[u8]) -> Vec<u8> {
    [VECTOR, crate::parser::syntax_fragments::CLOSE_ANGLE, t, CLOSE_ANGLE].concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_getter_fn_for_property_of_type(prop_name: &[u8], type_name: &[u8]) -> Vec<u8> {
    [
        crate::parser::syntax_fragments::GET_PREFIX,
        FN,
        GET_PREFIX,
        prop_name,
        UNIT,
        THIN_ARROW,
        type_name,
        OPEN_CURLY,
        LINE_FEED,
    ]
    .concat()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_opt_u16_string(int_arg: Option<u16>) -> Vec<u8> {
    if let Some(int) = int_arg {
        [SOME, OPEN_PAREN, int.to_string().as_bytes(), CLOSE_PAREN].concat()
    } else {
        NONE.to_vec()
    }
}
pub fn gen_opt_string(s_arg: &Option<String>) -> Vec<u8> {
    if let Some(s) = s_arg {
        [
            SOME,
            OPEN_PAREN,
            DOUBLE_QUOTE,
            s.as_bytes(),
            DOUBLE_QUOTE,
            TO_OWNED,
            CLOSE_PAREN,
        ]
        .concat()
    } else {
        NONE.to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn gen_type_name(odata_type_name: &str) -> String {
    convert_case::Casing::to_case(&odata_type_name, convert_case::Case::UpperCamel)
}
