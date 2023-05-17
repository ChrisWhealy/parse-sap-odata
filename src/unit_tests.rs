use super::*;

mod tests {
    use super::*;
    use crate::property::Property;
    use crate::schema::complex_type::ComplexType;
    use crate::schema::entity_set::EntitySet;
    use crate::schema::entity_type::EntityType;

    fn show_metadata(entity_sets: &Vec<EntitySet>) {
        for set in entity_sets {
            println!("{:#?}", set);
        }
    }

    fn show_properties(properties: &Vec<Property>) {
        for prop in properties {
            println!("Property {:#?} = {:#?}", prop.name, prop.property_type);
        }
    }

    fn show_entity_types(entity_types: &Vec<EntityType>) {
        for et in entity_types {
            println!("EntityType {:#?}, key {:#?}", et.name, et.key);
            show_properties(&et.properties)
        }
    }

    fn show_complex_types(complex_types: Option<Vec<ComplexType>>) {
        match complex_types {
            Some(cts) => {
                for ct in cts {
                    println!("Showing properties for {:?}", ct.name);
                    show_properties(&ct.properties)
                }
            }
            None => println!("No complex types defined"),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Two entity types and an association
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // #[test]
    // pub fn test_sap_basic() {
    //     let edmx = Edmx::from_str(include_str!("../tests/two_entity_types.xml")).unwrap();
    //     let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

    //     show_metadata(schema.entity_sets().unwrap())
    // }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Full metadata doc minus complex types
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // #[test]
    // pub fn test_sap_no_complex_types() {
    //     let edmx = Edmx::from_str(include_str!("../tests/no_complex_types.xml")).unwrap();
    //     let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

    //     show_metadata(schema.entity_sets().unwrap())
    // }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Full metadata doc
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn test_sap_full() {
        let edmx = Edmx::from_str(include_str!("../tests/sap_gwsample_basic_full.xml")).unwrap();
        let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

        println!("* * * * * * * * * * ENTITY TYPES * * * * * * * * * *");
        show_entity_types(&schema.entity_types);
        println!("* * * * * * * * * COMPLEX TYPES  * * * * * * * * * *");
        show_complex_types(schema.complex_types.clone());
    }
}
