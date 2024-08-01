# Usage

This crate contains the functionality to build two Rust modules from the metadata belonging to an SAP OData V2 service.

## Generating Modules

1. In a root level project folder called `/odata`, store a local copy the OData service's metadata XML.

   E.G. To consume the demo OData service `GWSAMPLE_BASIC`, store the `$metadata` response in the file `/odata/gwsample_basic.xml`

1. In your `Cargo.toml`, add an entry to `[build-dependencies]` with the `parser` feature enabled.

   ```toml
   [build-dependencies]
   parse-sap-odata = { version = "1.3", features = ["parser"]}
   ```

1. In the project's build script, call `parser::gen_src` function passing in the file name of the XML metadata file.

    ```rust
    use parse_sap_odata::parser::gen_src;

    fn main() {
        gen_src(
            "gwsample_basic", // metadata_file_name.  The ".xml" extension is added automatically
            "GWSAMPLE_BASIC"  // Value of the Namespace attribute of the <Schema> tag
        );
    }
    ```

1. Two Rust modules will be generated: one for the OData service document, and one for the service metadata.

## Using the Generated Modules

1. In `Cargo.toml`, you need `[dependencies]` on the crates `parse-sap-odata` and `parse-sap-atom-feed`.

   ```toml
   parse-sap-odata = "1.3"
   parse-sap-atom-feed = "0.2"
   ```

1. The generated modules also have their own runtime `[dependencies]`:

   ```toml
   chrono = { version = "0.4", features = ["serde"] }
   quick-xml = { version = "0.32", features = ["serialize"] }
   rust_decimal = { version = "1.30", features = ["serde-with-str"]}
   serde = { version = "1.0", features = ["derive"] }
   uuid = { version = "1.4", features = ["serde"] }
   ```

1. In the source code of your application, use the `include!()` and `env!()` macros to pull in the generated modules, then bring them into scope with a `use` command:

   ```rust
   // Include the generated code
   include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));
   include!(concat!(env!("OUT_DIR"), "/gwsample_basic_metadata.rs"));

   use gwsample_basic::*;
   use gwsample_basic_metadata::*;

   // Use the BusinessPartner struct for example
   fn main() {
       let bp: BusinessPartner = Default::default();
       println!("{:#?}", bp);
   }
   ```
