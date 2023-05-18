use super::*;

mod tests {
    use super::*;
    use crate::schema::entity_container::EntityContainer;
    use std::fmt::Debug;

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
        println!("Showing {} entries", entity.len());

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
    // Full metadata doc
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn test_sap_full() {
        let edmx = Edmx::from_str(include_str!("../tests/sap_gwsample_basic_full.xml")).unwrap();
        let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

        show_entity(&schema.entity_types, "ENTITY TYPES");
        show_optional_entity(&schema.complex_types, "COMPLEX TYPES");
        show_entity(&schema.associations, "ASSOCIATIONS");
        show_entity_container(&schema.entity_container);
        // show_entity(&schema.atom_links, "ATOM LINKS");
    }
}
