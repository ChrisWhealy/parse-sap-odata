use std::fmt::{Debug, Display};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn handle_test_bool(received_bool: bool) -> Result<(), String> {
    if received_bool {
        Ok(())
    } else {
        Err("Expected Boolean True.  Got Boolean False instead".to_string())
    }
}

pub fn handle_test_comparison<T>(received_val: &T, expected_val: &T) -> Result<(), String>
where
    T: PartialEq + Clone + Debug + Display,
{
    if received_val.eq(expected_val) {
        Ok(())
    } else {
        Err(format!("Expected '{}', got '{}' instead", expected_val, received_val))
    }
}

pub fn handle_test_comparison_opt<T>(received_val: &Option<T>, expected_val: &Option<T>) -> Result<(), String>
where
    T: PartialEq + Clone + Debug + Display,
{
    if received_val.is_none() && expected_val.is_none() {
        Ok(())
    } else if received_val.is_some() && expected_val.is_some() {
        handle_test_comparison(&received_val.clone().unwrap(), &expected_val.clone().unwrap())
    } else {
        Err(format!("Can't compare {:?} with {:?}.", received_val, expected_val))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Syn-based helpers for validating generated Rust struct field source code

/// Parse a single struct field from generated source, wrapping it in a throwaway struct so that
/// syn can give useful span information if the parse fails.
pub fn parse_field(field_src: &str) -> Result<syn::Field, String> {
    let src = format!("struct Test {{ {field_src} }}");
    let item: syn::ItemStruct = syn::parse_str(&src)
        .map_err(|e| format!("Generated field is not valid Rust:\n{field_src}\n\nError: {e}"))?;
    let syn::Fields::Named(ref fields) = item.fields else {
        return Err("Expected named fields".to_string());
    };
    fields
        .named
        .first()
        .ok_or_else(|| "Expected at least one field".to_string())
        .cloned()
}

/// Return the name of the outermost type in a field (the last path segment, or `()` for unit).
pub fn outer_type_name(field: &syn::Field) -> Result<String, String> {
    match &field.ty {
        syn::Type::Path(tp) => tp
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .ok_or_else(|| "Type path has no segments".to_string()),
        syn::Type::Tuple(tt) if tt.elems.is_empty() => Ok("()".to_string()),
        _ => Err("Unexpected field type (not a path or unit tuple)".to_string()),
    }
}

pub fn has_serde_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|a| a.path().is_ident("serde"))
}

/// Parse a block of generated Rust source as a complete file.  The source is included in the
/// error message so a failing test immediately shows what was generated.
pub fn parse_module(src: &str) -> Result<syn::File, String> {
    syn::parse_str(src).map_err(|e| format!("Generated code is not valid Rust:\n{src}\n\nError: {e}"))
}

/// Locate a named struct inside a parsed file, returning an error if it is absent.
pub fn find_struct<'a>(file: &'a syn::File, name: &str) -> Result<&'a syn::ItemStruct, String> {
    file.items
        .iter()
        .find_map(|item| {
            if let syn::Item::Struct(s) = item {
                if s.ident == name { Some(s) } else { None }
            } else {
                None
            }
        })
        .ok_or_else(|| format!("No struct named '{name}' found in generated code"))
}

/// Extract all field names from a named struct.
pub fn struct_field_names(item_struct: &syn::ItemStruct) -> Vec<String> {
    match &item_struct.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .map(|i| i.to_string())
            .collect(),
        _ => vec![],
    }
}
