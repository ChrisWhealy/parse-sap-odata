# Parse the Metadata from an SAP OData Service

This is an absolute bare-minimum rough implementation of the EDMX 1.0 format for parsing using serde/quick-xml into a Rust structure.

## Testing

`git clone `

`cargo test`

## Example

Parse an example `metadata.xml` file, and print all the `EntitySets` within the default schema.
```rust
let edmx = Edmx::from_str(include_str!("my-metadata.xml")).unwrap();
let schema = edmx.default_schema().unwrap();

for entity_set in schema.entity_sets().unwrap() {
  println!("{:#?}", entity_set);
}
```

Using the [test file](tests/folketinget.xml) from the Danish Parliament, you should see output similar to this:
```
EntitySet {
    name: "Afstemning",
    entity_type: "FT.Domain.Models.Afstemning",
}
EntitySet {
    name: "Afstemningstype",
    entity_type: "FT.Domain.Models.Afstemningstype",
}

(... and so on)
```
