# Parse the Metadata from an SAP OData V2 Service

***This is a work in progress!***

Parses the metadata XML describing an SAP OData V2 service and generates two Rust modules: one for the Service Document and one for the metadata document.

## Available Functionality

* [x] `<ComplexType>` and `<EntityType>` elements are mapped to Rust `structs`
* [x] Transforms `Edm.DateTime` into `chrono::NaiveDateTime` using a custom deserializer
* [x] `Edm.Decimal` fields are handled using the `Decimal` deserializer in crate `rust_decimal`; however, this offers only partial support
* [ ] `<FunctionImport>` functionality will be supported in time, but is not currently available
* [ ] The metadata module is currently empty and needs to be populated - I'm working on it...

## Table of Contents

* [Usage](./docs/usage.md)
* [OData Complex Types](./docs/complex_types.md)
* [EntitySets Enum](./docs/entitysets_enum.md)
* [Limitations and Issues](./docs/limitations.md)

## TODOs

1. Populate the empty OData metadata module.
2. Improve support for fields of type `Edm.Decimal`.
3. Support Function Imports.

## Change Log

| Version | Description
|--:|---
1.2.5 | Update `Cargo.toml` dependency versions
1.2.4 | Add `get_key()` function to `EntityType` trait