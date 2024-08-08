use check_keyword::CheckKeyword;

use crate::{
    edmx::data_services::schema::complex_type::ComplexType,
    parser::{
        syntax_fragments::{
            derive_traits::*,
            fragment_generators::{comment_for, gen_start_struct, gen_type_name_upper_camel, impl_from_str_for},
            serde_fragments::*,
            END_BLOCK, SEPARATOR,
        },
        AsRustSrc,
    },
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Generate complex type structs
pub fn gen_complex_types(cts: &Vec<ComplexType>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let mut ignored_cts: usize = 0;

    out_buffer.append(&mut comment_for("COMPLEX TYPES"));

    for (idx, ct) in cts.into_iter().enumerate() {
        if idx > 0 && idx + ignored_cts + 1 < cts.len() {
            out_buffer.append(&mut SEPARATOR.to_vec());
        }

        if let Some(mut ct_src) = gen_complex_type_src(ct) {
            out_buffer.append(&mut ct_src);
        } else {
            ignored_cts += 1;
        }
    }

    out_buffer
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// EDM Complex Type Instance -> Rust declaration
fn gen_complex_type_src(ct: &ComplexType) -> Option<Vec<u8>> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let ct_name = gen_type_name_upper_camel(&ct.name);

    // If the complex type contains only one field and that field's name suffix is a basic Rust type, then it is
    // unnecessary to create a Rust struct as this complex type can be replaced with a single Rust type.
    // This happens with SAP complex types such as `CT_String` which just contains a single field called `String`.
    if ct.properties.len() > 1 && !ct_name.is_keyword() {
        let mut props = ct.properties.clone();
        props.sort();

        out_buffer.append(&mut derive_str(vec![
            DeriveTraits::CLONE,
            DeriveTraits::DEBUG,
            DeriveTraits::DEFAULT,
            DeriveTraits::SERIALIZE,
            DeriveTraits::DESERIALIZE,
        ]));
        out_buffer.append(&mut SERDE_RENAME_ALL_PASCAL_CASE.to_vec());
        out_buffer.append(&mut gen_start_struct(&ct_name));

        for prop in props {
            out_buffer.append(&mut prop.to_rust());
        }

        out_buffer.append(&mut END_BLOCK.to_vec());

        // Implement `from_str` for this struct
        out_buffer.append(&mut impl_from_str_for(&ct_name));
        Some(out_buffer)
    } else {
        // This is just a simple type pretending to have a complex
        None
    }
}
