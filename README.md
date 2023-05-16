# Parse the Metadata from an SAP OData Service

First attempt at consuming an the metadata from an SAP Odata service.

## Testing

`git clone https://github.com/lighthouse-no/parse-sap-odata`

To run the tests with the `println!()` output suppressed

`cargo test`

To run the tests with `println!()` output

`cargo test -- --nocapture`

## Example

Parse an example `metadata.xml` file, and print all the `EntitySets` within the default schema.

```rust
let edmx = Edmx::from_str(include_str!("some-odata-metadata.xml")).unwrap();
let schema = edmx.fetch_schema("GWSAMPLE_BASIC").unwrap();

for set in schema.entity_sets().unwrap() {
  println!("{:#?}", set);
}
```

Running the tests will generate output similar this:

```
EntitySet {
    name: "ProductSet",
    entity_type: "GWSAMPLE_BASIC.Product",
}
EntitySet {
    name: "VH_UnitQuantitySet",
    entity_type: "GWSAMPLE_BASIC.VH_UnitQuantity",
}

SNIP...
```
