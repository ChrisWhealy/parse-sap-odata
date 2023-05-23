mod tests {
    use std::cmp::max;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, Read, Write};
    use std::str::FromStr;

    use crate::edmx::Edmx;
    use crate::utils::parse_error::ParseError;

    fn longest(m: &HashMap<&str, &str>) -> usize {
        m.iter().fold(0, |max_len, e| max(max_len, e.0.len()))
    }

    fn gen_service_list() -> (HashMap<&'static str, &'static str>, usize) {
        let mut service_list: HashMap<&str, &str> = HashMap::new();

        service_list.insert("catalogservice", "CATALOGSERVICE");
        service_list.insert("catalogservice_v0002", "CATALOGSERVICE");
        service_list.insert("epm_ref_apps_shop_srv", "EPM_REF_APPS_SHOP");
        service_list.insert("epm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV");
        service_list.insert("epm_ref_apps_prod_man_srv", "EPM_REF_APPS_PROD_MAN_SRV");
        service_list.insert("error_log_srv", "ERROR_LOG_SRV");
        service_list.insert("gwdemo", "GWDEMO");
        service_list.insert("gwsample_basic", "GWSAMPLE_BASIC");
        service_list.insert("interop", "INTEROP");
        service_list.insert("page_builder_conf", "PAGE_BUILDER_CONF");
        service_list.insert("page_builder_cust", "PAGE_BUILDER_CUST");
        service_list.insert("page_builder_pers", "PAGE_BUILDER_PERS");
        service_list.insert("rmtsampleflight", "RMTSAMPLEFLIGHT");
        service_list.insert("sepmra_gr_post", "SEPMRA_GR_POST");
        service_list.insert("sepmra_ovw", "SEPMRA_OVW");
        service_list.insert("sepmra_po_apv", "EPM_REF_APPS_PROD_MAN_SRV");
        service_list.insert("sepmra_po_man", "SEPMRA_PO_MAN");
        service_list.insert("sepmra_prod_man", "SEPMRA_PROD_MAN");
        service_list.insert("sepmra_shop", "SEPMRA_SHOP");
        service_list.insert("sepmra_so_man", "SEPMRA_SO_MAN");
        service_list.insert("sgbt_nte_cds_api_d_srv", "SGBT_NTE_CDS_API_D_SRV");
        service_list.insert("sgbt_nte_cds_api_srv", "SGBT_NTE_CDS_API_SRV");
        service_list.insert("transport", "TRANSPORT");
        service_list.insert("z_test_cds_with_param_srv", "Z_TEST_CDS_WITH_PARAM_SRV");
        service_list.insert("zagencycds_srv", "ZAGENCYCDS_SRV");
        service_list.insert("zdevelopercenter", "ZDEVELOPERCENTER");
        service_list.insert("zepm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV");
        service_list.insert("ze2e100_sol_2_srv", "ZE2E100_SOL_2_SRV");
        service_list.insert("zepm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV");
        service_list.insert("zrfc1_srv", "ZRFC1_SRV");
        service_list.insert("zpdcds_srv", "ZPDCDS_SRV");
        service_list.insert("zsocds_srv", "ZSOCDS_SRV");

        let max_length = longest(&service_list);

        (service_list, max_length)
    }

    fn write_entity<T: Debug>(out_buf: &mut Vec<u8>, maybe_entity: Option<&Vec<T>>) {
        match maybe_entity {
            Some(entity) => {
                if entity.len() > 0 {
                    for e in entity {
                        out_buf.append(&mut format!("{:#?}\n", e).as_bytes().to_vec());
                    }
                    out_buf.append(&mut vec![10]); // Add line feed
                }
            }
            None => {}
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a given metadata document
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    fn parse_sap_metadata(metadata_file_name: &str, namespace: &str) -> Result<usize, ParseError> {
        let mut xml_buffer: Vec<u8> = Vec::new();
        let mut out_buffer: Vec<u8> = Vec::new();

        let xml_input_path = format!("./tests/{}.xml", metadata_file_name);

        let mut out_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("./parsed/{}.txt", metadata_file_name))?;

        let f_xml = File::open(&xml_input_path)?;
        let _file_size = BufReader::new(f_xml).read_to_end(&mut xml_buffer);
        let xml = String::from_utf8(xml_buffer)?;
        let edmx = Edmx::from_str(&xml)?;
        let Some(_schema) = edmx.data_services.fetch_schema(namespace) else {
            return Err(ParseError { msg : format!("Namespace {} not found in schema", namespace)});
        };

        write_entity(&mut out_buffer, Some(&vec![edmx]));

        // write_entity(&mut out_buffer, Some(&schema.entity_types));
        // write_entity(&mut out_buffer, schema.complex_types.as_ref());
        // write_entity(&mut out_buffer, Some(&schema.associations));

        // if let Some(entity_container) = &schema.entity_container {
        //     write_entity(&mut out_buffer, Some(&entity_container.entity_sets));
        //     write_entity(&mut out_buffer, Some(&entity_container.association_sets));
        //     write_entity(&mut out_buffer, entity_container.function_imports.as_ref());
        // }

        // write_entity(&mut out_buffer, schema.annotation_list.as_ref());
        // write_entity(&mut out_buffer, Some(&schema.atom_links));

        out_file.write_all(&out_buffer).unwrap();

        return Ok(out_buffer.len());
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse all the working OData services listed on the ES5 SAP Dev Center OData server
    // https://sapes5.sapdevcenter.com/sap/opu/odata/iwfnd/CATALOGSERVICE/CatalogCollection('ES5')/Services
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn parse_all() {
        let (srv_list, max_length) = gen_service_list();

        for srv in srv_list {
            print!(
                "Parsing {}.xml{:>width$}=> ",
                srv.0,
                " ",
                width = max_length - srv.0.len() + 1
            );

            match parse_sap_metadata(srv.0, srv.1) {
                Err(err) => println!("Error: {}", err.msg),
                Ok(bytes) => println!("{} bytes written", bytes),
            };
        }
    }
}
