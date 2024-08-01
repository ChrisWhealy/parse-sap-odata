# OData Complex Types

In the event an Entity Type definition uses a complex type, then the complex type is first created as a Rust `struct`.

Entity Type `struct`s are then declared in which each field is either the Rust type equivalent to the Entity Data Model type, or some complex type declared early.

An example of this is the `Address` property in the `BusinessPartner` Entity Type.

```xml
<EntityType Name="BusinessPartner" sap:content-version="1">
  <Key>
    <PropertyRef Name="BusinessPartnerID"/>
  </Key>
  <Property Name="Address" Type="GWSAMPLE_BASIC.CT_Address" Nullable="false"/>

  <!-- SNIP -->

</EntityType>
```

The type name of the `Address` `<Property>` is the Schema namespace, followed by the complex type name.  
E.G `GWSAMPLE_BASIC.CT_Address`.

This refers to the `<ComplexType>` definition:

```xml
<ComplexType Name="CT_Address">
  <Property Name="City"        Type="Edm.String" MaxLength="40" sap:label="City"        sap:semantics="city"/>
  <Property Name="PostalCode"  Type="Edm.String" MaxLength="10" sap:label="Postal Code" sap:semantics="zip"/>
  <Property Name="Street"      Type="Edm.String" MaxLength="60" sap:label="Street"      sap:semantics="street"/>
  <Property Name="Country"     Type="Edm.String" MaxLength="3"  sap:label="Country"     sap:semantics="country"/>
  <Property Name="Building"    Type="Edm.String" MaxLength="10" sap:label="Building"/>
  <Property Name="AddressType" Type="Edm.String" MaxLength="2"  sap:label="Address Type"/>
</ComplexType>
```

So the above XML definition is translated into the following Rust `struct`:

```rust
#[derive(Clone, Debug, Default)]
pub struct CtAddress {
    pub address_type: Option<String>,
    pub building: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub street: Option<String>,
}
```

Then the `<EntityType>` for `BusinessPartner` is translated into the followinf Rust `struct`

```rust
pub struct BusinessPartner {
    pub address: CtAddress,
    pub business_partner_id: String,
    //SNIP
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

This particular type turns out not to be complex at all because it contains a single field whose type is equivalent to a standard Rust `String`.

In such cases, it is redundant to create a Rust `struct` for these "simple" complex types; instead, the corresponding `<EntityType>` field is declared simply as the standard Rust type.
