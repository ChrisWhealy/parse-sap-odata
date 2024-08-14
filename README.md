# Parse the Metadata from an SAP OData V2 Service

Parses the metadata XML describing an SAP OData V2 service and generates two Rust modules: one for the Service Document and one for the metadata document.

The code generated by this crate is designed to work in conjunction with the crate [`parse-sap-atom-feed`](https://crates.io/crates/parse-sap-atom-feed).

## Available Functionality

1. **Module representing the OData service document.**

   The metadata description of the OData service is used to declare for all the `struct`s needed to interact with your chosen OData service.
   
   For each OData Collection, a Rust `struct` is generated to represent a row of that collection, then the entire collection is represented as a `Vec` of that generated `struct`.
   
   `Edm.DateTime` fields are transformed into `chrono::NaiveDateTime` using a custom deserializer.

   `Edm.Decimal` fields are handled using the `Decimal` deserializer in crate `rust_decimal`; however, this can only support some of properties used by SAP.

1. **Module representing the OData `$metadata` document.**

   The metadata description of the OData service is also used to declare the `struct`s that describe the service's metadata.

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

| Version | Task    | Description
|--:|---------|---
1.3.6 | Chore   | Refactor parser generation code
1.3.5 | Fix     | Correct use declaration for feature
1.3.4 | Chore   | Improve and expand tests
1.3.3 | Chore   | Internal refactoring to improve architectural consistency
1.3.2 | Feature | Generate metadata for `Association`s and `AssociationSet`s
1.3.1 | Chore   | Internal optimisation and refactoring.<br>No changes to functionality
1.3.0 | Feature | Generate a metadata module
1.2.5 | Chore   | Update `Cargo.toml` dependency versions
1.2.4 | Feature | Add `get_key()` function to `EntityType` trait
