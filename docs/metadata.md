# Metadata Module

The entire metadata document is parsed, but only the following entities are transformed into Rust source code and written to the generated metadata module:

* [`<EntityType>`](./entity_type.md)
* [`<ComplexType>`](./complex_type.md)
* [`<Association>` and `<AssociationSet>`](./associations.md)

The following entities are not transformed into Rust source code:

* `<FunctionImport>`
* `<NavigationProperty>`

