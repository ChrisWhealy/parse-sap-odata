use rust_decimal::{serde::str, Decimal};
use serde::Deserialize;
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement {
    #[serde(with = "rust_decimal::serde::str")]
    price: Decimal,
}

impl std::str::FromStr for DecimalElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement {
    #[serde(with = "rust_decimal::serde::str_option")]
    price: Option<Decimal>,
}

impl std::str::FromStr for OptionalDecimalElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_decimal() {
    let test_price = "1234.56";
    let price = Decimal::from_str(test_price).unwrap();
    let price_xml = &format!("<Test><d:Price>{}</d:Price></Test>", test_price);

    match DecimalElement::from_str(price_xml) {
        Ok(result) => assert_eq!(price, result.price),
        Err(err) => assert!(false, "{}", err),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_optional_decimal() {
    let test_price = "1234.56";
    let price = Decimal::from_str(test_price).unwrap();
    let price_xml = &format!("<Test><d:Price>{}</d:Price></Test>", price);
    let zero_price_xml = "<Test><d:Price>0</d:Price></Test>";

    match OptionalDecimalElement::from_str(price_xml) {
        Ok(result) => assert_eq!(result.price, Some(price)),
        Err(err) => assert!(false, "{}", err),
    }

    match OptionalDecimalElement::from_str(zero_price_xml) {
        Ok(result) => assert_eq!(result.price, Some(Decimal::from(0))),
        Err(err) => assert!(false, "{}", err),
    }
}

#[test]
#[should_panic]
fn should_panic_on_empty_decimal() {
    let empty_price_xml = "<Test><d:Price /></Test>";

    OptionalDecimalElement::from_str(empty_price_xml).unwrap();
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_convert_unsafe_property_name() {
    use check_keyword::CheckKeyword;
    let kw1 = "type";
    let kw2 = "match";

    assert!(kw1.is_keyword());
    assert!(kw2.is_keyword());
    assert_eq!(kw1.into_safe(), "r#type");
    assert_eq!(kw2.into_safe(), "r#match");
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_not_convert_safe_property_name() {
    use check_keyword::CheckKeyword;
    let kw = "wibble";

    assert!(!kw.is_keyword());
    assert_eq!(kw.into_safe(), "wibble");
}
