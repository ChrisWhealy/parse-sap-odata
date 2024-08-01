# Metadata Module

Most of the metadata captured from the OData `$metadata` HTTP request is output to a separate module.

Currently, the metatdata information for `<FunctionImport>`, `<NavigationProperty>`, `<Association>` and `<AssociationSet>` is parsed, but is not output to the generated metadata module.

## Metadata for Complex Type `struct`s

For each metadata `<ComplexType>` containing more than one `<Property>`, a corresponding `struct` is created.

E.G. The metadata XML for an `Address` complex type could be;

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

This is then translated to the following Rust `struct`;
 
```rust   
pub struct CtAddressMetadata {
    pub address_type: Property,
    pub building: Property,
    pub city: Property,
    pub country: Property,
    pub postal_code: Property,
    pub street: Property,
}
```
    
All fields within a complex type metadata `struct` are of type `Property`.

    
## Metadata for Entity Type `struct`s

One or more `struct`s for each metadata `<EntityType>`.

E.G. The metadata XML for `BusinessPartner`

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

Is translated into the following Rust `struct`:

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
    
The fields in this `struct` will only ever be of type `Property` or of a previously declared complex type `struct`.

All `<EntityType>` metadata `struct`s have a `key` field of type `Vec<PropertyRef>`

## Implementation of Metadata Entity Type `struct`s

Each metadata `struct` for an `<EntityType>` has an implementation containing a getter function for the `key` and a getter function for each `struct` field.

The `get_key()` function returns an instance of `Vec<PropertyRef>`

The field getter functions return either an instance of a `Property` or an instance of `ComplexType`.

E.G. The implementation of the `BusinessPartnerMetadata` `struct` shown above starts as follows:

```rust
impl BusinessPartnerMetadata {
    pub fn get_key() -> Vec<PropertyRef> {
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
