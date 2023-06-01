# Parse the Metadata from an SAP OData Service

Parse the metadata XML describing an SAP OData service and generate `struct`s for each EDM:

* `ComplexType`
* `EntityType`
* `FunctionImport`

This is a work in progress

## TODOs

* Support function Imports
* Implement the `Build.rs` process

## Prerequisites

The Rust tool chain can be installed by following [these instructions](https://www.rust-lang.org/tools/install)

## Clone Reposotory

```bash
$ git clone https://github.com/lighthouse-no/parse-sap-odata
```

## Testing

All generated code is written to the directory stored in the compile time venvironment variable `OUT_DIR`

```bash
13:49 $ cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/parse_sap_odata-37f19949c8ea98f2)

running 1 test
Generating source code from page_builder_pers.xml
Generating source code from sepmra_gr_post.xml
Generating source code from ze2e100_sol_2_srv.xml
Generating source code from sgbt_nte_cds_api_d_srv.xml
Generating source code from interop.xml
Generating source code from zsocds_srv.xml
Generating source code from epm_ref_apps_po_apv_srv.xml
Generating source code from zdevelopercenter.xml
Generating source code from sepmra_prod_man.xml
Generating source code from sepmra_so_man.xml
Generating source code from z_test_cds_with_param_srv.xml
Generating source code from page_builder_cust.xml
Generating source code from epm_ref_apps_shop_srv.xml
Generating source code from zepm_ref_apps_po_apv_srv.xml
Generating source code from transport.xml
Generating source code from sepmra_po_apv.xml
Generating source code from page_builder_conf.xml
Generating source code from rmtsampleflight.xml
Generating source code from gwsample_basic.xml
Generating source code from zagencycds_srv.xml
Generating source code from zpdcds_srv.xml
Generating source code from epm_ref_apps_prod_man_srv.xml
Generating source code from sepmra_po_man.xml
Generating source code from catalogservice_v0002.xml
Generating source code from catalogservice.xml
Generating source code from gwdemo.xml
Generating source code from sepmra_ovw.xml
Generating source code from error_log_srv.xml
Generating source code from sgbt_nte_cds_api_srv.xml
Generating source code from zrfc1_srv.xml
Generating source code from sepmra_shop.xml
test unit_tests::tests::gen_src_all ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 2.26s
```

For each input XML file, a corresponding `.rs` file will appear in the `./gen` directory.
