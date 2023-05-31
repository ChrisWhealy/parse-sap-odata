mod tests {
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::io::Write;

    use crate::utils::{
        longest,
        parse_odata::{deserialize_sap_metadata, gen_src},
        write_entity,
    };

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // A list of all the working OData services available from the ES5 SAP Dev Center OData server
    // https://sapes5.sapdevcenter.com/sap/opu/odata/iwfnd/CATALOGSERVICE/CatalogCollection('ES5')/Services
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

        let max_name_len = longest(&service_list);

        (service_list, max_name_len)
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Parse all listed OData services
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn parse_all() {
        let mut out_buffer: Vec<u8> = Vec::new();
        let (srv_list, max_name_len) = gen_service_list();

        for srv in srv_list {
            print!(
                "Parsing {}.xml{:>width$}=> ",
                srv.0,
                " ",
                width = max_name_len - srv.0.len() + 1,
            );

            match deserialize_sap_metadata(srv.0) {
                Err(err) => println!("Error: {}", err.msg),
                Ok(edmx) => {
                    out_buffer.clear();
                    let mut out_file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(format!("./parsed/{}.txt", srv.0))
                        .unwrap();

                    write_entity(&mut out_buffer, Some(&vec![edmx]));

                    out_file.write_all(&out_buffer).unwrap();
                    println!("{:>7} bytes written", out_buffer.len());
                }
            };
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Generate Rust type declarations for all listed OData services
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[test]
    pub fn gen_src_all() {
        let (srv_list, _max_name_len) = gen_service_list();

        for srv in srv_list {
            println!("Generating source code from {}.xml", srv.0);
            gen_src(srv.0, srv.1);
        }
    }
}
