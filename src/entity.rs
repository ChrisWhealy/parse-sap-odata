// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Implemented by every generated OData entity-set struct.
/// Provides the ordered list of field names as declared in the OData metadata document.
pub trait ODataEntity {
    fn field_names() -> &'static [&'static str];
}
