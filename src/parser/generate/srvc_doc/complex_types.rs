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
pub fn gen_complex_types(cts: &Vec<ComplexType>) -> (Vec<u8>, Vec<String>) {
    let mut ignored_cts: usize = 0;

    cts.into_iter().enumerate().fold(
        // Accumulator's initial value is a comment separator
        (gen_comment_separator_for(COMPLEX_TYPES), vec![]),
        |(mut acc_src, mut acc_crate_refs), (idx, ct)| {
            if idx > 0 && idx + ignored_cts + 1 < cts.len() {
                acc_src.append(&mut SEPARATOR.to_vec());
            }

            if let (Some(mut ct_src), mut crs) = gen_complex_type_src(ct) {
                acc_crate_refs.append(&mut crs);
                acc_src.append(&mut ct_src);
            } else {
                ignored_cts += 1;
            }

            (acc_src, acc_crate_refs)
        },
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM Complex Type Instance -> Rust declaration
fn gen_complex_type_src(ct: &ComplexType) -> (Option<Vec<u8>>, Vec<String>) {
    let ct_name = to_upper_camel_case(&ct.name);

    // If the complex type contains only one field and that field's name suffix is a basic Rust type, then this complex
    // type can be replaced with a single variable of the corresponding Rust type.
    // This happens with SAP complex types such as `CT_String` which contains a single field called `String`.
    // A consequence of this approach is that any SAP annotations that might exist for this "simple" complex type will
    // not be captured by the corresponding Rust type
    if ct.properties.len() > 1 && !ct_name.is_keyword() {
        let mut crate_refs: Vec<String> = vec![];
        let mut props = ct.properties.clone();
        props.sort();

        let mut out_buffer: Vec<u8> = props.into_iter().fold(
            // The accumulator's initial value is the derive and serde attributes, plus the struct declaration
            gen_deserializable_struct(&ct_name),
            |mut acc, prop| {
                let (mut src, cr) = prop.to_rust();
                if !cr.is_empty() { crate_refs.push(cr); }
                acc.append(&mut src);
                acc
            },
        );

        out_buffer.append(&mut [END_BLOCK, &*gen_impl_from_str_for(&ct_name)].concat());

        (Some(out_buffer), crate_refs)
    } else {
        // This is just a simple type with a complex
        (None, vec![])
    }
}
