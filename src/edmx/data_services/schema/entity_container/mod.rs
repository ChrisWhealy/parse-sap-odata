pub mod association_set;
pub mod entity_set;
pub mod function_import;

use crate::ms_annotations::MSAnnotationsEntityType;
use association_set::AssociationSet;
use serde::{Deserialize, Serialize};

use entity_set::EntitySet;
use function_import::FunctionImport;

static LINE_FEED: &[u8] = &[0x0A];
static SPACE: &[u8] = &[0x20];
static DOUBLE_QUOTE: &[u8] = &[0x22];
static COMMA: &[u8] = &[0x2C];
static COLON2: &[u8] = &[0x3A, 0x3A];
static FAT_ARROW: &[u8] = &[0x3D, 0x3E];
static OPEN_CURLY: &[u8] = &[0x7B];
static CLOSE_CURLY: &[u8] = &[0x7D];

static START_ENUM: &[u8] = "pub enum ".as_bytes();
static START_IMPL: &[u8] = "impl ".as_bytes();
static FN_DECL: &[u8] = "pub const fn value(&self) -> &'static str {".as_bytes();
static MATCH_SELF: &[u8] = "match *self {".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// EntityContainer
//
// Child Nodes:
//   1:n EntitySet
//   1:n AssociationSet
//   0:n FunctionImport
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityContainer {
    pub name: String,

    #[serde(flatten)]
    pub ms_annotations: MSAnnotationsEntityType,

    #[serde(rename = "sap:supported-formats")]
    pub sap_supported_formats: Option<String>,

    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,

    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,

    #[serde(rename = "FunctionImport", default)]
    pub function_imports: Option<Vec<FunctionImport>>,
}

impl EntityContainer {
    pub fn to_enum_with_impl(&self) -> Vec<u8> {
        let cont_name_camel = convert_case::Casing::to_case(&self.name, convert_case::Case::UpperCamel);

        // Output the start of an enum for this entity container
        // enum <entity_container_name> {↩︎
        let mut output_enum = [START_ENUM, cont_name_camel.as_bytes(), SPACE, OPEN_CURLY, LINE_FEED].concat();

        // Output the start of a function implementation called "value" for this enum
        // impl <entity_container_name> {↩︎
        //     pub const fn value(&self) -> &'static str {↩︎
        //         match *self {↩︎
        let mut output_impl = [
            START_IMPL,
            cont_name_camel.as_bytes(),
            SPACE,
            OPEN_CURLY,
            LINE_FEED,
            FN_DECL,
            LINE_FEED,
            MATCH_SELF,
            LINE_FEED,
        ]
        .concat();

        // Create entity set enum
        for ent_set in self.entity_sets.iter() {
            let ent_set_name_camel = convert_case::Casing::to_case(&ent_set.name, convert_case::Case::UpperCamel);

            // Add enum member
            output_enum.append(&mut [ent_set_name_camel.as_bytes(), COMMA, LINE_FEED].concat());

            // Add function impl member value
            output_impl.append(
                &mut [
                    cont_name_camel.as_bytes(),
                    COLON2,
                    ent_set_name_camel.as_bytes(),
                    SPACE,
                    FAT_ARROW,
                    SPACE,
                    DOUBLE_QUOTE,
                    &ent_set.name.as_bytes(),
                    DOUBLE_QUOTE,
                    COMMA,
                    LINE_FEED,
                ]
                .concat(),
            );
        }

        output_enum.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, LINE_FEED].concat());
        output_impl.append(&mut [LINE_FEED, CLOSE_CURLY, LINE_FEED, CLOSE_CURLY, LINE_FEED, CLOSE_CURLY].concat());

        return [output_enum, output_impl].concat();
    }
}
