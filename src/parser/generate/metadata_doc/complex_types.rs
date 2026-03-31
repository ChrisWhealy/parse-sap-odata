use check_keyword::CheckKeyword;

use crate::parser::generate::syntax_fragments::{
    END_BLOCK, METADATA, PROPERTY, RUSTC_ALLOW_DEAD_CODE, SEPARATOR,
};
use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::generate::*,
    property::Property,
    utils::{odata_name_to_rust_safe_name, to_upper_camel_case},
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata complex type structs, writing output into `out` and returning skipped type names
pub fn gen_metadata_complex_types_into(out: &mut String, cts: &[ComplexType]) -> Vec<String> {
    let (src, skipped_cts) = gen_metadata_complex_types(cts);
    out.push_str(&src);
    skipped_cts
}

pub fn gen_metadata_complex_types(cts: &[ComplexType]) -> (String, Vec<String>) {
    let mut skipped_cts: Vec<String> = vec![];
    let mut ignored_cts: usize = 0;

    let out_buffer: String =
        cts.into_iter()
            .enumerate()
            .fold(gen_comment_separator_for("COMPLEX TYPES"), |mut acc, (idx, ct)| {
                if idx > 0 && idx + ignored_cts + 1 < cts.len() {
                    acc.push_str(SEPARATOR);
                }

                // If the complex type contains only one field and that field's name suffix is a basic Rust type, then
                // the complex type can be replaced with a single variable of the Rust type.
                // This happens with certain SAP complex types such as `CT_String` which just contains a single field
                // called `String`.
                let ct_name = to_upper_camel_case(&ct.name);

                if ct.properties.len() > 1 && !ct_name.is_keyword() {
                    let ct_name = format!("{}{}", to_upper_camel_case(&ct.name), METADATA);
                    let mut ct_props: Vec<_> = ct.properties.iter().collect();
                    ct_props.sort();

                    acc.push_str(&gen_metadata_complex_type(&ct_name, &ct_props));
                } else {
                    // This is just a simple type pretending to have a complex
                    skipped_cts.push(ct.name.clone());
                    ignored_cts += 1;
                }

                acc
            });

    (out_buffer, skipped_cts)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// ComplexType -> Rust metadata declaration
fn gen_metadata_complex_type(ct_name: &str, ct_props: &[&Property]) -> String {
    let mut out_buffer = String::new();
    out_buffer.push_str(RUSTC_ALLOW_DEAD_CODE);
    out_buffer.push_str(&gen_start_struct(ct_name));

    for ct_prop in ct_props {
        gen_struct_field_into(&mut out_buffer, &odata_name_to_rust_safe_name(&ct_prop.odata_name), PROPERTY);
    }

    out_buffer.push_str(END_BLOCK);
    out_buffer
}
