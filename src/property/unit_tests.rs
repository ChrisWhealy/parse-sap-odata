use crate::{
    parser::AsRustSrc,
    property::Property,
    sap_annotations::property::SAPAnnotationsProperty,
    test_utils::*
};

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

impl FromStr for DecimalElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement {
    #[serde(with = "rust_decimal::serde::str_option")]
    price: Option<Decimal>,
}

impl FromStr for OptionalDecimalElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_decimal() -> Result<(), String> {
    let test_price = "1234.56";
    let price = Decimal::from_str(test_price).unwrap();
    let price_xml = &format!("<Test><d:Price>{}</d:Price></Test>", test_price);

    match DecimalElement::from_str(price_xml) {
        Ok(result) => handle_test_comparison(&price, &result.price),
        Err(err) => Err(err.to_string()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_optional_decimal() -> Result<(), String> {
    let test_price = "1234.56";
    let price = Decimal::from_str(test_price).unwrap();
    let price_xml = &format!("<Test><d:Price>{}</d:Price></Test>", price);
    let zero_price_xml = "<Test><d:Price>0</d:Price></Test>";

    match OptionalDecimalElement::from_str(price_xml) {
        Ok(result) => handle_test_comparison_opt(&result.price, &Some(price))?,
        Err(err) => return Err(err.to_string()),
    }

    match OptionalDecimalElement::from_str(zero_price_xml) {
        Ok(result) => handle_test_comparison_opt(&result.price, &Some(Decimal::from(0))),
        Err(err) => Err(err.to_string()),
    }
}

#[test]
fn should_handle_empty_decimal() -> Result<(), String> {
    let empty_price_xml = "<Test><d:Price /></Test>";
    handle_test_bool(OptionalDecimalElement::from_str(empty_price_xml).unwrap().price.is_none())
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
fn gen_property_of_type(prop_name: &str, prop_type: &str, is_nullable: bool) -> Property {
    let dummy_annotations = SAPAnnotationsProperty {
        label: Some("Dummy label".to_string()),
        heading: Some("Dummy heading".to_string()),
        quick_info: Some("Nothing to see here, move along".to_string()),
        is_unicode: false,
        semantics: None,
        is_creatable: false,
        is_updatable: true,
        is_sortable: false,
        is_filterable: true,
        is_addressable: false,
        is_required_in_filter: false,
        filter_restriction: None,
        filter_for: None,
        text: None,
        text_for: None,
        unit: None,
        precision: None,
        is_visible: false,
        field_control: None,
        validation_regexp: None,
        display_format: None,
        value_list: None,
        lower_boundary: None,
        upper_boundary: None,
        aggregation_role: None,
        super_ordinate: None,
        attribute_for: None,
        hierarchy_node_for: None,
        hierarchy_node_external_key_for: None,
        hierarchy_level_for: None,
        hierarchy_parent_node_for: None,
        hierarchy_parent_navigation_for: None,
        hierarchy_drill_state_for: None,
        hierarchy_node_descendant_count_for: None,
        hierarchy_preorder_rank_for: None,
        hierarchy_sibling_rank_for: None,
        parameter: None,
        is_annotation: false,
        updatable_path: None,
        preserve_flag_for: None,
        has_variable_scale: false,
    };

    Property {
        odata_name: prop_name.to_string(),
        edm_type: prop_type.to_string(),
        nullable: is_nullable,
        max_length: None,
        precision: None,
        scale: None,
        concurrency_mode: None,
        fc_keep_in_content: true,
        fc_target_path: None,
        sap_annotations: dummy_annotations,
        deserializer_fn: "",
        deserializer_module: "",
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_convert_edm_property_types() -> Result<(), String> {
    let prop = gen_property_of_type("BusinessPartnerID", "Edm.String", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(
        &src_lines[0].to_string(),
        &r#"#[serde(rename = "BusinessPartnerID")]"#.to_string(),
    )?;
    handle_test_comparison(&src_lines[1].to_string(), &"pub business_partner_id:String,".to_string())?;

    let prop = gen_property_of_type("SomeBinaryStuff", "Edm.Binary", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub some_binary_stuff:Vec<u8>,".to_string())?;

    let prop = gen_property_of_type("IsOverMisunderestimatable", "Edm.Boolean", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub is_over_misunderestimatable:bool,".to_string())?;

    let prop = gen_property_of_type("ASingleByte", "Edm.Byte", true);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub a_single_byte:u8,".to_string())?;

    let prop = gen_property_of_type("OnceUponAWednesday", "Edm.DateTime", true);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub once_upon_a_wednesday:Option<chrono::NaiveDateTime>,".to_string(),
    )?;

    let prop = gen_property_of_type("SomeDecimalNumber", "Edm.Decimal", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub some_decimal_number:rust_decimal::Decimal,".to_string(),
    )?;

    let prop = gen_property_of_type("DoubleTrouble", "Edm.Double", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub double_trouble:f64,".to_string())?;

    let prop = gen_property_of_type("GooeyGuid", "Edm.Guid", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub gooey_guid:uuid::Uuid,".to_string())?;

    let prop = gen_property_of_type("MaybeAnInt16", "Edm.Int16", true);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub maybe_an_int_16:Option<i16>,".to_string())?;

    let prop = gen_property_of_type("MaybeAnInt32", "Edm.Int32", true);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub maybe_an_int_32:Option<i32>,".to_string())?;

    let prop = gen_property_of_type("MaybeAnInt64", "Edm.Int64", true);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub maybe_an_int_64:Option<i64>,".to_string())?;

    let prop = gen_property_of_type("NothingToSeeHere", "Edm.Null", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub nothing_to_see_here:(),".to_string())?;

    let prop = gen_property_of_type("SingleByte", "Edm.SByte", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub single_byte:i8,".to_string())?;

    let prop = gen_property_of_type("SinglePrecisionFloat", "Edm.Single", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub single_precision_float:f32,".to_string())?;

    let prop = gen_property_of_type("WhatsTheTimeEccles", "Edm.Time", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub whats_the_time_eccles:std::time::SystemTime,".to_string(),
    )?;

    // Unknown EDM type
    let prop = gen_property_of_type("WhatIsIt", "Edm.NonsenseTypeValue", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub what_is_it:String,".to_string())?;

    // Unknown type
    let prop = gen_property_of_type("ThatsWeird", "NotSeenThisBefore", false);
    let src_lines = to_rust_src(prop.to_rust());
    handle_test_comparison(&src_lines[0].to_string(), &"pub thats_weird:NotSeenThisBefore,".to_string())
}
