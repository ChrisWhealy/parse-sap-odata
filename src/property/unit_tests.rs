use crate::{
    parser::{
        generate::syntax_fragments::{CRATE_CHRONO, CRATE_RUST_DECIMAL},
        AsRustSrc,
    },
    property::Property,
    sap_annotations::property::SAPAnnotationsProperty,
    test_utils::*,
};

use rust_decimal::{serde::str, Decimal};
use serde::Deserialize;
use std::str::FromStr;

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
    type Err = quick_xml::DeError;

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
    type Err = quick_xml::DeError;

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
        deserializer_fn: "".to_string(),
        // deserializer_module: "".to_string(),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_convert_edm_string() -> Result<(), String> {
    let prop = gen_property_of_type("BusinessPartnerID", "Edm.String", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(
        &src_lines[0].to_string(),
        &r#"#[serde(rename = "BusinessPartnerID")]"#.to_string(),
    )?;
    handle_test_comparison(&src_lines[1].to_string(), &"pub business_partner_id:String,".to_string())
}

#[test]
fn should_convert_edm_binary() -> Result<(), String> {
    let prop = gen_property_of_type("SomeBinaryStuff", "Edm.Binary", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub some_binary_stuff:Vec<u8>,".to_string())
}

#[test]
fn should_convert_edm_boolean() -> Result<(), String> {
    let prop = gen_property_of_type("IsOverMisunderestimatable", "Edm.Boolean", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub is_over_misunderestimatable:bool,".to_string())
}

#[test]
fn should_convert_edm_byte() -> Result<(), String> {
    let prop = gen_property_of_type("ASingleByte", "Edm.Byte", true);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub a_single_byte:u8,".to_string())
}

#[test]
fn should_convert_edm_datetime() -> Result<(), String> {
    let prop = gen_property_of_type("OnceUponAWednesday", "Edm.DateTime", true);
    let (src, cr) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub once_upon_a_wednesday:Option<chrono::NaiveDateTime>,".to_string(),
    )?;
    handle_test_comparison(&cr, &CRATE_CHRONO.to_string())
}

#[test]
fn should_convert_edm_datetime_offset() -> Result<(), String> {
    let prop = gen_property_of_type("OnceUponAWednesday", "Edm.DateTime", true);
    let (src, cr) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub once_upon_a_wednesday:Option<chrono::NaiveDateTime>,".to_string(),
    )?;
    handle_test_comparison(&cr, &CRATE_CHRONO.to_string())
}

#[test]
fn should_convert_edm_decimal() -> Result<(), String> {
    let prop = gen_property_of_type("SomeDecimalNumber", "Edm.Decimal", false);
    let (src, cr) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub some_decimal_number:rust_decimal::Decimal,".to_string(),
    )?;
    handle_test_comparison(&cr, &CRATE_RUST_DECIMAL.to_string())
}

#[test]
fn should_convert_edm_double() -> Result<(), String> {
    let prop = gen_property_of_type("DoubleTrouble", "Edm.Double", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub double_trouble:f64,".to_string())
}

#[test]
fn should_convert_edm_guid() -> Result<(), String> {
    let prop = gen_property_of_type("GooeyGuid", "Edm.Guid", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub gooey_guid:uuid::Uuid,".to_string())
}

#[test]
fn should_convert_edm_int16() -> Result<(), String> {
    let prop = gen_property_of_type("MaybeAnInt16", "Edm.Int16", true);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub maybe_an_int_16:Option<i16>,".to_string())
}

#[test]
fn should_convert_edm_int32() -> Result<(), String> {
    let prop = gen_property_of_type("MaybeAnInt32", "Edm.Int32", true);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub maybe_an_int_32:Option<i32>,".to_string())
}

#[test]
fn should_convert_edm_int64() -> Result<(), String> {
    let prop = gen_property_of_type("MaybeAnInt64", "Edm.Int64", true);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub maybe_an_int_64:Option<i64>,".to_string())
}

#[test]
fn should_convert_edm_null() -> Result<(), String> {
    let prop = gen_property_of_type("NothingToSeeHere", "Edm.Null", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub nothing_to_see_here:(),".to_string())
}

#[test]
fn should_convert_edm_sbyte() -> Result<(), String> {
    let prop = gen_property_of_type("SingleByte", "Edm.SByte", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub single_byte:i8,".to_string())
}

#[test]
fn should_convert_edm_float_single() -> Result<(), String> {
    let prop = gen_property_of_type("SinglePrecisionFloat", "Edm.Single", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub single_precision_float:f32,".to_string())
}

#[test]
fn should_convert_edm_time() -> Result<(), String> {
    let prop = gen_property_of_type("WhatsTheTimeEccles", "Edm.Time", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(
        &src_lines[0].to_string(),
        &"pub whats_the_time_eccles:std::time::SystemTime,".to_string(),
    )
}

#[test]
fn should_convert_nonsense_type() -> Result<(), String> {
    // Unknown EDM type
    let prop = gen_property_of_type("WhatIsIt", "Edm.NonsenseTypeValue", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub what_is_it:String,".to_string())
}

#[test]
fn should_convert_unknown_type() -> Result<(), String> {
    // Unknown type
    let prop = gen_property_of_type("ThatsWeird", "NotSeenThisBefore", false);
    let (src, _) = prop.to_rust();
    let src_lines = to_rust_src(src);
    handle_test_comparison(&src_lines[0].to_string(), &"pub thats_weird:NotSeenThisBefore,".to_string())
}
