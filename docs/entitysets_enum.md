# Entitysets Enum

On the basis that a single OData service exposes a static list of entity sets, and that within the scope of any single request, you will only ever be interacting with a single entity set, it makes sense to treat each entity set name as an `enum` variant.

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

## Convenience Functions

Three convenience functions are then implemented for `enum GwsampleBasicEntities`:

```rust
impl GwsampleBasicEntities {
    pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> { /* SNIP */ }
    pub const fn variant_name(&self) -> &'static str { /* SNIP */ }
    pub fn variant_names() -> Vec<&'static str> { /* SNIP */ }
}
```

## Entitysets Enum `iterator` function

For standard Rust `enums` such as `Option` and `Result`, it makes little sense to attempt to loop over their variants simply because these `enum`s exist specifically to gather together diverse types into a single object.
E.G. The `Option` `enum` exists to provide a type-safe mechanism for handling the possibility that a variable might not contain a value.

However, an OData service guarantees that the entity set names form an immutable, type-safe list.
Therefore, on the basis of this guarantee, the entity set names are placed into an `enum` that implements an `iterator` function over its variants.

```rust
pub fn iterator() -> impl Iterator<Item = GwsampleBasicEntities> {
    [
        GwsampleBasicEntities::BusinessPartnerSet,
        GwsampleBasicEntities::ProductSet,
        GwsampleBasicEntities::SalesOrderSet,
        GwsampleBasicEntities::SalesOrderLineItemSet,
        GwsampleBasicEntities::ContactSet,
        GwsampleBasicEntities::VhSexSet,
        GwsampleBasicEntities::VhCountrySet,
        GwsampleBasicEntities::VhAddressTypeSet,
        GwsampleBasicEntities::VhCategorySet,
        GwsampleBasicEntities::VhCurrencySet,
        GwsampleBasicEntities::VhUnitQuantitySet,
        GwsampleBasicEntities::VhUnitWeightSet,
        GwsampleBasicEntities::VhUnitLengthSet,
        GwsampleBasicEntities::VhProductTypeCodeSet,
        GwsampleBasicEntities::VhBpRoleSet,
        GwsampleBasicEntities::VhLanguageSet,
    ]
    .iter()
    .copied()
}
```

## Entitysets Enum `variant_name` function

This function returns the name of the entity set variant as a static string slice:

```rust
pub const fn variant_name(&self) -> &'static str {
    match *self {
        GwsampleBasicEntities::BusinessPartnerSet => "BusinessPartnerSet",
        GwsampleBasicEntities::ProductSet => "ProductSet",
        GwsampleBasicEntities::SalesOrderSet => "SalesOrderSet",
        GwsampleBasicEntities::SalesOrderLineItemSet => "SalesOrderLineItemSet",
        GwsampleBasicEntities::ContactSet => "ContactSet",
        GwsampleBasicEntities::VhSexSet => "VH_SexSet",
        GwsampleBasicEntities::VhCountrySet => "VH_CountrySet",
        GwsampleBasicEntities::VhAddressTypeSet => "VH_AddressTypeSet",
        GwsampleBasicEntities::VhCategorySet => "VH_CategorySet",
        GwsampleBasicEntities::VhCurrencySet => "VH_CurrencySet",
        GwsampleBasicEntities::VhUnitQuantitySet => "VH_UnitQuantitySet",
        GwsampleBasicEntities::VhUnitWeightSet => "VH_UnitWeightSet",
        GwsampleBasicEntities::VhUnitLengthSet => "VH_UnitLengthSet",
        GwsampleBasicEntities::VhProductTypeCodeSet => "VH_ProductTypeCodeSet",
        GwsampleBasicEntities::VhBpRoleSet => "VH_BPRoleSet",
        GwsampleBasicEntities::VhLanguageSet => "VH_LanguageSet",
    }
}
```

## Entitysets Enum `variant_names` function

By making use of the above `iterator` and `bvariant_name` functions, the `variant_names` function returns the names of the entity sets as a vector of string slices.

```rust
pub fn variant_names() -> Vec<&'static str> {
    GwsampleBasicEntities::iterator().fold(Vec::new(), |mut acc: Vec<&'static str>, es| {
        acc.push(&mut es.variant_name());
        acc
    })
}
```
