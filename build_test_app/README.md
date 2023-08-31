# Build Test App

This is a simple test app that invokes `parse-sap-odata` in a build script for the SAP demo OData service `GWSAMPLE_BASIC` and then consumes generated the `struct`s and `enum`s.

```shell
$ cargo run
 Compiling parse-sap-odata v1.1.8 (/Users/chris/Developer/lighthouse-no/parse-sap-odata)
 Compiling build-test-app v1.2.0 (/Users/chris/Developer/lighthouse-no/parse-sap-odata/build_test_app)
  Finished dev [unoptimized + debuginfo] target(s) in 8.92s
   Running `target/debug/build-test-app`
```

The app then starts a webserver running on <http://localhost:8080> that displays a simple drop down list

![Start screen](../img/start_screen.png)

Select the desired entity set and the first 100 entries will be displayed in plain text.
This output comes from the Rust `println!()` macro printing the Rust `struct` into which the entity set data has been parsed.

## Known Issues/Workarounds

### Invalid ETag Attribute Values

When calling the SAP demo OData service `GWSAMPLE_BASIC`, various entity sets (such as `BusinessPartnerSet` and `ProductSet`) return `<entry>` tags whose `m:etag` attribute contains an invalid value.

The raw XML will contain `m:etag` attributes with values like this:

```xml
<entry m:etag="W/"datetime'2023-08-31T01%3A00%3A06.0000000'"">
```

The extra `"W/` characters at the start and the extra `"` character at the end are invalid XML and will cause the XML parser to throws its toys out of the pram.
These invalid values are checked for and removed to give:

```xml
<entry m:etag="datetime'2023-08-31T01%3A00%3A06.0000000'">
```

### Empty `DateOfBirth` Field

Some entity sets have `<entry>` tags that contain `edm:DateTime` properties that can be null.
For `DateOfBirth` fields, instead of specifying an empty `DateOfBirth` field as:

```xml
<d:DateOfBirth/>
```

It is provided as:

```xml
<d:DateOfBirth m:null="true"/>
```

When attempting to parse such values as `Option<chrono::NaiveDateTime>`, the `quick_xml` parser aborts with the error `Premature end of input`.
Consequently, such fields are interpreted simply as `String`s.

A dedicated parser function is needed here.

### Parsing `Edm.Decimal` Fields

At the moment, there is no dedicated parser function for fields of type `Edm.Decimal`; they are currently stored as `f64` values.

## Prerequisites

You must already have a userid and password for the SAP Dev Center server `sapes5.sapdevcenter.com`

1. Clone this repo
1. `cd parse_sap_odata/build_test_app`
1. Create a `.env` file containing your SAP DevCenter userid and password in the following format

   ```
   SAP_USER=<your userid>
   SAP_PASSWORD=<your password>
   ```

## Running `build_test_app`

1. In directory `build_test_app` run `cargo run`
1. Open your browser and go to <http://localhost:8080>
1. Select the name of the entity set whose data you want to see

1. You will then see the first 100 entries from the selected entity set in JSON format.

## Testing this Crate Locally

The prefered test tool is [`nextest`](https://crates.io/crates/cargo-nextest) which can be [installed from here](https://nexte.st/).

If you have not installed `nextest`, `cargo test` can be used instead.