use check_keyword::CheckKeyword;

use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::{
        generate::{
            gen_comment_separator_for_into, gen_impl_from_str_for_into,
            syntax_fragments::{serde_fragments::*, COMPLEX_TYPES, END_BLOCK, SEPARATOR},
        },
        AsRustSrc,
    },
    utils::to_upper_camel_case,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate complex type structs, writing output into `out` and returning crate references
pub fn gen_complex_types_into(out: &mut String, cts: &[ComplexType]) -> Vec<String> {
    let (src, crs) = gen_complex_types(cts);
    out.push_str(&src);
    crs
}

pub fn gen_complex_types(cts: &[ComplexType]) -> (String, Vec<String>) {
    let mut ignored_cts: usize = 0;
    let mut acc_src = String::new();
    let mut acc_crate_refs: Vec<String> = Vec::new();

    // Start the source code with a comment separator line
    gen_comment_separator_for_into(&mut acc_src, COMPLEX_TYPES);

    for (idx, ct) in cts.iter().enumerate() {
        if idx > 0 && idx + ignored_cts + 1 < cts.len() {
            acc_src.push_str(SEPARATOR);
        }

        if let (Some(ct_src), mut crs) = gen_complex_type_src(ct) {
            acc_crate_refs.append(&mut crs);
            acc_src.push_str(&ct_src);
        } else {
            ignored_cts += 1;
        }
    }

    (acc_src, acc_crate_refs)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM Complex Type Instance -> Rust declaration
fn gen_complex_type_src(ct: &ComplexType) -> (Option<String>, Vec<String>) {
    let ct_name = to_upper_camel_case(&ct.name);

    // If the complex type contains only one field and that field's name suffix is a basic Rust type, then this complex
    // type can be replaced with a single variable of the corresponding Rust type.
    // This happens with SAP complex types such as `CT_String` which contains a single field called `String`.
    // A consequence of this approach is that any SAP annotations that might exist for this "simple" complex type will
    // not be captured by the corresponding Rust type
    if ct.properties.len() > 1 && !ct_name.is_keyword() {
        let mut crate_refs: Vec<String> = vec![];
        let mut props: Vec<_> = ct.properties.iter().collect();
        props.sort();

        let mut out: String = props.into_iter().fold(
            // The accumulator's initial value is the derive and serde attributes, plus the struct declaration
            gen_deserializable_struct(&ct_name),
            |mut acc, prop| {
                let (src, cr) = prop.to_rust();
                if !cr.is_empty() {
                    crate_refs.push(cr);
                }

                acc.push_str(&src);
                acc
            },
        );

        out.push_str(END_BLOCK);
        gen_impl_from_str_for_into(&mut out, &ct_name);
        (Some(out), crate_refs)
    } else {
        // This is just a simple type with a complex
        (None, vec![])
    }
}
