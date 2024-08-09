# Parse the Metadata from an SAP OData V2 Service

***This is a work in progress!***

Parses the metadata XML describing an SAP OData V2 service and generates two Rust modules: one for the Service Document and one for the metadata document.

## Available Functionality

1. Creates a module representing the OData service document.

   This contains a set of `struct`s for holding `<ComplexType>` and `<EntityType>` data returned from the OData service.

   `Edm.DateTime` fields are transformed into `chrono::NaiveDateTime` using a custom deserializer.

   `Edm.Decimal` fields are handled using the `Decimal` deserializer in crate `rust_decimal`; however, this offers only partial support

1. Creates a module representing the OData `$metadata` document.

   The metadata XML tags are mapped as follows:

   | XML                                                                 | Rust Declaration
   |---------------------------------------------------------------------|---
   | `<ComplexType Name="CT_Blah">`                                      | `pub struct CtBlahMetadata {`&hellip;`}`
   | `<EntityType Name="Blah">`                                          | `pub struct BlahMetadata {`&hellip;`}`
   | `<Association Name="Assoc_ThisOne_ThatOne">`                        | Creates an Association `enum` member `ThisOneThatOne`
   | `<AssociationSet Name="Assoc_ThisOne_ThatOne_AssocSet" `&hellip;`>` | Creates an AssociationSet `enum` member `ThisOneThatOne`

   For each `EntityType` metadata `struct`, there is an implementation containing a `key()` function and getter function for each `struct` property.
   Each property getter functions returns an instance of a `Property` struct that includes SAP annotations.

   The `Name` property of each `<Association>` metadata tag is stripped of the `Assoc_` prefix and added as a member to the Associations `enum`.
   This `enum` has an implementation containing a getter function for each association.

   The `Name` property of each `<AssociationSet>` metadata tag is stripped of the `Assoc_` prefix and the `_AssocSet` suffix and added as a member to the AssociationSets `enum`.
   This `enum` has an implementation containing a getter function for each association set.

## Table of Contents

* [Usage](./docs/usage.md)
* [OData Complex Types](./docs/complex_types.md)
* [Metadata Module](./docs/metadata.md)
* [EntitySets Enum](./docs/entitysets_enum.md)
* [Limitations and Issues](./docs/limitations.md)

## Change Log

| Version | Description
|--:|---
1.3.3 | Internal refactoring to improve architectural consistency<br>No changes to functionality
1.3.2 | Generate metadata for `Association`s and `AssociationSet`s
1.3.1 | Internal optimisation and refactoring.<br>No changes to functionality
1.3.0 | Generate a metadata module
1.2.5 | Update `Cargo.toml` dependency versions
1.2.4 | Add `get_key()` function to `EntityType` trait
