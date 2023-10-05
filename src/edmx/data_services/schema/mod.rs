pub mod association;
pub mod complex_type;
pub mod entity_container;
pub mod entity_type;

#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::*;

use crate::{
    atom::AtomLink,
    oasis::annotations::Annotations,
    sap_annotations::SAPAnnotationsSchema,
    xml::{default_xml_language, default_xml_namespace},
};

use association::Association;
use complex_type::ComplexType;
use entity_container::EntityContainer;
use entity_type::EntityType;

use serde::{Deserialize, Serialize};

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
        let pascal_entity_types_name = format!(
            "{}EntityTypes",
            convert_case::Casing::to_case(&self.namespace, convert_case::Case::Pascal)
        );

        // Output the start of an enum that collates all the entity type names
        // #[derive(Debug)]↩︎
        // pub enum <schema_namespace>EntityTypes {↩︎
        let mut output_enum = derive_str(vec![DeriveTraits::DEBUG]).as_slice().to_vec();
        output_enum.append(&mut gen_enum_start(&pascal_entity_types_name));

        // Output the start of a "value" function within the enum implementation
        //   pub const fn value(&self) -> &'static str {↩︎
        //       match *self {↩︎
        let mut fn_value = gen_enum_value_fn_start();

        // Create entity type enum
        for ent_type in self.entity_types.iter() {
            let ent_type_name_camel = convert_case::Casing::to_case(&ent_type.name, convert_case::Case::UpperCamel);

            // Add variant to enum and value function
            output_enum.append(&mut gen_enum_variant(&ent_type_name_camel));
            fn_value.append(&mut gen_enum_match_arm(
                &pascal_entity_types_name,
                &ent_type_name_camel,
                &ent_type.name,
            ));
        }

        output_enum.append(&mut end_block());
        fn_value.append(&mut end_block());
        fn_value.append(&mut end_block());

        return [output_enum, gen_impl_start(&pascal_entity_types_name), fn_value, end_block()].concat();
    }
}
