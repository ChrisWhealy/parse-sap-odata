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

