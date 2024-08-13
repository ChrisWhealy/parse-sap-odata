use serde::{Deserialize, Serialize};

use association::Association;
use complex_type::ComplexType;
use entity_container::EntityContainer;
use entity_type::EntityType;

use crate::{
    atom::AtomLink,
    oasis::annotations::Annotations,
    sap_annotations::schema::SAPAnnotationsSchema,
    xml::{default_xml_language, default_xml_namespace},
};

#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::{
    derive_traits::*,
    fragment_generators::{
        gen_enum_impl_fn_variant_name, gen_enum_match_arm, gen_enum_start, gen_enum_variant, gen_impl_start
    },
    *,
};
use crate::utils::to_upper_camel_case;

pub mod association;
pub mod complex_type;
pub mod entity_container;
pub mod entity_type;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Schema>` tag
///
/// # Child Nodes
///
/// `1:n EntityType`<br>
/// `0:n Association`<br>
/// `0:n ComplexType`<br>
/// `1:1 EntityContainer`
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "@xmlns", default = "default_xml_namespace")]
    pub xml_namespace: String,
    #[serde(rename = "@Namespace", default)]
    pub namespace: String,
    #[serde(rename = "@xml:lang", default = "default_xml_language")]
    pub xml_lang: String,
    #[serde(flatten)]
    pub sap_annotations: SAPAnnotationsSchema,
    #[serde(rename = "EntityType", default)]
    pub entity_types: Vec<EntityType>,
    #[serde(rename = "ComplexType", default)]
    pub complex_types: Option<Vec<ComplexType>>,
    #[serde(rename = "Association", default)]
    pub associations: Vec<Association>,
    #[serde(rename = "EntityContainer", default)]
    pub entity_container: Option<EntityContainer>,
    #[serde(rename = "Annotations", default)]
    pub annotation_list: Option<Vec<Annotations>>,
    // Appears in the original XML as the tagname "atom:link"
    #[serde(rename = "link")]
    pub atom_links: Vec<AtomLink>,
}

#[cfg(feature = "parser")]
impl Schema {
    pub fn to_entity_types_enum(&self) -> Vec<u8> {
        let upper_camel_entity_types = format!("{}EntityTypes", to_upper_camel_case(&self.namespace));

        // Output the start of an enum that collates all the entity type names
        // #[derive(Debug)]↩︎
        // pub enum <schema_namespace>EntityTypes {↩︎
        let mut output_enum = derive_str(vec![DeriveTraits::DEBUG]);
        output_enum.append(&mut gen_enum_start(&upper_camel_entity_types));

        // Output the start of the "variant_name" function within the enum implementation
        let mut fn_variant_name = gen_enum_impl_fn_variant_name();

        // Create entity type enum
        for ent_type in self.entity_types.iter() {
            let ent_type_name_camel = to_upper_camel_case(&ent_type.name);

            // Add variant to enum and value function
            output_enum.append(&mut gen_enum_variant(&ent_type_name_camel));
            fn_variant_name.append(&mut gen_enum_match_arm(
                &upper_camel_entity_types,
                &ent_type_name_camel,
                &ent_type.name,
            ));
        }

        output_enum.append(&mut END_BLOCK.to_vec());
        fn_variant_name.append(&mut [END_BLOCK, END_BLOCK].concat());

        [
            &*output_enum,
            &*gen_impl_start(&upper_camel_entity_types),
            &*fn_variant_name,
            END_BLOCK,
        ]
        .concat()
    }
}
