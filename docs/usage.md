# Usage

This crate contains the functionality to build two Rust modules from the metadata belonging to an SAP OData V2 service.

## Generating Modules

1. In the root level of your business app project, create a folder called `/odata` and in it, store a local copy the OData service's metadata XML.

   E.G. To consume the demo OData service `GWSAMPLE_BASIC`, store the `$metadata` response in the file `/odata/gwsample_basic.xml`

1. In your `Cargo.toml`, add a `[build-dependencies]` on `parse-sap-odata` with the `parser` feature enabled.

   ```toml
   [build-dependencies]
   parse-sap-odata = { version = "1.4", features = ["parser"]}
   ```

1. In the project's build script, call `parser::gen_src` function passing in the file name of the XML metadata file.

    ```rust
    fn main() {
        parse_sap_odata::parser::gen_src(
            "gwsample_basic", // metadata_file_name.  The ".xml" extension is added automatically
            "GWSAMPLE_BASIC"  // Value of the Namespace attribute of the <Schema> tag
        );
    }
    ```

1. Two Rust modules will be generated: one for the OData service document, and one for the service metadata.

## Using the Generated Modules

1. In `Cargo.toml`, add at least the following:

   ```toml
   [dependencies]
   # parse-sap-odata's fixed dependencies 
   serde = { version = "1.0", features = ["derive"] }
   quick-xml = "0.36"
   
   # parse-sap-odata's possible dependencies (varies depending on which EDM properties are encountered)
   chrono = { version = "0.4", features = ["serde"]}
   rust_decimal = "1.36"
   uuid = { version = "1.10", features = ["serde"]}
   
   # Dependencies needed by this app
   parse-sap-odata = "1.4"
   parse-sap-atom-feed = "1.1"
   ```

1. In the source code of your application, use the `include_mod!()` macro to bring the generated modules into scope:

   ```rust
   // Include the generated modules
   parse_sap_odata::include_mod!("gwsample_basic");
   parse_sap_odata::include_mod!("gwsample_basic_metadata");

   use gwsample_basic::*;
   use gwsample_basic_metadata::*;

   // Use the BusinessPartner struct for example
   fn main() {
       let bp: BusinessPartner = Default::default();
       println!("{:#?}", bp);
   }
   ```
