// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Single characters
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub static LINE_FEED: &[u8] = &[0x0a];
pub static SPACE: &[u8] = &[0x20];
pub static DOUBLE_QUOTE: &[u8] = &[0x22];
pub static COMMA: &[u8] = &[0x2C];
pub static COLON: &[u8] = &[0x3A];
pub static OPEN_SQR: &[u8] = &[0x5B];
pub static CLOSE_SQR: &[u8] = &[0x5D];
pub static OPEN_CURLY: &[u8] = &[0x7B];
pub static CLOSE_CURLY: &[u8] = &[0x7D];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Syntactical separators
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub static COLON2: &[u8] = &[0x3A, 0x3A];
pub static FAT_ARROW: &[u8] = &[0x3D, 0x3E];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Type names
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub static BOOLEAN: &[u8] = "bool".as_bytes();
pub static F32: &[u8] = "f32".as_bytes();
pub static F64: &[u8] = "f64".as_bytes();
pub static I8: &[u8] = "i8".as_bytes();
pub static I16: &[u8] = "i16".as_bytes();
pub static I32: &[u8] = "i32".as_bytes();
pub static I64: &[u8] = "i64".as_bytes();
pub static U8: &[u8] = "u8".as_bytes();
pub static STRING: &[u8] = "String".as_bytes();
pub static VECTOR_U8: &[u8] = "Vec<u8>".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Keywords or keyword fragments
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub static OPTION_DECLARATION: &[u8] = "Option<".as_bytes();
pub static TYPE_TERMINATOR: &[u8] = ">".as_bytes();
pub static PUBLIC: &[u8] = "pub".as_bytes();
pub static START_ENUM: &[u8] = "pub enum ".as_bytes();
pub static START_IMPL: &[u8] = "impl ".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// External types
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub static DECIMAL: &[u8] = "rust_decimal::Decimal".as_bytes();
pub static NAIVE_DATE_TIME: &[u8] = "chrono::NaiveDateTime".as_bytes();
pub static STD_TIME_SYSTEMTIME: &[u8] = "std::time::SystemTime".as_bytes();
pub static UUID: &[u8] = "uuid::Uuid".as_bytes();

pub static DERIVE_COPY_CLONE_DEBUG: &[u8] = "#[derive(Copy, Clone, Debug)]".as_bytes();
pub static DERIVE_CLONE_DEBUG_DEFAULT: &[u8] = &"#[derive(Clone, Debug, Default)]".as_bytes();
pub static FN_VALUE_DECL: &[u8] = "pub const fn value(&self) -> &'static str {".as_bytes();
pub static FN_ITERATOR_DECL_START: &[u8] = "pub fn iterator() -> impl Iterator<Item = ".as_bytes();
pub static FN_ITERATOR_DECL_END: &[u8] = "> {".as_bytes();
pub static MATCH_SELF: &[u8] = "match *self {".as_bytes();
pub static CALL_ITER: &[u8] = ".iter()".as_bytes();
pub static CALL_COPIED: &[u8] = ".copied()".as_bytes();
pub static AS_OPT_LIST_START: &[u8] = "
pub fn as_list() -> Vec<&'static str> {
  let mut list = "
    .as_bytes();
pub static AS_OPT_LIST_END: &[u8] = "::iterator().fold(Vec::new(), |mut acc: Vec<&'static str>, es| {
      acc.insert(0, &mut es.value());
      acc
  });
  list.reverse();
  list
}
"
.as_bytes();

pub static START_PUB_STRUCT: &[u8] = &"pub struct ".as_bytes();
