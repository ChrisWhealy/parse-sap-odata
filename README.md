# Parse the Metadata from an SAP OData Service

First attempt at consuming the metadata from an SAP Odata service.

This is a work in progress

## TODOs

At the moment, some field names in OData EntityTypes clash with Rust reserved words.

These field names need to be escaped: E.G. `type` --> `r#type`

## Prerequisites

The Rust tool chain can be installed by following [these instructions](https://www.rust-lang.org/tools/install)

## Testing

```bash
$ git clone https://github.com/lighthouse-no/parse-sap-odata
```

To test the parsing process, run only the `parse_all` test:

```bash
$ cargo test parse_all -- --nocapture
   Compiling parse-sap-odata v0.1.0 (/Users/chris/Developer/lighthouse-no/parse-sap-odata)
    Finished test [unoptimized + debuginfo] target(s) in 0.88s
     Running unittests src/lib.rs (target/debug/deps/parse_sap_odata-c2a45b946a567a47)

running 1 test
Parsing interop.xml                   =>  276444 bytes written
Parsing sepmra_shop.xml               =>  179460 bytes written
Parsing catalogservice_v0002.xml      =>   80675 bytes written
Parsing zdevelopercenter.xml          =>   19678 bytes written
Parsing sgbt_nte_cds_api_srv.xml      =>   63474 bytes written
Parsing ze2e100_sol_2_srv.xml         =>  100926 bytes written
Parsing zpdcds_srv.xml                =>   38757 bytes written
Parsing transport.xml                 =>   19981 bytes written
Parsing epm_ref_apps_prod_man_srv.xml =>  299482 bytes written
Parsing rmtsampleflight.xml           =>  256713 bytes written
Parsing epm_ref_apps_shop_srv.xml     =>  179706 bytes written
Parsing catalogservice.xml            =>    2162 bytes written
Parsing sepmra_prod_man.xml           =>  760258 bytes written
...SNIP...
```

For each input XML file, a corresponding parsed `.txt` file will appear in the `./parsed` directory.

To test the code generation process, run only the `gen_src` test:

```bash
cargo test gen_src -- --nocapture
   Compiling parse-sap-odata v0.1.0 (/Users/chris/Developer/lighthouse-no/parse-sap-odata)
    Finished test [unoptimized + debuginfo] target(s) in 1.75s
     Running unittests src/lib.rs (target/debug/deps/parse_sap_odata-3e89fc9e5463a5ac)

running 1 test
Generating source code from catalogservice.xml
Generating source code from catalogservice_v0002.xml
Generating source code from error_log_srv.xml
Generating source code from sgbt_nte_cds_api_d_srv.xml
Generating source code from zpdcds_srv.xml
Generating source code from zepm_ref_apps_po_apv_srv.xml
Generating source code from gwsample_basic.xml
Generating source code from epm_ref_apps_prod_man_srv.xml
Generating source code from sepmra_po_man.xml
Generating source code from sepmra_shop.xml
Generating source code from epm_ref_apps_shop_srv.xml
Generating source code from rmtsampleflight.xml
Generating source code from zsocds_srv.xml
Generating source code from page_builder_conf.xml

...SNIP...
```
