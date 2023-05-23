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
Parsing sepmra_so_man.xml             => 485399 bytes written
Parsing sgbt_nte_cds_api_srv.xml      => 57071 bytes written
Parsing epm_ref_apps_prod_man_srv.xml => 278661 bytes written
Parsing sgbt_nte_cds_api_d_srv.xml    => 120762 bytes written
Parsing error_log_srv.xml             => 65387 bytes written
Parsing sepmra_prod_man.xml           => 715037 bytes written
Parsing sepmra_po_apv.xml             => 278661 bytes written
Parsing epm_ref_apps_shop_srv.xml     => 163937 bytes written
Parsing zpdcds_srv.xml                => 34138 bytes written
Parsing transport.xml                 => 17815 bytes written
Parsing zdevelopercenter.xml          => 17289 bytes written
Parsing zrfc1_srv.xml                 => 55294 bytes written
Parsing gwsample_basic.xml            => 250118 bytes written
Parsing epm_ref_apps_po_apv_srv.xml   => 72755 bytes written
Parsing catalogservice.xml            => 2003 bytes written
Parsing catalogservice_v0002.xml      => 71866 bytes written
Parsing zepm_ref_apps_po_apv_srv.xml  => 89367 bytes written
Parsing sepmra_po_man.xml             => 353588 bytes written
Parsing rmtsampleflight.xml           => 226979 bytes written
Parsing z_test_cds_with_param_srv.xml => 26925 bytes written
Parsing page_builder_pers.xml         => 168205 bytes written
Parsing zsocds_srv.xml                => 64473 bytes written
Parsing page_builder_conf.xml         => 168205 bytes written
Parsing sepmra_ovw.xml                => 183276 bytes written
Parsing sepmra_gr_post.xml            => 74253 bytes written
Parsing ze2e100_sol_2_srv.xml         => 90509 bytes written
Parsing gwdemo.xml                    => 363053 bytes written
Parsing interop.xml                   => 244936 bytes written
Parsing zagencycds_srv.xml            => 15063 bytes written
Parsing sepmra_shop.xml               => 163691 bytes written
Parsing page_builder_cust.xml         => 168205 bytes written
test unit_tests::tests::parse_all ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
```

For each input XML file, a corresponding parsed `.txt` file will appear in the `./parsed` directory.
