use super::*;

mod tests {
    use super::*;
    use crate::schema::entity_set::EntitySet;

    fn show_metadata(entity_sets: &Vec<EntitySet>) {
        for set in entity_sets {
            println!("{:#?}", set);
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Danish government OData file
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // #[test]
    // pub fn test_parse_folketinget_metadata() {
    //     let edmx = Edmx::from_str(include_str!("../tests/folketinget.xml")).unwrap();
    //     let schema = edmx.default_schema().unwrap();

    //     show_metadata(schema.entity_sets().unwrap());

    //     assert_eq!(
    //         50,
    //         edmx.default_schema().unwrap().entity_sets().unwrap().len()
    //     );
    // }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Two entity types and an association
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn test_sap_basic() {
        let edmx = Edmx::from_str(include_str!("../tests/two_entity_types.xml")).unwrap();
        let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

        show_metadata(schema.entity_sets().unwrap())
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Full metadata doc minus complex types
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn test_sap_no_complex_types() {
        let edmx = Edmx::from_str(include_str!("../tests/no_complex_types.xml")).unwrap();
        let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

        show_metadata(schema.entity_sets().unwrap())
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Full metadata doc
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // #[test]
    // pub fn test_sap_full() {
    //     let edmx = Edmx::from_str(include_str!("../tests/sap_gwsample_basic_full.xml")).unwrap();
    //     let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

    //     show_metadata(schema.entity_sets().unwrap())
    // }
}
