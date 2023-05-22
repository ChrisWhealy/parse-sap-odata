# Parse the Metadata from an SAP OData Service

First attempt at consuming the metadata from an SAP Odata service.

This is a work in progress

## Testing

`git clone https://github.com/lighthouse-no/parse-sap-odata`

Running the tests doesn't actually test anything in the normal sense; instead, I've used a test simply as a framework for parsing the metadata and writing the output to a text file.

`cargo test -- --nocapture`.

The parsed output will appear in the `./parsed` directory as one `.txt` file per XML file.
