# Parse the Metadata from an SAP OData V2 Service

***This is a work in progress!***

Parses the metadata XML describing an SAP OData V2 service and generates two Rust modules: one for the Service Document and one for the metadata document.

## Available Functionality

1. Creates a module representing the OData service document.

   This contains a set of `struct`s for holding `<ComplexType>` and `<EntityType>` data returned from the OData service.

   `Edm.DateTime` fields are transformed into `chrono::NaiveDateTime` using a custom deserializer.

   `Edm.Decimal` fields are handled using the `Decimal` deserializer in crate `rust_decimal`; however, this offers only partial support

1. Creates a module representing the OData `$metadata` document.

   This contains a set of `structs` that hold the `<ComplexType>` and `<EntityType>` metadata.

   Each metadata `struct` has an `impl` containing a getter function for each `struct` property that returns an instance of that property's metadata, including any SAP annotations.

## Table of Contents

* [Usage](./docs/usage.md)
* [OData Complex Types](./docs/complex_types.md)
* [Metadata Module](./docs/metadata.md)
* [EntitySets Enum](./docs/entitysets_enum.md)
* [Limitations and Issues](./docs/limitations.md)

## Change Log

| Version | Description
|--:|---
1.3.1 | Internal optimisation and refactoring.  No changes to functionality
1.3.0 | Generate a metadata module
1.2.5 | Update `Cargo.toml` dependency versions
1.2.4 | Add `get_key()` function to `EntityType` trait
