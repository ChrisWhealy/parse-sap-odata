# Limitations and Issues

1. Currently when generating a Rust `struct`, only the `Name` and `Type` properties are extracted from the XML `<EntityType>` declaration.
  Consider how the other XML attribute values and SAP annotations could be made available within the generated Rust `struct`.

1. The `<FunctionImport>`, `<Association>` and `<AssociationSet>` metadata tags are parsed, but their contents is not currently acted upon.

1. All [SAP OData V2 Annotations](https://sap.github.io/odata-vocabularies/docs/v2-annotations.html) are processed by [serde](https://serde.rs); however, no action is yet taken based on the annotation values.

1. When calling some of the entity sets in the demo OData service `GWSAMPLE_BASIC`, certain XML properties are returned whose values are not valid XML.
   Consequently, when `quick_xml` attempts to parse such values, it simply throws its toys out the pram and doesn't want to play anymore.

   If you encounter such errors, the raw XML string must first be sanitised before attempting to parse it.

   This functionality is described in the [README of `parse-sap-odata-demo`](https://github.com/lighthouse-no/parse-sap-odata-demo).

1. The `rust_decimal::serde::str` deserializer can offer only partial support for fields of `Edm.Decimal` because it knows nothing about the attributes `Precision` and `Scale`:

   ```xml
   <Property Name="Price" Type="Edm.Decimal" Precision="16" Scale="3" sap:unicode="false" sap:unit="CurrencyCode" sap:label="Unit Price"/>
   ```

   Consequently, these attribute values are not considered when extracting a decimal value.
