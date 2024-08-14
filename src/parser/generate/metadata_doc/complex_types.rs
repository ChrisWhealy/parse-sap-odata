use check_keyword::CheckKeyword;

use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::generate::*,
    property::Property,
    utils::{odata_name_to_rust_safe_name, to_upper_camel_case},
};
use crate::parser::generate::syntax_fragments::{
    COLON, COMMA, END_BLOCK, LINE_FEED, METADATA, PROPERTY, PUBLIC, RUSTC_ALLOW_DEAD_CODE, SEPARATOR,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata complex type structs
pub fn gen_metadata_complex_types(cts: &Vec<ComplexType>) -> (Vec<u8>, Vec<String>) {
    let mut skipped_cts: Vec<String> = vec![];
    let mut ignored_cts: usize = 0;
    let mut out_buffer: Vec<u8> = gen_comment_separator_for("COMPLEX TYPES");

    for (idx, ct) in cts.into_iter().enumerate() {
        if idx > 0 && idx + ignored_cts + 1 < cts.len() {
            out_buffer.append(&mut SEPARATOR.to_vec());
        }

        // If the complex type contains only one field and that field's name suffix is a basic Rust type, then the
        // complex type can be replaced with a single variable of the Rust type.
        // This happens with SAP complex types such as `CT_String` which just contains a single field called `String`.
        let ct_name = to_upper_camel_case(&ct.name);

        if ct.properties.len() > 1 && !ct_name.is_keyword() {
            let ct_name = format!("{}{}", to_upper_camel_case(&ct.name), METADATA);
            let mut ct_props = ct.properties.clone();
            ct_props.sort();

            out_buffer.append(&mut gen_metadata_complex_type(&ct_name, &ct_props));
        } else {
            // This is just a simple type pretending to have a complex
            skipped_cts.push(ct.name.clone());
            ignored_cts += 1;
        }
    }

    (out_buffer, skipped_cts)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// ComplexType -> Rust metadata declaration
fn gen_metadata_complex_type(ct_name: &str, ct_props: &Vec<Property>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = [RUSTC_ALLOW_DEAD_CODE, LINE_FEED, &*gen_start_struct(&ct_name)].concat();

    for ct_prop in ct_props {
        out_buffer.append(
            &mut [
                PUBLIC,
                odata_name_to_rust_safe_name(&ct_prop.odata_name).as_bytes(),
                COLON,
                PROPERTY,
                COMMA,
            ]
            .concat(),
        );
    }

    out_buffer.append(&mut END_BLOCK.to_vec());
    out_buffer
}