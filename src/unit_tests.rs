mod tests {
    use crate::edmx::data_services::schema::entity_container::EntityContainer;
    use crate::edmx::Edmx;
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::str::FromStr;

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

    fn show_entity<T: Debug>(entity: &Vec<T>, entity_name: &str) {
        print_banner(entity_name);

        for e in entity {
            println!("{:#?}", e);
        }
    }

    fn show_optional_entity<T: Debug>(maybe_entity: &Option<Vec<T>>, entity_name: &str) {
        match maybe_entity {
            Some(entity) => show_entity(entity, entity_name),
            None => println!("No {} defined", entity_name.to_lowercase()),
        }
    }

    fn show_entity_container(maybe_entity_container: &Option<EntityContainer>) {
        match maybe_entity_container {
            Some(entity_container) => {
                show_entity(
                    &entity_container.entity_sets,
                    "ENTITY CONTAINER: ENTITY SETS",
                );

                show_entity(
                    &entity_container.association_sets,
                    "ENTITY CONTAINER: ASSOCIATION SETS",
                );

                show_optional_entity(
                    &entity_container.function_imports,
                    "ENTITY CONTAINER: FUNCTION IMPORTS",
                );
            }
            None => println!("This schema does not have an EntityContainer"),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a given metadata document
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    fn parse_sap_metadata(metadata_file_name: &str, namespace: &str) {
        let mut xml_buffer: Vec<u8> = Vec::new();
        let xml_path = format!("./tests/{}", metadata_file_name);
        let f_xml = File::open(&xml_path).expect(&format!("File {} not found", xml_path));
        let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);

        if let Ok(xml) = String::from_utf8(xml_buffer) {
            let edmx = Edmx::from_str(&xml).unwrap();
            let schema = edmx.data_services.fetch_schema(namespace).unwrap();

            show_entity(&schema.entity_types, "ENTITY TYPES");
            show_optional_entity(&schema.complex_types, "COMPLEX TYPES");
            show_entity(&schema.associations, "ASSOCIATIONS");
            show_entity_container(&schema.entity_container);
            show_optional_entity(&schema.annotation_list, "ANNOTATIONS");
            show_entity(&schema.atom_links, "ATOM LINKS");
        } else {
            println!("ERROR: XML file is not in UTF8 format!")
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a variety of OData metadata documents
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn test_parser() {
        // parse_sap_metadata("gwsample_basic.xml", "GWSAMPLE_BASIC");
        // parse_sap_metadata("sepmra_gr_post.xml", "SEPMRA_GR_POST");
        // parse_sap_metadata("sepmra_prod_man.xml", "SEPMRA_PROD_MAN");
        // parse_sap_metadata("epm_ref_apps_shop_srv.xml", "EPM_REF_APPS_SHOP");
        parse_sap_metadata("zdevelopercenter.xml", "ZDEVELOPERCENTER");
    }
}
