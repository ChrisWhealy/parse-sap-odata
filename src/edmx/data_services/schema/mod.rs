pub mod association;
pub mod complex_type;
pub mod entity_container;
pub mod entity_type;

use crate::{
    atom::AtomLink,
    oasis::annotations::Annotations,
    parser::syntax_fragments::*,
    sap_annotations::default_sap_schema_version,
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
    #[serde(rename = "xmlns", default = "default_xml_namespace")]
    pub xml_namespace: String,

    #[serde(rename = "Namespace", default)]
    pub namespace: String,

    #[serde(rename = "xml:lang", default = "default_xml_language")]
    pub xml_lang: String,

    #[serde(rename = "sap:schema_version", default = "default_sap_schema_version")]
    pub sap_schema_version: String,

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

impl Schema {
    pub fn to_entity_types_enum(&self) -> Vec<u8> {
        let pascal_entity_types_name = format!(
            "{}EntityTypes",
            convert_case::Casing::to_case(&self.namespace, convert_case::Case::Pascal)
        );

        // Output the start of an enum for all the entity types
        // #[derive(Debug)]↩︎
        // pub enum <schema_namespace>EntityTypes {↩︎
        let mut output_enum = [
            derive_str(vec![DeriveTraits::DEBUG]).as_slice(),
            START_ENUM,
            pascal_entity_types_name.as_bytes(),
            SPACE,
            OPEN_CURLY,
            LINE_FEED,
        ]
        .concat();

        // Output the start of an enum implementation
        // impl <schema_namespace>EntityTypes {↩︎
        let output_impl = [START_IMPL, pascal_entity_types_name.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat();

        // Output the start of a "value" function within the enum implementation
        //   pub const fn value(&self) -> &'static str {↩︎
        //       match *self {↩︎
        let mut fn_value = [FN_VALUE_DECL, LINE_FEED, MATCH_SELF, LINE_FEED].concat();

        // Create entity type enum
        for ent_type in self.entity_types.iter() {
            let ent_type_name_camel = convert_case::Casing::to_case(&ent_type.name, convert_case::Case::UpperCamel);

            // Add variant to enum
            output_enum.append(&mut [ent_type_name_camel.as_bytes(), COMMA, LINE_FEED].concat());

            // Add variant to value function
            fn_value.append(
                &mut [
                    pascal_entity_types_name.as_bytes(),
                    COLON2,
                    ent_type_name_camel.as_bytes(),
                    SPACE,
                    FAT_ARROW,
                    SPACE,
                    DOUBLE_QUOTE,
                    &ent_type.name.as_bytes(),
                    DOUBLE_QUOTE,
                    COMMA,
                    LINE_FEED,
                ]
                .concat(),
            );
        }

        output_enum.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, LINE_FEED].concat());
        fn_value.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, CLOSE_CURLY, LINE_FEED].concat());

        return [
            output_enum,
            output_impl,
            fn_value,
            CLOSE_CURLY.to_vec(),
            LINE_FEED.to_vec(),
            LINE_FEED.to_vec(),
        ]
        .concat();
    }
}
