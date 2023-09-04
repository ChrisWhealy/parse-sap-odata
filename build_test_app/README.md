# Build Test App

This is a simple test app that invokes `parse-sap-odata` in a build script for the SAP demo OData service `GWSAMPLE_BASIC` and then use the generated `struct`s and `enum`s to consume teh data from this service's entity sets.

## Prerequisites

You must already have a userid and password for the SAP Dev Center server `sapes5.sapdevcenter.com`

1. Clone this repo
1. `cd parse_sap_odata/build_test_app`
1. Create a `.env` file containing your SAP DevCenter userid and password in the following format

   ```
   SAP_USER=<your userid>
   SAP_PASSWORD=<your password>
   ```
Once the `.env` file has been created, you can start the app.

```shell
$ cargo run
 Compiling parse-sap-odata v1.1.8 (/Users/chris/Developer/lighthouse-no/parse-sap-odata)
 Compiling build-test-app v1.2.0 (/Users/chris/Developer/lighthouse-no/parse-sap-odata/build_test_app)
  Finished dev [unoptimized + debuginfo] target(s) in 8.92s
   Running `target/debug/build-test-app`
```

Visit <http://localhost:8080> and you will see a simple drop down list containing the entity sets available on the `GWSAMPLE_BASIC` OData service.

![Start screen](../img/start_screen.png)

Select the desired entity set and the first 100 entries will be displayed in plain text.
This output comes from the Rust `println!()` macro printing the Rust `struct` into which the entity set data has been parsed.

## Known Issues/Workarounds

### Invalid ETag Attribute Values

When calling the SAP demo OData service `GWSAMPLE_BASIC`, various entity sets (such as `BusinessPartnerSet` and `ProductSet`) return `<entry>` tags whose `m:etag` attribute contains a value that is not in valid XML format.

The raw XML will contain `m:etag` attributes with values like this:

```xml
<entry m:etag="W/"datetime'2023-08-31T01%3A00%3A06.0000000'"">
```

The extra `"W/` characters at the start and the extra `"` character at the end are invalid XML and may cause an XML parser to throws its toys out of the pram.
So, before parsing the raw string containing this XML, such invalid values are checked for and removed.

In this case, the `m:etag` value is sanitised to:

```xml
<entry m:etag="datetime'2023-08-31T01%3A00%3A06.0000000'">
```

### Naked Ampersand Characters

Certain text descriptions are provided that contain an unescaped (or naked) ampersand character.
For example:

```xml
<d:Category>PDAs & Organizers</d:Category>
<d:Landx>St Kitts&Nevis</d:Landx>
```

In order to stop the XML parser from barfing, these characters must be replaced with their character encoding:

```xml
<d:Category>PDAs &amp; Organizers</d:Category>
<d:Landx>St Kitts&amp;Nevis</d:Landx>
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
At the moment therefore, such fields are interpreted simply as `String`s.

TODO: A dedicated parser function is needed here.

### Parsing `Edm.Decimal` Fields

At the moment, there is no dedicated parser function for fields of type `Edm.Decimal`; they are currently stored as `f64` values.
This means at the moment that the value of the `Precision` attribute is not retained.

## Testing this Crate Locally

The prefered test tool is [`nextest`](https://crates.io/crates/cargo-nextest) which can be [installed from here](https://nexte.st/).

```shell
$ cargo nextest run
 Compiling build-test-app v1.2.0 (/Users/chris/Developer/lighthouse-no/parse-sap-odata/build_test_app)
  Finished test [unoptimized + debuginfo] target(s) in 2.75s
  Starting 3 tests across 1 binary
      PASS [   0.025s] build-test-app::bin/build-test-app unit_tests::should_parse_contact_set
      PASS [   0.027s] build-test-app::bin/build-test-app unit_tests::should_parse_business_partner_set
      PASS [   0.024s] build-test-app::bin/build-test-app unit_tests::should_parse_product_set
----------
   Summary [   0.028s] 3 tests run: 3 passed, 0 skipped
```

If you have not installed `nextest`, `cargo test` can be used instead.
