use super::*;

mod tests {
    use super::*;
    use crate::schema::association::Association;
    use crate::schema::complex_type::ComplexType;
    use crate::schema::entity_type::EntityType;

    const SEPARATOR: &str =
        "* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *";

    fn centre(msg: &str, display_width: usize) -> String {
        let pad = if msg.len() >= display_width {
            0
        } else {
            msg.len() + (display_width / 2 - msg.len() / 2)
        };
        format!("{:>width$}", msg, width = pad)
    }

    fn print_banner(msg: &str) {
        println!("{}", SEPARATOR);
        println!("{}", centre(msg, SEPARATOR.len()));
        println!("{}", SEPARATOR);
    }

    fn show_entity_types(entity_types: &Vec<EntityType>) {
        print_banner("ENTITY TYPES");

        for et in entity_types {
            println!("{:#?}", et);
            // show_properties(&et.properties)
        }
    }

    fn show_complex_types(complex_types: Option<Vec<ComplexType>>) {
        print_banner("COMPEX TYPES");

        match complex_types {
            Some(cts) => {
                for ct in cts {
                    println!("{:#?}", ct);
                }
            }
            None => println!("No complex types defined"),
        }
    }

    fn show_associations(associations: &Vec<Association>) {
        print_banner("ASSOCIATIONS");

        for assoc in associations {
            println!("{:#?}", assoc);
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

        show_entity_types(&schema.entity_types);
        show_complex_types(schema.complex_types.clone());
        show_associations(&schema.associations);
    }
}
