## Metadata for Entity Type `struct`s

One or more `struct`s are created for each `<EntityType>` listed in the metadata.

E.G. In the `GWSAMPLE_BASIC` service, the metadata XML for `BusinessPartner` is the following:

```xml
<EntityType Name="BusinessPartner" sap:content-version="1">
    <Key>
        <PropertyRef Name="BusinessPartnerID"/>
    </Key>
    <Property Name="Address" Type="GWSAMPLE_BASIC.CT_Address" Nullable="false"/>
    <Property Name="BusinessPartnerID" Type="Edm.String" Nullable="false" MaxLength="10" sap:unicode="false" sap:label="Bus. Part. ID" sap:creatable="false" sap:updatable="false"/>
    <Property Name="CompanyName" Type="Edm.String" Nullable="false" MaxLength="80" sap:unicode="false" sap:label="Company Name"/>
    <Property Name="WebAddress" Type="Edm.String" sap:unicode="false" sap:label="Web Address" sap:sortable="false" sap:filterable="false" sap:semantics="url"/>
    <Property Name="EmailAddress" Type="Edm.String" Nullable="false" MaxLength="255" sap:unicode="false" sap:label="E-Mail Address" sap:semantics="email"/>
    <Property Name="PhoneNumber" Type="Edm.String" MaxLength="30" sap:unicode="false" sap:label="Phone No." sap:semantics="tel"/>
    <Property Name="FaxNumber" Type="Edm.String" MaxLength="30" sap:unicode="false" sap:label="Fax Number"/>
    <Property Name="LegalForm" Type="Edm.String" MaxLength="10" sap:unicode="false" sap:label="Legal Form"/>
    <Property Name="CurrencyCode" Type="Edm.String" Nullable="false" MaxLength="5" sap:unicode="false" sap:label="Currency" sap:semantics="currency-code"/>
    <Property Name="BusinessPartnerRole" Type="Edm.String" Nullable="false" MaxLength="3" sap:unicode="false" sap:label="Bus. Part. Role"/>
    <Property Name="CreatedAt" Type="Edm.DateTime" Precision="7" sap:unicode="false" sap:label="Time Stamp" sap:creatable="false" sap:updatable="false"/>
    <Property Name="ChangedAt" Type="Edm.DateTime" Precision="7" ConcurrencyMode="Fixed" sap:unicode="false" sap:label="Time Stamp" sap:creatable="false" sap:updatable="false"/>
    <NavigationProperty Name="ToSalesOrders" Relationship="GWSAMPLE_BASIC.Assoc_BusinessPartner_SalesOrders" FromRole="FromRole_Assoc_BusinessPartner_SalesOrders" ToRole="ToRole_Assoc_BusinessPartner_SalesOrders"/>
    <NavigationProperty Name="ToContacts" Relationship="GWSAMPLE_BASIC.Assoc_BusinessPartner_Contacts" FromRole="FromRole_Assoc_BusinessPartner_Contacts" ToRole="ToRole_Assoc_BusinessPartner_Contacts"/>
    <NavigationProperty Name="ToProducts" Relationship="GWSAMPLE_BASIC.Assoc_BusinessPartner_Products" FromRole="FromRole_Assoc_BusinessPartner_Products" ToRole="ToRole_Assoc_BusinessPartner_Products"/>
</EntityType>
```

This XML is transformed into the following Rust `struct`:

```rust
pub struct BusinessPartnerMetadata {
    pub key: Vec<PropertyRef>,
    pub address: CtAddressMetadata,
    pub business_partner_id: Property,
    pub business_partner_role: Property,
    pub changed_at: Property,
    pub company_name: Property,
    pub created_at: Property,
    pub currency_code: Property,
    pub email_address: Property,
    pub fax_number: Property,
    pub legal_form: Property,
    pub phone_number: Property,
    pub web_address: Property,
}
```

All `<EntityType>` metadata `struct`s have an additional `key` field of type `Vec<PropertyRef>`
    
All fields in a metadata `struct` will either be of type `Property` or of a previously declared complex type `struct`.

## Implementation of Metadata Entity Type `struct`s

Each metadata `struct` for an `<EntityType>` has an implementation containing a getter function for the `key` and a getter function for each `struct` field.

* The `get_key()` function returns a vector of `PropertyRef`
* The field getter functions return either an instance of a `Property` or an instance of some `ComplexType`.

E.G. The implementation of the `BusinessPartnerMetadata` `struct` shown above starts as follows:

```rust
impl BusinessPartnerMetadata {
    pub fn key() -> Vec<PropertyRef> {
        vec![PropertyRef {
                name: "business_partner_id".to_owned(),
            }]
    }

    pub fn get_address() -> ComplexType {
        ComplexType {
            name: "CT_Address".to_owned(),
            properties: vec![
                Property {...}, // City
                Property {...}, // PostalCode
                Property {...}, // Street
                Property {...}, // Building
                Property {...}, // Country
                Property {...}, // AddressType
            ]
        }
    }
    
    pub fn get_business_partner_id() -> Property {
        Property {...}
    }

    // SNIP
}
```

## Metadata for `<Property>` Elements

As can be seen from the example above, all `<EntityType>` elements contain one or more `<Property>` elements.

Within each `<Property>` element, the two most important attributes are `Name` and `Type` as these values are used to define the Rust struct member name and type.

For each struct member of type `Property`, there is a corresponding getter function.
So the XML `<Property>` element called `BusinessPartnerID`:

```xml
<Property Name="BusinessPartnerID" Type="Edm.String" Nullable="false" MaxLength="10" sap:unicode="false" sap:label="Bus. Part. ID" sap:creatable="false" sap:updatable="false"/>
```

will have the following getter method created for it:

```rust
pub fn get_business_partner_id() -> Property {
    Property {
        odata_name: "BusinessPartnerID".to_owned(),
        edm_type: EdmType::Primitive(EdmPrimitive::String),
        nullable: false,
        max_length: Some(10),
        precision: None,
        scale: None,
        concurrency_mode: None,
        fc_keep_in_content: false,
        fc_target_path: None,
        sap_annotations: SAPAnnotationsProperty {
            // SNIP
        },
        deserializer_fn: "".to_owned(),
    }
}
```

As of version 1.6, the `edm_type` member in the `Property` struct is of type `EdmType` rather than being simply of type `String`.

## Representation of EDM Datatypes

### `EdmType` Declaration

All EDM datatypes are declared with the following `enum`:

```rust
pub enum EdmType {
    Primitive(EdmPrimitive),
    Complex(String),
    Unknown(String),
}
```

In any valid OData Metadata XML document, all `<Property Type="some_edm_type" ...>` attribute values should be one of either `EdmType::Primitive` or `EdmType::Complex`.
However, it is always possible that we might encounter a type attribute value that cannot be parsed; therefore, allowance is also made for `EdmType::Unknown`

### `EdmPrimitive` Declaration

The `EdmPrimitive` enum contains a member for each known EDM datatype plus a catchall `Unknown(String)` member.

```rust
pub enum EdmPrimitive {
    Binary,
    Boolean,
    Byte,
    DateTime,
    DateTimeOffset,
    Decimal,
    Double,
    Guid,
    Int16,
    Int32,
    Int64,
    Null,
    SByte,
    Single,
    String,
    Time,
    Unknown(String),
}
```

To provide this enum with bidirectional `string <--> enum type` conversion, both the `std::fmt::Display` and `From<&str>` traits have been implemented
