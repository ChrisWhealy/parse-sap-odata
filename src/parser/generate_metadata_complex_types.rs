use check_keyword::CheckKeyword;

use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::syntax_fragments::{
        fragment_generators::{comment_for, gen_type_name, start_struct},
        COLON, COMMA, END_BLOCK, METADATA, PROPERTY, PUBLIC, RUSTC_ALLOW_DEAD_CODE, SEPARATOR,
    },
    property::Property,
    utils::odata_name_to_rust_safe_name,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate metadata complex type structs
pub fn gen_metadata_complex_types(out_buffer: &mut Vec<u8>, cts: &Vec<ComplexType>) -> Vec<String> {
    let mut skipped_cts: Vec<String> = vec![];
    let mut ignored_cts: usize = 0;

    out_buffer.append(&mut comment_for("COMPLEX TYPES"));

    for (idx, ct) in cts.into_iter().enumerate() {
        if idx > 0 && idx + ignored_cts + 1 < cts.len() {
            out_buffer.append(&mut SEPARATOR.to_vec());
        }

        // If the complex type contains only one field and that field's name suffix is a basic Rust type, then it is
        // unnecessary to represent the complex type as a Rust struct as it can be replaced with a single Rust type.
        // This happens with SAP complex types such as `CT_String` which just contains a single field called `String`.
        let ct_name = gen_type_name(&ct.name);

        if ct.properties.len() > 1 && !ct_name.is_keyword() {
            let ct_name = format!("{}{}", gen_type_name(&ct.name), METADATA);
            let mut ct_props = ct.properties.clone();
            ct_props.sort();

            out_buffer.append(&mut gen_metadata_complex_type(&ct_name, &ct_props));
        } else {
            // This is just a simple type pretending to have a complex
            skipped_cts.push(ct.name.clone());
            ignored_cts += 1;
        }
    }

    skipped_cts
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// ComplexType -> Rust metadata declaration
fn gen_metadata_complex_type(ct_name: &str, ct_props: &Vec<Property>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();

    out_buffer.append(&mut RUSTC_ALLOW_DEAD_CODE.to_vec());
    out_buffer.append(&mut start_struct(&ct_name));

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
