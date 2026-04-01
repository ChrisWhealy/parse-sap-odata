use std::fmt::Formatter;

use crate::{
    edmx::data_services::schema::association::referential_constraint::dependent::Dependent,
    parser::generate::{
        gen_owned_string_src,
        syntax_fragments::{CLOSE_CURLY, CLOSE_SQR, COLON, COMMA, LINE_FEED, OPEN_CURLY, VEC_BANG},
    },
};

static MY_NAME: &str = "Dependent";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum DependentFieldNames {
    Role,
    PropertyRefs,
}

impl DependentFieldNames {
    fn value(prop_name: DependentFieldNames) -> &'static str {
        match prop_name {
            DependentFieldNames::Role => "role",
            DependentFieldNames::PropertyRefs => "property_refs",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_from_dependent(f: &mut Formatter<'_>, prop_md: DependentFieldNames, val: &str) -> std::fmt::Result {
    write!(
        f,
        "{}{}{}{}{}",
        DependentFieldNames::value(prop_md),
        COLON,
        val,
        COMMA,
        LINE_FEED
    )
}

impl std::fmt::Display for Dependent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut prop_refs_str = String::new();
        prop_refs_str.push_str(VEC_BANG);

        for pr in self.property_refs.iter() {
            prop_refs_str.push_str(&pr.to_string());
        }

        prop_refs_str.push_str(CLOSE_SQR);

        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_from_dependent(f, DependentFieldNames::Role, &gen_owned_string_src(&self.role))?;
        line_from_dependent(f, DependentFieldNames::PropertyRefs, &prop_refs_str)?;
        write!(f, "{CLOSE_CURLY}")
    }
}
