# Metadata Module

> ***CAVEAT***<br>
> Since Rust does not allow for the dynamic allocation of types at runtime, it remains to be seen how useful this `Association` and `AssociationSet` information is in practice.


Most of the metadata captured from the OData `$metadata` HTTP request is output to a separate module.
That is, all Schema `<EntityType>` and `<ComplexType>` entities are transformed to Rust `struct`s and `enum`s.

The metadata information for the following entities is parsed, but not yet written to the generated metadata module:

* `<FunctionImport>`
* `<NavigationProperty>`

## Metadata for Complex Type `struct`s

For each metadata `<ComplexType>` containing more than one `<Property>`, a corresponding metadata `struct` is created.

E.G. In the `GWSAMPLE_BASIC` service, the metadata XML for the complex type `CT_Address`is:

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

This is then translated to Rust `struct` whose name ends with `Metadata`:
 
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

One or more `struct`s are created for each metadata `<EntityType>`.

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

This is translated into the following Rust `struct`:

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
    
The fields in this `struct` will only ever be of type `Property` or of a previously declared complex type `struct`.

## Implementation of Metadata Entity Type `struct`s

Each metadata `struct` for an `<EntityType>` has an implementation containing a getter function for the `key` and a getter function for each `struct` field.

* The `get_key()` function returns a vector of `PropertyRef`
* The field getter functions return either an instance of a `Property` or an instance of `ComplexType`.

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

## Metadata for Associations

An `Association` describes the relationship between two `EntityTypes`.

Using the `GWSAMPLE_BASIC` OData service as an example, there is an association called `Assoc_VH_Country_BusinessPartners` between the entity types `BusinessPartner` and `Country`.

All associations have exactly two `End` objects, each of which describes the "From" and "To" sides of the association.

In this case, one `Country` is related to many `BusinessPartners`

```rust
pub fn vh_country_business_partners() -> Association {
    Association {
        name: "Assoc_VH_Country_BusinessPartners".to_owned(),
        sap_content_version: "1".to_owned(),
        ends: [
            End {
                role: "FromRole_Assoc_VH_Country_BusinessPartners".to_owned(),
                entity_set: None,
                end_type: Some("VhCountry".to_owned()),
                multiplicity: Some("1".to_owned()),
            },
            End {
                role: "ToRole_Assoc_VH_Country_BusinessPartners".to_owned(),
                entity_set: None,
                end_type: Some("BusinessPartner".to_owned()),
                multiplicity: Some("*".to_owned()),
            },
        ],
        referential_constraint: None,
    }
}
```

Since this is a simple relationship, there is no need to define a referential constraint.

### Referential Constraints

A referential constraint describes a foreign key relationship.
This is the situation in which the set of permissible values for a non-key field in one `EntityType` is defined (or constrained) by the key values found in some other `EntityType`.

For example, the referential constraint in association `Assoc_VH_ProductTypeCode_Products` describes the fact that the non-key field `Product.type_code` may only use a value found in the `EntitySet` field `VhProductTypeCode.type_code`.

```rust
pub fn vh_product_type_code_products() -> Association {
    Association {
        // SNIP

        referential_constraint: Some(ReferentialConstraint {
            principal: Principal {
                role: "FromRole_Assoc_VH_ProductTypeCode_Products".to_owned(),
                property_refs: vec![PropertyRef {
                    name: "type_code".to_owned(),
                }],
            },
            dependent: Dependent {
                role: "ToRole_Assoc_VH_ProductTypeCode_Products".to_owned(),
                property_refs: vec![PropertyRef {
                    name: "type_code".to_owned(),
                }],
            },
        }),
    }
}
```

There is no requirement for the "From" and "To" field names to be the same.
This can be seen in cases where an `EntitySet` containing the German field name `waers` (for currency code) is linked to the field name `currency_code` in some other `EntitySet`.

## Metadata for AssociationSets

In the same way that an `Association` describes the relationship between two `EntityType`s, an `AssociationSet` describes the relationship between two `EntitySet`s.

