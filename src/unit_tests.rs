mod tests {
    use std::fmt::Debug;
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, Read, Write};
    use std::str::FromStr;

    use crate::edmx::Edmx;
    use crate::utils::parse_error::ParseError;

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
        let Some(schema) = edmx.data_services.fetch_schema(namespace) else {
            return Err(ParseError { msg : format!("Namespace {} not found in schema", namespace)});
        };

        write_entity(&mut out_buffer, Some(&schema.entity_types));
        write_entity(&mut out_buffer, schema.complex_types.as_ref());
        write_entity(&mut out_buffer, Some(&schema.associations));

        match &schema.entity_container {
            Some(entity_container) => {
                write_entity(&mut out_buffer, Some(&entity_container.entity_sets));
                write_entity(&mut out_buffer, Some(&entity_container.association_sets));
                write_entity(&mut out_buffer, entity_container.function_imports.as_ref());
            }
            None => {}
        }

        write_entity(&mut out_buffer, schema.annotation_list.as_ref());
        write_entity(&mut out_buffer, Some(&schema.atom_links));

        if out_buffer.len() > 0 {
            out_file.write_all(&out_buffer).unwrap();
        }

        return Ok(out_buffer.len());
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse a variety of OData metadata documents
    // All the OData services listed below are taken from the ES5 SAP Dev Center OData server
    // https://sapes5.sapdevcenter.com/sap/opu/odata/iwfnd/CATALOGSERVICE/CatalogCollection('ES5')/Services
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn parse_all() {
        assert!(parse_sap_metadata("catalogservice", "CATALOGSERVICE").is_ok());
        assert!(parse_sap_metadata("catalogservice_v0002", "CATALOGSERVICE").is_ok());
        assert!(parse_sap_metadata("epm_ref_apps_shop_srv", "EPM_REF_APPS_SHOP").is_ok());
        assert!(parse_sap_metadata("epm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV").is_ok());
        assert!(
            parse_sap_metadata("epm_ref_apps_prod_man_srv", "EPM_REF_APPS_PROD_MAN_SRV").is_ok()
        );
        assert!(parse_sap_metadata("error_log_srv", "ERROR_LOG_SRV").is_ok());
        assert!(parse_sap_metadata("gwdemo", "GWDEMO").is_ok());
        assert!(parse_sap_metadata("gwsample_basic", "GWSAMPLE_BASIC").is_ok());
        assert!(parse_sap_metadata("interop", "INTEROP").is_ok());
        assert!(parse_sap_metadata("page_builder_conf", "PAGE_BUILDER_CONF").is_ok());
        assert!(parse_sap_metadata("page_builder_cust", "PAGE_BUILDER_CUST").is_ok());
        assert!(parse_sap_metadata("page_builder_pers", "PAGE_BUILDER_PERS").is_ok());
        assert!(parse_sap_metadata("rmtsampleflight", "RMTSAMPLEFLIGHT").is_ok());
        assert!(parse_sap_metadata("sepmra_gr_post", "SEPMRA_GR_POST").is_ok());
        assert!(parse_sap_metadata("sepmra_ovw", "SEPMRA_OVW").is_ok());
        assert!(parse_sap_metadata("sepmra_po_apv", "EPM_REF_APPS_PROD_MAN_SRV").is_ok());
        assert!(parse_sap_metadata("sepmra_po_man", "SEPMRA_PO_MAN").is_ok());
        assert!(parse_sap_metadata("sepmra_prod_man", "SEPMRA_PROD_MAN").is_ok());
        assert!(parse_sap_metadata("sepmra_shop", "SEPMRA_SHOP").is_ok());
        assert!(parse_sap_metadata("sepmra_so_man", "SEPMRA_SO_MAN").is_ok());
        assert!(parse_sap_metadata("sgbt_nte_cds_api_d_srv", "SGBT_NTE_CDS_API_D_SRV").is_ok());
        assert!(parse_sap_metadata("sgbt_nte_cds_api_srv", "SGBT_NTE_CDS_API_SRV").is_ok());
        assert!(parse_sap_metadata("transport", "TRANSPORT").is_ok());
        assert!(
            parse_sap_metadata("z_test_cds_with_param_srv", "Z_TEST_CDS_WITH_PARAM_SRV").is_ok()
        );
        assert!(parse_sap_metadata("zagencycds_srv", "ZAGENCYCDS_SRV").is_ok());
        assert!(parse_sap_metadata("zdevelopercenter", "ZDEVELOPERCENTER").is_ok());
        assert!(parse_sap_metadata("zepm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV").is_ok());
        assert!(parse_sap_metadata("ze2e100_sol_2_srv", "ZE2E100_SOL_2_SRV").is_ok());
        assert!(parse_sap_metadata("zepm_ref_apps_po_apv_srv", "EPM_REF_APPS_PO_APV_SRV").is_ok());
        assert!(parse_sap_metadata("zrfc1_srv", "ZRFC1_SRV").is_ok());
        assert!(parse_sap_metadata("zpdcds_srv", "ZPDCDS_SRV").is_ok());
        assert!(parse_sap_metadata("zsocds_srv", "ZSOCDS_SRV").is_ok());
    }
}
