# Parse the Metadata from an SAP OData Service

This is a work in progress!

Parse the metadata XML describing an SAP OData service and generate Rust entities for each EDM type:

* [x] `ComplexType`
* [x] `EntityType`
* [ ] `FunctionImport`


> ***TODO***
>
> Currently when generating a Rust `struct`, only the `Name` and `Type` properties are used.
> Consider how the other XML attribute values and SAP annotations could be made available within the Rust `struct`.

## XML Input

Currently, the metadata XML for an OData service must be written to a local file that this code generator then reads.

> ***TODO***
>
> Consider fetching the metadata at build time &mdash; but this raises questions about whether allowing a build script to look outside its sandbox is an anti-pattern.
>
> Yeah, it might well be...

Within the metadata for the `GWSAMPLE_BASIC` OData service, there is the following entity type declaration for `SalesOrderLineItem`:

```xml
<EntityType Name="SalesOrderLineItem" sap:content-version="1">
  <Key>
    <PropertyRef Name="SalesOrderID"/>
    <PropertyRef Name="ItemPosition"/>
  </Key>
  <Property Name="SalesOrderID" Type="Edm.String" Nullable="false" MaxLength="10" sap:unicode="false" sap:label="Sa. Ord. ID" sap:updatable="false"/>
  <Property Name="ItemPosition" Type="Edm.String" Nullable="false" MaxLength="10" sap:unicode="false" sap:label="PO Item Pos" sap:creatable="false" sap:updatable="false"/>
  <Property Name="ProductID" Type="Edm.String" Nullable="false" MaxLength="10" sap:unicode="false" sap:label="Product ID"/>
  <Property Name="Note" Type="Edm.String" MaxLength="255" sap:unicode="false" sap:label="Description" sap:sortable="false" sap:filterable="false"/>
  <Property Name="NoteLanguage" Type="Edm.String" MaxLength="2" sap:unicode="false" sap:label="Language" sap:creatable="false" sap:updatable="false" sap:sortable="false" sap:filterable="false"/>
  <Property Name="CurrencyCode" Type="Edm.String" MaxLength="5" sap:unicode="false" sap:label="Currency" sap:creatable="false" sap:updatable="false" sap:semantics="currency-code"/>
  <Property Name="GrossAmount" Type="Edm.Decimal" Precision="16" Scale="3" sap:unicode="false" sap:unit="CurrencyCode" sap:label="Gross Amt." sap:creatable="false" sap:updatable="false"/>
  <Property Name="NetAmount" Type="Edm.Decimal" Precision="16" Scale="3" sap:unicode="false" sap:unit="CurrencyCode" sap:label="Net Amt." sap:creatable="false" sap:updatable="false"/>
  <Property Name="TaxAmount" Type="Edm.Decimal" Precision="16" Scale="3" sap:unicode="false" sap:unit="CurrencyCode" sap:label="Tax Amt." sap:creatable="false" sap:updatable="false"/>
  <Property Name="DeliveryDate" Type="Edm.DateTime" Nullable="false" Precision="7" sap:unicode="false" sap:label="Time Stamp" sap:sortable="false" sap:filterable="false"/>
  <Property Name="Quantity" Type="Edm.Decimal" Nullable="false" Precision="13" Scale="3" sap:unicode="false" sap:unit="QuantityUnit" sap:label="Quantity" sap:sortable="false" sap:filterable="false"/>
  <Property Name="QuantityUnit" Type="Edm.String" MaxLength="3" sap:unicode="false" sap:label="Qty. Unit" sap:creatable="false" sap:updatable="false" sap:sortable="false" sap:filterable="false" sap:semantics="unit-of-measure"/>
  <NavigationProperty Name="ToHeader" Relationship="GWSAMPLE_BASIC.Assoc_SalesOrder_SalesOrderLineItems" FromRole="ToRole_Assoc_SalesOrder_SalesOrderLineItems" ToRole="FromRole_Assoc_SalesOrder_SalesOrderLineItems"/>
  <NavigationProperty Name="ToProduct" Relationship="GWSAMPLE_BASIC.Assoc_Product_SalesOrderLineItems" FromRole="ToRole_Assoc_Product_SalesOrderLineItems" ToRole="FromRole_Assoc_Product_SalesOrderLineItems"/>
</EntityType>
```

## Rust Output

From the above definition, the `Name` and `Type` attributes are used to generate the following basic Rust `struct`:

```rust
#[derive(Clone, Copy, Debug)]
pub struct SalesOrderLineItem {
    pub currency_code: Option<String>,
    pub delivery_date: NaiveDateTime,
    pub gross_amount: rust_decimal::Decimal,
    pub item_position: String,
    pub net_amount: rust_decimal::Decimal,
    pub note: Option<String>,
    pub note_language: Option<String>,
    pub product_id: String,
    pub quantity: rust_decimal::Decimal,
    pub quantity_unit: Option<String>,
    pub sales_order_id: String,
    pub tax_amount: rust_decimal::Decimal,
}
```

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
#[derive(Clone, Copy, Debug)]
pub struct Address {
    pub address_type: Option<String>,
    pub building: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub street: Option<String>,
}
```

## OData "Simple" Complex Types

The metadata for the `GWSAMPLE_BASIC` OData service contains the following complex type:

```xml
<ComplexType Name="CT_String">
  <Property Name="String" Type="Edm.String" Nullable="false" sap:creatable="false" sap:updatable="false" sap:sortable="false" sap:filterable="false"/>
</ComplexType>
```

Allowing for the current situation in which additional attribute values and SAP Annotations are not preserved, this particular type turns out not to be complex at all &mdash; its just a `String`.
In such cases, fields declared to be of these "simple" complex types (such as `CT_String`), are collapsed down to the Rust native type of the single inner property &mdash; which in this example is simply a `String`.

## Other TODOs

* Support Function Imports
* Check whether the Rust `chrono::naive::NaiveDateTime` type is the best fit for `Edm.DateTimeOffset`
* Implement `build.rs` script to allow another app to consume the generated code
  * Write generated code to the directory held in the compile time environment variable `OUT_DIR`

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
