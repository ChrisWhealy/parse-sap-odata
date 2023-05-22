# Parse the Metadata from an SAP OData Service

First attempt at consuming the metadata from an SAP Odata service.

This is a work in progress

## Prerequisites

The Rust tool chain can be installed by following [these instructions](https://www.rust-lang.org/tools/install)

## Testing

```bash
$ git clone https://github.com/lighthouse-no/parse-sap-odata
$ cargo test -- --nocapture
   Compiling parse-sap-odata v0.1.0 (/Users/chris/Developer/lighthouse-no/parse-sap-odata)
    Finished test [unoptimized + debuginfo] target(s) in 0.88s
     Running unittests src/lib.rs (target/debug/deps/parse_sap_odata-c2a45b946a567a47)

running 1 test
Parsing ./tests/catalogservice.xml.  441 bytes written
Parsing ./tests/catalogservice_v0002.xml.  36322 bytes written
Parsing ./tests/epm_ref_apps_shop_srv.xml.  87012 bytes written
Parsing ./tests/epm_ref_apps_po_apv_srv.xml.  37213 bytes written
Parsing ./tests/epm_ref_apps_prod_man_srv.xml.  166391 bytes written
Parsing ./tests/error_log_srv.xml.  33355 bytes written
Parsing ./tests/gwdemo.xml.  186417 bytes written
Parsing ./tests/gwsample_basic.xml.  129405 bytes written
Parsing ./tests/interop.xml.  124305 bytes written
Parsing ./tests/page_builder_conf.xml.  86337 bytes written
Parsing ./tests/page_builder_cust.xml.  86337 bytes written
Parsing ./tests/page_builder_pers.xml.  86337 bytes written
Parsing ./tests/rmtsampleflight.xml.  115344 bytes written
Parsing ./tests/sepmra_gr_post.xml.  38708 bytes written
Parsing ./tests/sepmra_ovw.xml.  93950 bytes written
Parsing ./tests/sepmra_po_apv.xml.  166391 bytes written
Parsing ./tests/sepmra_po_man.xml.  200975 bytes written
Parsing ./tests/sepmra_prod_man.xml.  404635 bytes written
Parsing ./tests/sepmra_shop.xml.  86606 bytes written
Parsing ./tests/sepmra_so_man.xml.  281602 bytes written
Parsing ./tests/sgbt_nte_cds_api_d_srv.xml.  62474 bytes written
Parsing ./tests/sgbt_nte_cds_api_srv.xml.  28945 bytes written
Parsing ./tests/transport.xml.  8286 bytes written
Parsing ./tests/z_test_cds_with_param_srv.xml.  13205 bytes written
Parsing ./tests/zagencycds_srv.xml.  6849 bytes written
Parsing ./tests/zdevelopercenter.xml.  8127 bytes written
Parsing ./tests/zepm_ref_apps_po_apv_srv.xml.  46017 bytes written
Parsing ./tests/ze2e100_sol_2_srv.xml.  47779 bytes written
Parsing ./tests/zepm_ref_apps_po_apv_srv.xml.  46017 bytes written
Parsing ./tests/zrfc1_srv.xml.  28118 bytes written
Parsing ./tests/zpdcds_srv.xml.  16744 bytes written
Parsing ./tests/zsocds_srv.xml.  32858 bytes written
test unit_tests::tests::parse_all ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
```

For each input XML file, a corresponding parsed `.txt` file will appear in the `./parsed` directory.
