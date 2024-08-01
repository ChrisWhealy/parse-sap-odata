use serde::{Deserialize, Serialize};

use crate::{
    parser::syntax_fragments::{CLOSE_PAREN, COLON2, DOUBLE_QUOTE, NONE, ONE, OPEN_PAREN, SOME},
    utils::{de_str_to_bool, default_false, default_true},
};

pub mod association_set;
pub mod entity_container;
pub mod entity_set;
pub mod entity_type;
pub mod function_import;
pub mod navigation_property;
pub mod property;
pub mod schema;

static ATOM: &str = "atom";
static JSON: &str = "json";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn default_sap_content_version() -> String {
    String::from_utf8(ONE.to_vec()).unwrap()
}
pub fn default_sap_schema_version() -> String {
    String::from_utf8(ONE.to_vec()).unwrap()
}
pub fn default_entity_container_supported_formats() -> Vec<String> {
    vec![String::from(ATOM), String::from(JSON)]
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPFieldControlProperty {
    #[serde(rename = "0")]
    Hidden,
    #[serde(rename = "1")]
    ReadOnly,
    #[serde(rename = "3")]
    Optional,
    #[serde(rename = "7")]
    Mandatory,
}

impl SAPFieldControlProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPFieldControlProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPFieldControlProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                own_name,
                COLON2,
                &*anno_type.as_enum_member(),
                DOUBLE_QUOTE,
                CLOSE_PAREN,
            ]
            .concat()
        } else {
            NONE.to_vec()
        }
    }

    pub fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPFieldControlProperty::Hidden => "Hidden",
            SAPFieldControlProperty::ReadOnly => "ReadOnly",
            SAPFieldControlProperty::Optional => "Optional",
            SAPFieldControlProperty::Mandatory => "Mandatory",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPDisplayFormatProperty {
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "NonNegative")]
    NonNegative,
    #[serde(rename = "UpperCase")]
    UpperCase,
}

impl SAPDisplayFormatProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPDisplayFormatProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPDisplayFormatProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                own_name,
                COLON2,
                &*anno_type.as_enum_member(),
                DOUBLE_QUOTE,
                CLOSE_PAREN,
            ]
            .concat()
        } else {
            NONE.to_vec()
        }
    }

    pub fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPDisplayFormatProperty::Date => "Date",
            SAPDisplayFormatProperty::NonNegative => "NonNegative",
            SAPDisplayFormatProperty::UpperCase => "UpperCase",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// This enum acts as its own metadata
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPFilterRestrictionProperty {
    #[serde(rename = "single-value")]
    SingleValue,
    #[serde(rename = "multi-value")]
    MultiValue,
    #[serde(rename = "interval")]
    Interval,
}

impl SAPFilterRestrictionProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPFilterRestrictionProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPFilterRestrictionProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                own_name,
                COLON2,
                &*anno_type.as_enum_member(),
                DOUBLE_QUOTE,
                CLOSE_PAREN,
            ]
            .concat()
        } else {
            NONE.to_vec()
        }
    }

    pub fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPFilterRestrictionProperty::SingleValue => "SingleValue",
            SAPFilterRestrictionProperty::MultiValue => "MultiValue",
            SAPFilterRestrictionProperty::Interval => "Interval",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// This enum acts as its own metadata
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPAggregationRoleProperty {
    #[serde(rename = "dimension")]
    Dimension,
    #[serde(rename = "measure")]
    Measure,
    #[serde(rename = "totaled-properties-list")]
    TotalPropertiesList,
}

impl SAPAggregationRoleProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPAggregationRoleProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPAggregationRoleProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                own_name,
                COLON2,
                &*anno_type.as_enum_member(),
                DOUBLE_QUOTE,
                CLOSE_PAREN,
            ]
            .concat()
        } else {
            NONE.to_vec()
        }
    }

    pub fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPAggregationRoleProperty::Dimension => "Dimension",
            SAPAggregationRoleProperty::Measure => "Measure",
            SAPAggregationRoleProperty::TotalPropertiesList => "TotalPropertiesList",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPParameterProperty {
    #[serde(rename = "mandatory")]
    Mandatory,
    #[serde(rename = "optional")]
    Optional,
}

impl SAPParameterProperty {
    pub fn opt_anno_type(opt_self: &Option<SAPParameterProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPParameterProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [
                SOME,
                OPEN_PAREN,
                DOUBLE_QUOTE,
                own_name,
                COLON2,
                &*anno_type.as_enum_member(),
                DOUBLE_QUOTE,
                CLOSE_PAREN,
            ]
            .concat()
        } else {
            NONE.to_vec()
        }
    }

    pub fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPParameterProperty::Mandatory => "Mandatory",
            SAPParameterProperty::Optional => "Optional",
        };

        member.as_bytes().to_vec()
    }
}
