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
        println!("* {}", centre(msg, SEPARATOR.len() - 2));
        println!("{}", SEPARATOR);
    }

    fn show_entity<T: Debug>(entity: &Vec<T>, entity_name: &str) {
        print_banner(entity_name);

        if entity.len() > 0 {
            for e in entity {
                println!("{:#?}", e);
            }
        } else {
            println!("No {} defined", entity_name.to_lowercase())
        }
    }

    fn show_optional_entity<T: Debug>(maybe_entity: &Option<Vec<T>>, entity_name: &str) {
        match maybe_entity {
            Some(entity) => show_entity(entity, entity_name),
            None => {
                print_banner(entity_name);
                println!("No {} defined", entity_name.to_lowercase())
            }
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

        if let Ok(f_xml) = File::open(&xml_path) {
            let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);

            print_banner(&format!("Parsing file {}", xml_path));

            if let Ok(xml) = String::from_utf8(xml_buffer) {
                let edmx = Edmx::from_str(&xml).unwrap();

                if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                    show_entity(&schema.entity_types, "ENTITY TYPES");
                    show_optional_entity(&schema.complex_types, "COMPLEX TYPES");
                    show_entity(&schema.associations, "ASSOCIATIONS");
                    show_entity_container(&schema.entity_container);
                    show_optional_entity(&schema.annotation_list, "ANNOTATIONS");
                    show_entity(&schema.atom_links, "ATOM LINKS");
                } else {
                    println!("ERROR: No schema for namespace {} found", namespace)
                }
            } else {
                println!("ERROR: XML file is not in UTF8 format!")
            }
        } else {
            println!("ERROR: File {} not found", xml_path)
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a variety of OData metadata documents
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn test_parser() {
        // parse_sap_metadata("catalogservice.xml", "CATALOGSERVICE");
        // parse_sap_metadata("catalogservice_v0002.xml", "CATALOGSERVICE");
        // parse_sap_metadata("epm_ref_apps_shop_srv.xml", "EPM_REF_APPS_SHOP");
        // parse_sap_metadata("epm_ref_apps_po_apv_srv.xml", "EPM_REF_APPS_PO_APV_SRV");
        // parse_sap_metadata("epm_ref_apps_prod_man_srv.xml", "EPM_REF_APPS_PROD_MAN_SRV");
        // parse_sap_metadata("error_log_srv.xml", "ERROR_LOG_SRV");
        // parse_sap_metadata("gwdemo.xml", "GWDEMO");
        // parse_sap_metadata("gwsample_basic.xml", "GWSAMPLE_BASIC");
        // parse_sap_metadata("interop.xml", "INTEROP");
        // parse_sap_metadata("page_builder_conf.xml", "PAGE_BUILDER_CONF");
        // parse_sap_metadata("page_builder_cust.xml", "PAGE_BUILDER_CUST");
        parse_sap_metadata("page_builder_pers.xml1", "PAGE_BUILDER_PERS");
        // parse_sap_metadata("rmtsampleflight.xml", "RMTSAMPLEFLIGHT");
        // parse_sap_metadata("sepmra_gr_post.xml", "SEPMRA_GR_POST");
        // parse_sap_metadata("sepmra_ovw.xml", "SEPMRA_OVW");
        // parse_sap_metadata("sepmra_po_apv.xml", "EPM_REF_APPS_PROD_MAN_SRV");
        // parse_sap_metadata("sepmra_po_man.xml", "SEPMRA_PO_MAN");
        // parse_sap_metadata("sepmra_prod_man.xml", "SEPMRA_PROD_MAN");
        // parse_sap_metadata("sepmra_shop.xml", "SEPMRA_SHOP");
        // parse_sap_metadata("sepmra_so_man.xml", "SEPMRA_SO_MAN");
        // parse_sap_metadata("sgbt_nte_cds_api_d_srv.xml", "SGBT_NTE_CDS_API_D_SRV");
        // parse_sap_metadata("sgbt_nte_cds_api_srv.xml", "SGBT_NTE_CDS_API_SRV");
        // parse_sap_metadata("transport.xml", "TRANSPORT");
        // parse_sap_metadata("z_test_cds_with_param_srv.xml", "Z_TEST_CDS_WITH_PARAM_SRV");
        // parse_sap_metadata("zagencycds_srv.xml", "ZAGENCYCDS_SRV");
        // parse_sap_metadata("zdevelopercenter.xml", "ZDEVELOPERCENTER");
        // parse_sap_metadata(
        //     "zepm_ref_apps_po_apv_srv_srv.xml",
        //     "EPM_REF_APPS_PO_APV_SRV",
        // );
        // parse_sap_metadata("ze2e100_sol_2_srv.xml", "ZE2E100_SOL_2_SRV");
        // parse_sap_metadata("zepm_ref_apps_po_apv_srv.xml", "EPM_REF_APPS_PO_APV_SRV");
        // parse_sap_metadata("zrfc1_srv.xml", "ZRFC1_SRV");
        // parse_sap_metadata("zpdcds_srv.xml", "ZPDCDS_SRV");
        // parse_sap_metadata("zsocds_srv.xml", "ZSOCDS_SRV");
    }
}
