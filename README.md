# Parse the Metadata from an SAP OData V2 Service

Parses the metadata XML describing an SAP OData V2 service and generates two Rust modules: one for the Service Document and one for the metadata document.

The code generated by this crate is designed to work in conjunction with the crate [`parse-sap-atom-feed`](https://crates.io/crates/parse-sap-atom-feed).

## Available Functionality

1. **Module representing the OData service document.**

   * The metadata description of the OData service is used to declare for all the `struct`s and `enum`s needed to interact with your chosen OData service.
   
   * For each OData Collection, a Rust `struct` is generated to represent a row of that collection, then the entire collection is represented as a `Vec` of that generated `struct`.
      * `Edm.DateTime` fields are transformed into `chrono::NaiveDateTime` using a custom deserializer.
      * `Edm.Decimal` fields transformed into `rust_decimal::Decimal`<br>However, this transformation can only account for the declared number of decimal places (defined by the `Scale` property).  The number of decimal digits (defined by the `Precision` property) is ignored as `rust_decimal::Decimal` values use a hard-coded precision of 64.

   * The generated module declares external crate dependencies on `quick_xml` and `serde` and, depending on whether `Edm.DateTime` and `Edm.Decimal` properties are encountered, external dependencies on `chrono` and `rust_decimal`.

1. **Module representing the OData `$metadata` document.**

   * For each `EntityType` occurring in the metadata:
      * A `struct` is created using the `EntityType`'s name followed by `Metadata`.
         E.G. `<EntityType Name="BusinessPartner">` generates `pub struct BusinessPartnerMetadata`
      * An implementation is created containing a `key()` function and getter function for each `struct` property that returns an instance of a `Property` object.

   * The `Name` property of each `<Association>` metadata tag is stripped of the `Assoc_` prefix and added as a member to the Associations `enum`.
   * The `Name` property of each `<AssociationSet>` metadata tag is stripped of the `Assoc_` prefix and the `_AssocSet` suffix and added as a member to the AssociationSets `enum`.
   * For each `<Association>` or `<AssociationSet>` `enum`, there is an implementation containing a getter function for each Association(Set).
   * The generated module declares an external crate dependency on `parse_sap_atom_feed`.

## Table of Contents

* [Usage](./docs/usage.md)
* [OData Complex Types](./docs/complex_types.md)
* [Metadata Module](./docs/metadata.md)
* [EntitySets Enum](./docs/entitysets_enum.md)
* [Limitations and Issues](./docs/limitations.md)

## Change Log

| Version | Task    | Description                                                                                              |
|--------:|---------|----------------------------------------------------------------------------------------------------------|
|   1.4.7 | Chore   | Add categories to `Cargo.toml`                                                                           |
|   1.4.6 | Chore   | Bump version of `parse-sap-atom-feed` dependency                                                         |
|   1.4.5 | Fix     | Correct failing test                                                                                     |
|   1.4.4 | Fix     | Split declaration of external crate dependencies into known and dynamically derived                      |
|   1.4.3 | Fix     | Add declarations for external crate dependencies                                                         |
|   1.4.2 | Fix     | Depend on AtomLink definition in `parse-sap-atom-feed`<br>Gate call to `rustfmt` behind `parser` feature |
|   1.4.1 | Fix     | Remove duplicate code and update docs                                                                    |
|   1.4.0 | Feature | Generate code to use custom `rust_decimal::Decimal` parser in `parse-sap-atom-feed`                      |
|   1.3.6 | Chore   | Refactor parser generation code                                                                          |
|   1.3.5 | Fix     | Correct use declaration for feature                                                                      |
|   1.3.4 | Chore   | Improve and expand tests                                                                                 |
|   1.3.3 | Chore   | Internal refactoring to improve architectural consistency                                                |
|   1.3.2 | Feature | Generate metadata for `Association`s and `AssociationSet`s                                               |
|   1.3.1 | Chore   | Internal optimisation and refactoring.<br>No changes to functionality                                    |
|   1.3.0 | Feature | Generate a metadata module                                                                               |
|   1.2.5 | Chore   | Update `Cargo.toml` dependency versions                                                                  |
|   1.2.4 | Feature | Add `get_key()` function to `EntityType` trait                                                           |
