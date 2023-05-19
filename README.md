# Parse the Metadata from an SAP OData Service

First attempt at consuming the metadata from an SAP Odata service.

This is a work in progress

## Testing

`git clone https://github.com/lighthouse-no/parse-sap-odata`

The tests don't actually test anything; instead, I've used a test simply as a framework for parsing the metadata and writing the output to a text file.

`cargo test -- --nocapture > ./tests/parsed_output.txt`.

When parsing the OData service `GWSAMPLE_BASIC`, the file `parsed_output.txt` will be several thousand lines long...
