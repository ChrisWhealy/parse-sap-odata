use crate::{
    parser::{
        generate::syntax_fragments::{CRATE_CHRONO, CRATE_GUID, CRATE_RUST_DECIMAL},
        AsRustSrc,
    },
    property::{edm_primitive::EdmPrimitive, edm_type::EdmType, Property},
    test_utils::*,
};

use rust_decimal::{serde::str, Decimal};
use serde::Deserialize;
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl FromStr for Property {
    type Err = quick_xml::de::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

fn property_xml(name: &str, edm_type: &str) -> String {
    format!(r#"<Property Name="{name}" Type="{edm_type}"/>"#)
}

fn property_xml_non_nullable(name: &str, edm_type: &str) -> String {
    format!(r#"<Property Name="{name}" Type="{edm_type}" Nullable="false"/>"#)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_parse_edm_binary_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("SomeBinary", "Edm.Binary")).unwrap();
    handle_test_comparison(&prop.odata_name, &"SomeBinary".to_string())?;
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Binary))
}

#[test]
fn should_parse_edm_boolean_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("IsActive", "Edm.Boolean")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Boolean))
}

#[test]
fn should_parse_edm_byte_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("TaxCode", "Edm.Byte")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Byte))
}

#[test]
fn should_parse_edm_datetime_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("CreatedAt", "Edm.DateTime")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::DateTime))
}

#[test]
fn should_parse_edm_datetimeoffset_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("ChangedAt", "Edm.DateTimeOffset")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::DateTimeOffset))
}

#[test]
fn should_parse_edm_decimal_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Price", "Edm.Decimal")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Decimal))
}

#[test]
fn should_parse_edm_double_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Ratio", "Edm.Double")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Double))
}

#[test]
fn should_parse_edm_guid_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("EntityId", "Edm.Guid")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Guid))
}

#[test]
fn should_parse_edm_int16_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("SmallCount", "Edm.Int16")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Int16))
}

#[test]
fn should_parse_edm_int32_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Count", "Edm.Int32")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Int32))
}

#[test]
fn should_parse_edm_int64_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("LargeCount", "Edm.Int64")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Int64))
}

#[test]
fn should_parse_edm_null_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Nothing", "Edm.Null")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Null))
}

#[test]
fn should_parse_edm_sbyte_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("SignedByte", "Edm.SByte")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::SByte))
}

#[test]
fn should_parse_edm_single_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("FloatVal", "Edm.Single")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Single))
}

#[test]
fn should_parse_edm_string_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Name", "Edm.String")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::String))
}

#[test]
fn should_parse_edm_time_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Duration", "Edm.Time")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Primitive(EdmPrimitive::Time))
}

#[test]
fn should_parse_unknown_edm_primitive_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Weird", "Edm.NonsenseType")).unwrap();
    handle_test_comparison(
        &prop.edm_type,
        &EdmType::Primitive(EdmPrimitive::Unknown("NonsenseType".to_owned())),
    )
}

#[test]
fn should_parse_bare_complex_type_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Location", "CT_Address")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Complex("CtAddress".to_owned()))
}

#[test]
fn should_parse_namespace_qualified_complex_type_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Address", "GWSAMPLE_BASIC.CT_Address")).unwrap();
    handle_test_comparison(&prop.edm_type, &EdmType::Complex("CtAddress".to_owned()))
}

#[test]
fn should_parse_unknown_type_property() -> Result<(), String> {
    let prop = Property::from_str(&property_xml("Opaque", "SomeNamespace.UnknownType")).unwrap();
    handle_test_comparison(
        &prop.edm_type,
        &EdmType::Unknown("SomeNamespace.UnknownType".to_owned()),
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement {
    #[serde(
        deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_3dp"
    )]
    price: Decimal,
}

impl FromStr for DecimalElement {
    type Err = quick_xml::de::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement {
    #[serde(
        deserialize_with = "parse_sap_atom_feed::deserializers::edm_decimal::to_rust_decimal_3dp_opt"
    )]
    price: Option<Decimal>,
}

impl FromStr for OptionalDecimalElement {
    type Err = quick_xml::de::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_decimal() -> Result<(), String> {
    let test_price_i64: i64 = 1234560;
    let test_price_str= "1234.56";
    let price = Decimal::new(test_price_i64, 3);
    let price_xml = &format!("<Test><d:Price>{}</d:Price></Test>", test_price_str);

    match DecimalElement::from_str(price_xml) {
        Ok(result) => {
            handle_test_comparison(&result.price,&price)?;
            handle_test_comparison(&result.price.to_string(), &"1234.560".to_string())
        },
        Err(err) => Err(err.to_string()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_optional_decimal() -> Result<(), String> {
    let test_price_i64: i64 = 1234560;
    let test_price_str= "1234.56";
    let price = Decimal::new(test_price_i64, 3);
    let price_xml = &format!("<Test><d:Price>{}</d:Price></Test>", test_price_str);
    let zero_price_xml = "<Test><d:Price>0</d:Price></Test>";

    match OptionalDecimalElement::from_str(price_xml) {
        Ok(result) => {
            handle_test_comparison_opt(&result.price, &Some(price))?;
            handle_test_comparison(&result.price.unwrap().to_string(), &"1234.560".to_string())?;
        },
        Err(err) => return Err(err.to_string()),
    }

    match OptionalDecimalElement::from_str(zero_price_xml) {
        Ok(result) => {
            handle_test_comparison_opt(&result.price, &Some(Decimal::new(0,3)))?;
            handle_test_comparison(&result.price.unwrap().to_string(), &"0.000".to_string())
        },
        Err(err) => Err(err.to_string()),
    }
}

#[test]
fn should_handle_empty_decimal() -> Result<(), String> {
    let empty_price_xml = "<Test><d:Price /></Test>";
    let price = OptionalDecimalElement::from_str(empty_price_xml).unwrap().price;
    handle_test_comparison(&price.unwrap().to_string(), &"0.000".to_string())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_convert_unsafe_property_name() -> Result<(), String> {
    use check_keyword::CheckKeyword;
    let kw1 = "type";
    let kw2 = "match";

    handle_test_bool(kw1.is_keyword())?;
    handle_test_bool(kw2.is_keyword())?;
    handle_test_comparison(&kw1.into_safe(), &"r#type".to_string())?;
    handle_test_comparison(&kw2.into_safe(), &"r#match".to_string())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_not_convert_safe_property_name() -> Result<(), String> {
    use check_keyword::CheckKeyword;
    let kw = "wibble";

    handle_test_bool(!kw.is_keyword())?;
    handle_test_comparison(&kw.into_safe(), &"wibble".to_string())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_convert_edm_string() -> Result<(), String> {
    let prop = Property::from_str(&property_xml_non_nullable("BusinessPartnerID", "Edm.String")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"business_partner_id".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"String".to_string())?;
    handle_test_bool(has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_binary() -> Result<(), String> {
    let prop = Property::from_str(&property_xml_non_nullable("SomeBinaryStuff", "Edm.Binary")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"some_binary_stuff".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"Vec".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_boolean() -> Result<(), String> {
    let prop = Property::from_str(&property_xml_non_nullable("IsOverMisunderestimatable", "Edm.Boolean")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"is_over_misunderestimatable".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"bool".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_byte() -> Result<(), String> {
    // Byte is never wrapped in Option regardless of nullability
    let prop = Property::from_str(&property_xml("ASingleByte", "Edm.Byte")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"a_single_byte".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"u8".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_datetime() -> Result<(), String> {
    // nullable=true (default) → Option<chrono::NaiveDateTime>
    let prop = Property::from_str(&property_xml("OnceUponAWednesday", "Edm.DateTime")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"once_upon_a_wednesday".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"Option".to_string())?;
    handle_test_bool(has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &CRATE_CHRONO.to_string())
}

#[test]
fn should_convert_edm_datetime_offset() -> Result<(), String> {
    // nullable=true (default) → Option<chrono::NaiveDateTime>
    let prop = Property::from_str(&property_xml("OnceUponAWednesday", "Edm.DateTimeOffset")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"once_upon_a_wednesday".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"Option".to_string())?;
    handle_test_bool(has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &CRATE_CHRONO.to_string())
}

#[test]
fn should_convert_edm_decimal() -> Result<(), String> {
    // nullable=false → rust_decimal::Decimal (not Option)
    let prop = Property::from_str(&property_xml_non_nullable("SomeDecimalNumber", "Edm.Decimal")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"some_decimal_number".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"Decimal".to_string())?;
    handle_test_bool(has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &CRATE_RUST_DECIMAL.to_string())
}

#[test]
fn should_convert_edm_double() -> Result<(), String> {
    // Double is never wrapped in Option regardless of nullability
    let prop = Property::from_str(&property_xml_non_nullable("DoubleTrouble", "Edm.Double")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"double_trouble".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"f64".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_guid() -> Result<(), String> {
    // Guid is never wrapped in Option regardless of nullability
    let prop = Property::from_str(&property_xml_non_nullable("GooeyGuid", "Edm.Guid")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"gooey_guid".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"Uuid".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &CRATE_GUID.to_string())
}

#[test]
fn should_convert_edm_int16() -> Result<(), String> {
    // nullable=true (default) → Option<i16>
    let prop = Property::from_str(&property_xml("MaybeAnInt16", "Edm.Int16")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"maybe_an_int_16".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"Option".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_int32() -> Result<(), String> {
    // nullable=true (default) → Option<i32>
    let prop = Property::from_str(&property_xml("MaybeAnInt32", "Edm.Int32")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"maybe_an_int_32".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"Option".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_int64() -> Result<(), String> {
    // nullable=true (default) → Option<i64>
    let prop = Property::from_str(&property_xml("MaybeAnInt64", "Edm.Int64")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"maybe_an_int_64".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"Option".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_null() -> Result<(), String> {
    // Null always maps to unit type ()
    let prop = Property::from_str(&property_xml_non_nullable("NothingToSeeHere", "Edm.Null")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"nothing_to_see_here".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"()".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_sbyte() -> Result<(), String> {
    let prop = Property::from_str(&property_xml_non_nullable("SingleByte", "Edm.SByte")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"single_byte".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"i8".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_float_single() -> Result<(), String> {
    // Single is never wrapped in Option regardless of nullability
    let prop = Property::from_str(&property_xml_non_nullable("SinglePrecisionFloat", "Edm.Single")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"single_precision_float".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"f32".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_edm_time() -> Result<(), String> {
    let prop = Property::from_str(&property_xml_non_nullable("WhatsTheTimeEccles", "Edm.Time")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(
        &field.ident.as_ref().unwrap().to_string(),
        &"whats_the_time_eccles".to_string(),
    )?;
    handle_test_comparison(&outer_type_name(&field)?, &"SystemTime".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_nonsense_type() -> Result<(), String> {
    // Unknown EDM primitive falls through to String
    let prop = Property::from_str(&property_xml_non_nullable("WhatIsIt", "Edm.ImATeapot")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"what_is_it".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"String".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}

#[test]
fn should_convert_unknown_type() -> Result<(), String> {
    // Completely unrecognised type: emitted as-is as the field type name
    let prop = Property::from_str(&property_xml_non_nullable("ThatsWeird", "NotSeenThisBefore")).unwrap();
    let (src, crate_ref) = prop.to_rust();
    let field = parse_field(&src)?;
    handle_test_comparison(&field.ident.as_ref().unwrap().to_string(), &"thats_weird".to_string())?;
    handle_test_comparison(&outer_type_name(&field)?, &"NotSeenThisBefore".to_string())?;
    handle_test_bool(!has_serde_attr(&field))?;
    handle_test_comparison(&crate_ref, &"".to_string())
}
