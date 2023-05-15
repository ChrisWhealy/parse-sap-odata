use super::*;

mod tests {
    use super::*;

    #[test]
    pub fn test_parse_folketinget_metadata() {
        let edmx = Edmx::from_str(include_str!("../tests/folketinget.xml")).unwrap();
        let schema = edmx.default_schema().unwrap();

        for entity_set in schema.entity_sets().unwrap() {
            println!("{:#?}", entity_set);
        }

        assert_eq!(
            50,
            edmx.default_schema().unwrap().entity_sets().unwrap().len()
        );
    }

    // #[test]
    // pub fn test_parse_sap_metadata() {
    //     let edmx = Edmx::from_str(include_str!("../tests/sap_gwsample_basic.xml")).unwrap();

    //     for set in edmx.default_schema().unwrap().entity_sets().unwrap() {
    //         println!("{:#?}", set);
    //     }
    // }
}
