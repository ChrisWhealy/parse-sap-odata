mod tests {
    use crate::edmx::data_services::schema::entity_container::EntityContainer;
    use crate::edmx::Edmx;
    use std::fmt::Debug;
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, Read, Write};
    use std::str::FromStr;

    // ASCII value of "* * * * * "
    const SEPARATOR: [u8; 80] = [
        42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42,
        32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32,
        42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 32, 42,
        32, 42, 32, 42, 32, 42, 32, 42, 32, 42, 10,
    ];

    fn centre(msg: &str, display_width: usize) -> String {
        let pad = if msg.len() >= display_width {
            0
        } else {
            msg.len() + (display_width / 2 - msg.len() / 2)
        };
        format!("{:>width$}", msg, width = pad)
    }

    fn write_banner(out_buff: &mut Vec<u8>, msg: &str) {
        out_buff.append(&mut SEPARATOR.to_vec());
        out_buff.append(
            &mut format!("* {}\n", centre(msg, SEPARATOR.len() - 2))
                .as_bytes()
                .to_vec(),
        );
        out_buff.append(&mut SEPARATOR.to_vec());
    }

    fn show_entity<T: Debug>(out_buf: &mut Vec<u8>, entity: &Vec<T>, entity_name: &str) {
        write_banner(out_buf, entity_name);

        if entity.len() > 0 {
            for e in entity {
                out_buf.append(&mut format!("{:#?}\n", e).as_bytes().to_vec());
            }
        } else {
            out_buf.append(
                &mut format!("No {} defined\n", entity_name.to_lowercase())
                    .as_bytes()
                    .to_vec(),
            )
        }
        out_buf.append(&mut vec![10]);
    }

    fn show_optional_entity<T: Debug>(
        out_buf: &mut Vec<u8>,
        maybe_entity: &Option<Vec<T>>,
        entity_name: &str,
    ) {
        match maybe_entity {
            Some(entity) => show_entity(out_buf, entity, entity_name),
            None => {
                write_banner(out_buf, entity_name);
                out_buf.append(
                    &mut format!("No {} defined\n", entity_name.to_lowercase())
                        .as_bytes()
                        .to_vec(),
                )
            }
        }
    }

    fn show_entity_container(
        out_buf: &mut Vec<u8>,
        maybe_entity_container: &Option<EntityContainer>,
    ) {
        match maybe_entity_container {
            Some(entity_container) => {
                show_entity(
                    out_buf,
                    &entity_container.entity_sets,
                    "ENTITY CONTAINER: ENTITY SETS",
                );

                show_entity(
                    out_buf,
                    &entity_container.association_sets,
                    "ENTITY CONTAINER: ASSOCIATION SETS",
                );

                show_optional_entity(
                    out_buf,
                    &entity_container.function_imports,
                    "ENTITY CONTAINER: FUNCTION IMPORTS",
                );
            }
            None => out_buf.append(
                &mut format!("This schema does not have an EntityContainer\n")
                    .as_bytes()
                    .to_vec(),
            ),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a given metadata document
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    fn parse_sap_metadata(metadata_file_name: &str, namespace: &str) {
        let mut xml_buffer: Vec<u8> = Vec::new();
        let mut out_buffer: Vec<u8> = Vec::new();

        let xml_input_path = format!("./tests/{}.xml", metadata_file_name);
        let mut out_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("./parsed/{}.txt", metadata_file_name))
            .unwrap();

        if let Ok(f_xml) = File::open(&xml_input_path) {
            println!("Parsing file {}", xml_input_path);

            let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);

            if let Ok(xml) = String::from_utf8(xml_buffer) {
                let edmx = Edmx::from_str(&xml).unwrap();

                if let Some(schema) = edmx.data_services.fetch_schema(namespace) {
                    show_entity(&mut out_buffer, &schema.entity_types, "ENTITY TYPES");
                    show_optional_entity(&mut out_buffer, &schema.complex_types, "COMPLEX TYPES");
                    show_entity(&mut out_buffer, &schema.associations, "ASSOCIATIONS");
                    show_entity_container(&mut out_buffer, &schema.entity_container);
                    show_optional_entity(&mut out_buffer, &schema.annotation_list, "ANNOTATIONS");
                    show_entity(&mut out_buffer, &schema.atom_links, "ATOM LINKS");
                } else {
                    out_buffer.append(
                        &mut format!(
                            "ERROR: Schema does not contain the namespace '{}'\n",
                            namespace
                        )
                        .as_bytes()
                        .to_vec(),
                    )
                }
            } else {
                out_buffer.append(
                    &mut "ERROR: XML file is not in UTF8 format!\n"
                        .as_bytes()
                        .to_vec(),
                )
            }
        } else {
            out_buffer.append(
                &mut format!("ERROR: Input file {} not found\n", xml_input_path)
                    .as_bytes()
                    .to_vec(),
            )
        }

        if out_buffer.len() > 0 {
            out_file.write_all(&out_buffer).unwrap();
        } else {
            out_file.write("Parsing failed\n".as_bytes()).unwrap();
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a variety of OData metadata documents
    // All the OData service name listed below are taken from the SAP Dev Center OData server
    // https://sapes5.sapdevcenter.com/sap/opu/odata/iwfnd/CATALOGSERVICE/CatalogCollection('ES5')/Services
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn parse_all() {
        parse_sap_metadata("catalogservice", "CATALOGSERVICE");
        parse_sap_metadata("catalogservice_v0002", "CATALOGSERVICE");
        parse_sap_metadata("epm_ref_apps_shop_srv", "EPM_REF_APPS_SHOP");
        parse_sap_metadata("epm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV");
        parse_sap_metadata("epm_ref_apps_prod_man_srv", "EPM_REF_APPS_PROD_MAN_SRV");
        parse_sap_metadata("error_log_srv", "ERROR_LOG_SRV");
        parse_sap_metadata("gwdemo", "GWDEMO");
        parse_sap_metadata("gwsample_basic", "GWSAMPLE_BASIC");
        parse_sap_metadata("interop", "INTEROP");
        parse_sap_metadata("page_builder_conf", "PAGE_BUILDER_CONF");
        parse_sap_metadata("page_builder_cust", "PAGE_BUILDER_CUST");
        parse_sap_metadata("page_builder_pers", "PAGE_BUILDER_PERS");
        parse_sap_metadata("rmtsampleflight", "RMTSAMPLEFLIGHT");
        parse_sap_metadata("sepmra_gr_post", "SEPMRA_GR_POST");
        parse_sap_metadata("sepmra_ovw", "SEPMRA_OVW");
        parse_sap_metadata("sepmra_po_apv", "EPM_REF_APPS_PROD_MAN_SRV");
        parse_sap_metadata("sepmra_po_man", "SEPMRA_PO_MAN");
        parse_sap_metadata("sepmra_prod_man", "SEPMRA_PROD_MAN");
        parse_sap_metadata("sepmra_shop", "SEPMRA_SHOP");
        parse_sap_metadata("sepmra_so_man", "SEPMRA_SO_MAN");
        parse_sap_metadata("sgbt_nte_cds_api_d_srv", "SGBT_NTE_CDS_API_D_SRV");
        parse_sap_metadata("sgbt_nte_cds_api_srv", "SGBT_NTE_CDS_API_SRV");
        parse_sap_metadata("transport", "TRANSPORT");
        parse_sap_metadata("z_test_cds_with_param_srv", "Z_TEST_CDS_WITH_PARAM_SRV");
        parse_sap_metadata("zagencycds_srv", "ZAGENCYCDS_SRV");
        parse_sap_metadata("zdevelopercenter", "ZDEVELOPERCENTER");
        parse_sap_metadata("zepm_ref_apps_po_apv_srv_srv", "EPM_REF_APPS_PO_APV_SRV");
        parse_sap_metadata("ze2e100_sol_2_srv", "ZE2E100_SOL_2_SRV");
        parse_sap_metadata("zepm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV");
        parse_sap_metadata("zrfc1_srv", "ZRFC1_SRV");
        parse_sap_metadata("zpdcds_srv", "ZPDCDS_SRV");
        parse_sap_metadata("zsocds_srv", "ZSOCDS_SRV");
    }
}
