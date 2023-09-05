# Parse the Metadata from an SAP OData V2 Service

This is a work in progress!

Parse the metadata XML describing an SAP OData V2 service and generate basic Rust entities for each EDM type:

* [x] `ComplexType`
* [x] `EntityType`
* [ ] `FunctionImport`

## Limitations and Issues

1. Currently when generating a Rust `struct`, only the `Name` and `Type` properties are extracted from the XML `<EntityType>` declaration.
  Consider how the other XML attribute values and SAP annotations could be made available within the generated Rust `struct`.

1. The OData metadata contained within the `<FunctionImport>`, `<Association>` and `<AssociationSet>` tags is parsed, but not currently acted upon.

1. When calling some of the entity sets in the demo OData service `GWSAMPLE_BASIC`, certain XML properties are returned whose values that are not valid XML.
   Consequently, when `quick_xml` attempts to parse such values, it simply throws its toys out the pram and doesn't want to play anymore.

   If you encounter such errors, the raw XML string must first be sanitised before attempting to parse it.

   This functionality is described in the [README of `parse-sap-odata-demo`](https://github.com/lighthouse-no/parse-sap-odata-demo).

---

## TODOs

1. Implement dedicated parser functions for the types `Edm.DateTime` and `Edm.Decimal` as fields of these types are currently interpreted simply as `String` and `f64` fields respectively.
1. Support Function Imports.

---

## Usage

You want to write a Rust application to interact with an SAP OData V2 service.

The functionality in this crate is invoked by the build script in your application and generates the `struct`s and `enum`s needed to consume the data returned by calling the OData service's entity sets.

Your app can then consume the entity set data using the code in crate [`parse-sap-atom-feed`](https://crates.io/crates/parse-sap-atom-feed)

### Declare Build Dependency

In the `Cargo.toml` of your application, define an entry in `[build-dependencies]` that points to the `parse-sap-odata` crate:

```toml
[build-dependencies]
parse-sap-odata = "1.1"
```

Your app will require at least these dependencies:

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
parse-sap-atom-feed = "0.1"
rust_decimal = "1.30"
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.4", features = ["serde"] }
```

### XML Input Files

All metadata XML for the OData services your app consumes must be located in the `./odata` directory immediately under your app's top level directory.

Using the demo service `GWSAMPLE_BASIC` available from SAP's Dev Center server, display the [metadata XML](https://sapes5.sapdevcenter.com/sap/opu/odata/iwbep/GWSAMPLE_BASIC/$metadata) for this service, then save that in file `./odata/gwsample_basic.xml`.

### Create a Build Script

In your app's build script (`build.rs`), run the generator for your desired OData service:

```rust
use parse_sap_odata::parser::gen_src;

fn main() {
    gen_src(
        "gwsample_basic", // metadata_file_name.  The ".xml" extension is assumed
        "GWSAMPLE_BASIC"  // Value of the Namespace attribute on the <Schema> tag
    );
}
```

More information about Rust [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) is available on the documentation site.

### Generated Output

If `cargo` detects a `build.rs` file in your project/crate, then it automatically populates the environment variable `OUT_DIR` and runs `build.rs` before compiling your application.

The `OUT_DIR` variable then points to the directory into which all build script output is written.

The default directory name is `target/debug/build/<your_package_name>/out`, and this is where you can find the generated `struct` declarations for the OData service.

You can specify your own value for `OUT_DIR` either by passing a value to `cargo`'s `--out_dir` flag, or by defining your own location in a `config.toml` file in the `./.cargo` directory.
See [Cargo Configuration](https://doc.rust-lang.org/cargo/reference/config.html) for more details.

All generated `struct`s implement at least the following traits `#[derive(Clone, Debug, Default)]`

---

## Referencing Generated Output

In the source code of your application, you can reference the generated source code like this:

```rust
// Include the generated code
include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

// Use the BusinessPartner struct for example
fn main() {
    let bp: BusinessPartner = Default::default();
    println!("{:#?}", bp);
}
```

---

## OData Complex Types

In the event an Entity Type definition uses a complex type, then the complex type is first created as a Rust `struct`.
The field in Rust `struct` that has this complex type is then defined using this `struct`.

An example of this is the `Address` property.

```xml
<EntityType Name="BusinessPartner" sap:content-version="1">
  <Key>
    <PropertyRef Name="BusinessPartnerID"/>
  </Key>
  <Property Name="Address" Type="GWSAMPLE_BASIC.CT_Address" Nullable="false"/>

  <!-- SNIP -->

</EntityType>
```

The Rust `struct` name is generated by trimming the namespace qualifier and (if present) the `CT_` prefix

```xml
<ComplexType Name="CT_Address">
  <Property Name="City" Type="Edm.String" MaxLength="40" sap:label="City" sap:semantics="city"/>
  <Property Name="PostalCode" Type="Edm.String" MaxLength="10" sap:label="Postal Code" sap:semantics="zip"/>
  <Property Name="Street" Type="Edm.String" MaxLength="60" sap:label="Street" sap:semantics="street"/>
  <Property Name="Building" Type="Edm.String" MaxLength="10" sap:label="Building"/>
  <Property Name="Country" Type="Edm.String" MaxLength="3" sap:label="Country" sap:semantics="country"/>
  <Property Name="AddressType" Type="Edm.String" MaxLength="2" sap:label="Address Type"/>
</ComplexType>
```

So the above XML definition becomes:

```rust
#[derive(Clone, Debug, Default)]
pub struct Address {
    pub address_type: Option<String>,
    pub building: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub street: Option<String>,
}
```

---

## OData "Simple" Complex Types

The metadata for the `GWSAMPLE_BASIC` OData service contains the following complex type:

```xml
<ComplexType Name="CT_String">
  <Property Name="String" Type="Edm.String" Nullable="false" sap:creatable="false" sap:updatable="false" sap:sortable="false" sap:filterable="false"/>
</ComplexType>
```

Allowing for the current situation in which additional attribute values and SAP Annotations are not preserved, this particular type turns out not to be complex at all &mdash; its just a `String`.
In such cases, fields declared to be of these "simple" complex types (such as `CT_String`), are collapsed down to the Rust native type of the single inner property &mdash; which in this example is simply a `String`.

---

## Entity Sets Enum

On the basis that a single OData service exposes a static list of entity sets, and within the scope of any single request, you will only ever be interacting with a single entity set, it makes sense to treat each entity set name as an `enum` variant.

Under the `<Schema>` element in the OData service document, there is an `<EntityContainer>` element.
All entity sets available through this OData service are identified here with their own `<EntitySet Name="<some_name>">` tag.

The following naming convention is used: `<odata_service_name>Entities`.

For example, the entity sets belonging to the OData service `GWSAMPLE_BASIC` become the following `enum`:

```rust
#[derive(Copy, Clone, Debug)]
pub enum GwsampleBasicEntities {
    BusinessPartnerSet,
    ProductSet,
    SalesOrderSet,
    SalesOrderLineItemSet,
    ContactSet,
    VhSexSet,
    VhCountrySet,
    VhAddressTypeSet,
    VhCategorySet,
    VhCurrencySet,
    VhUnitQuantitySet,
    VhUnitWeightSet,
    VhUnitLengthSet,
    VhProductTypeCodeSet,
    VhBpRoleSet,
    VhLanguageSet,
}
```

Three convenience functions are then implemented for `enum GwsampleBasicEntities`:

```rust
impl GwsampleBasicEntities {
    pub const fn value(&self) -> &'static str { /* SNIP */ }
    pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> { /* SNIP */ }
    pub fn as_list() -> Vec<&'static str> { /* SNIP */ }
}
```

### Entity Set Enum `value` function

This function returns the name of the entity set variant as a static string slice:

```rust
GwsampleBasicEntities::ProductSet::value();   // -> "ProductSet"
```

### Entity Set Enum `iterator` function

For standard Rust `enums` such as `Option` and `Result`, it makes little sense to attempt to loop over their variants simply because these `enum`s exist specifically to gather together diverse types into a single object.
E.G. The `Option` `enum` exists to provide a type-safe mechanism for handling the possibility that a variable might not contain a value.

However, an OData service guarantees that the entity set names form an immutable, type-safe list.
Therefore, on the basis of this guarantee, it is now both safe and meaningful to implement an `iterator` function.

### Entity Set Enum `as_list` function

By making use of the above `iterator` and `value` functions, the `as_list` function returns the names of the entity sets as a vector of string slices.
