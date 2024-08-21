use check_keyword::CheckKeyword;

use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::{
        generate::{
            gen_comment_separator_for, gen_impl_from_str_for,
            syntax_fragments::{serde_fragments::*, COMPLEX_TYPES, END_BLOCK, SEPARATOR},
        },
        AsRustSrc,
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate complex type structs
pub fn gen_complex_types(cts: &Vec<ComplexType>) -> Vec<u8> {
    let mut ignored_cts: usize = 0;

    cts.into_iter()
        .enumerate()
        .fold(gen_comment_separator_for(COMPLEX_TYPES), |mut acc, (idx, ct)| {
            if idx > 0 && idx + ignored_cts + 1 < cts.len() {
                acc.append(&mut SEPARATOR.to_vec());
            }

            if let Some(mut ct_src) = gen_complex_type_src(ct) {
                acc.append(&mut ct_src);
            } else {
                ignored_cts += 1;
            }

            acc
        })
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM Complex Type Instance -> Rust declaration
fn gen_complex_type_src(ct: &ComplexType) -> Option<Vec<u8>> {
    let ct_name = to_upper_camel_case(&ct.name);

    // If the complex type contains only one field and that field's name suffix is a basic Rust type, then the
    // complex type can be replaced with a single variable of the Rust type.
    // This happens with SAP complex types such as `CT_String` which just contains a single field called `String`.
    if ct.properties.len() > 1 && !ct_name.is_keyword() {
        let mut out_buffer: Vec<u8> = gen_deserializable_struct(&ct_name);
        let mut props = ct.properties.clone();

        props.sort();
        let _ = props.into_iter().map(|prop| out_buffer.append(&mut prop.to_rust()));

        out_buffer.append(&mut END_BLOCK.to_vec());

        // Implement `from_str` for this struct
        out_buffer.append(&mut gen_impl_from_str_for(&ct_name));
        Some(out_buffer)
    } else {
        // This is just a simple type pretending to have a complex
        None
    }
}
