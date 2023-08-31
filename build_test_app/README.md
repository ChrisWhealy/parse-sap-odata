# Build Test App

This is a simple test app that invokes `parse-sap-odata` in a build script for the SAP demo OData service `GWSAMPLE_BASIC` and then consumes generated the `struct`s and `enum`s.

The app then starts a webserver running <http://localhost:8080> that displays a simple drop down list

## Known Issues

### Invalid ETag Attribute Values

On the SAP demo OData `GWSAMPLE_BASIC`, various entity sets (such as `BusinessPartnerSet` and `ProductSet`) return `<entry>` with an `m:etag` attribute containing an invalid value.

Your will see `m:etag` attributes values like this:

```xml
<entry m:etag="W/"datetime'2023-08-31T01%3A00%3A06.0000000'"">
```

Instead of this:

```xml
<entry m:etag="datetime'2023-08-31T01%3A00%3A06.0000000'">
```

Such invalid values must be detected and removed otherwise the parser will reject the entire XML document.

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
    ![Start screen](../img/start_screen.png)
1. You will then see the first 100 entries from the selected entity set in JSON format.

## Testing this Crate Locally

The prefered test tool is [`nextest`](https://crates.io/crates/cargo-nextest) which can be [installed from here](https://nexte.st/).

If you have not installed `nextest`, `cargo test` can be used instead.
