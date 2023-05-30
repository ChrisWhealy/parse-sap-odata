# Parse the Metadata from an SAP OData Service

Parse the metadata XML describing an SAP Odata service and generate `struct`s for each EDM:

* `ComplexType`
* `EntityType`
* `FunctionImport`

This is a work in progress

## TODOs

Create `Build.rs` process

## Prerequisites

The Rust tool chain can be installed by following [these instructions](https://www.rust-lang.org/tools/install)

## Clone Reposotory

```bash
$ git clone https://github.com/lighthouse-no/parse-sap-odata
```

## Testing

### Test Parser

To test only the parsing process, run the `parse_all` test:

```bash
$ cargo test parse_all -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/parse_sap_odata-37f19949c8ea98f2)

running 1 test
Parsing error_log_srv.xml             =>   74912 bytes written
Parsing zepm_ref_apps_po_apv_srv.xml  =>   99106 bytes written
Parsing rmtsampleflight.xml           =>  256713 bytes written
Parsing epm_ref_apps_prod_man_srv.xml =>  299482 bytes written
Parsing zrfc1_srv.xml                 =>   63035 bytes written
Parsing catalogservice.xml            =>    2162 bytes written
Parsing sgbt_nte_cds_api_d_srv.xml    =>  133425 bytes written
Parsing gwsample_basic.xml            =>  274027 bytes written
Parsing ze2e100_sol_2_srv.xml         =>  100926 bytes written
Parsing page_builder_cust.xml         =>  185291 bytes written
Parsing transport.xml                 =>   19981 bytes written
Parsing sepmra_po_man.xml             =>  375171 bytes written
Parsing epm_ref_apps_shop_srv.xml     =>  179706 bytes written
Parsing sepmra_po_apv.xml             =>  299482 bytes written
Parsing zdevelopercenter.xml          =>   19678 bytes written
Parsing epm_ref_apps_po_apv_srv.xml   =>   80710 bytes written
Parsing gwdemo.xml                    =>  398960 bytes written
Parsing sgbt_nte_cds_api_srv.xml      =>   63474 bytes written
Parsing z_test_cds_with_param_srv.xml =>   29760 bytes written
Parsing catalogservice_v0002.xml      =>   80675 bytes written
Parsing sepmra_ovw.xml                =>  203951 bytes written
Parsing sepmra_gr_post.xml            =>   80433 bytes written
Parsing sepmra_so_man.xml             =>  515440 bytes written
Parsing page_builder_conf.xml         =>  185291 bytes written
Parsing interop.xml                   =>  276444 bytes written
Parsing zagencycds_srv.xml            =>   17006 bytes written
Parsing zpdcds_srv.xml                =>   38757 bytes written
Parsing zsocds_srv.xml                =>   72883 bytes written
Parsing page_builder_pers.xml         =>  185291 bytes written
Parsing sepmra_prod_man.xml           =>  760258 bytes written
Parsing sepmra_shop.xml               =>  179460 bytes written
test unit_tests::tests::parse_all ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.23s
```

For each input XML file, a corresponding parsed `.txt` file will appear in the `./parsed` directory.

### Test Code Generator

To test the code generation process, run the `gen_src_all` test:

```bash
13:49 $ cargo test gen_src_all -- --nocapture
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
