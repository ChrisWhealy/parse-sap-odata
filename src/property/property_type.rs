use serde::{Deserialize, Serialize};
use std::time::Duration;

use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum PropertyType {
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Binary types
    #[serde(rename = "Edm.Binary")]
    Binary {
        max_length: Option<u32>,
        fixed_length: Option<u32>,
        default: Option<Vec<u8>>,
    },

    #[serde(rename = "Edm.Boolean")]
    Boolean { default: Option<bool> },

    #[serde(rename = "Edm.Byte")]
    Byte { precision: Option<u8>, default: Option<Vec<u8>> },

    #[serde(rename = "Edm.SByte")]
    SByte { precision: Option<u8>, default: Option<i8> },

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Temporal types
    #[serde(rename = "Edm.DateTime")]
    DateTime {
        precision: Option<u8>,
        default: Option<NaiveDateTime>,
    },

    #[serde(rename = "Edm.DateTimeOffset")]
    DateTimeOffset { precision: Option<u8>, default: Option<Duration> },

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Numeric types
    #[serde(rename = "Edm.Decimal")]
    Decimal { precision: Option<u8>, default: Option<f64> },

    #[serde(rename = "Edm.Double")]
    Double { precision: Option<u8>, default: Option<f64> },

    #[serde(rename = "Edm.Single")]
    Single { precision: Option<u8>, default: Option<f64> },

    #[serde(rename = "Edm.Int16")]
    Int16 { precision: Option<u8>, default: Option<i16> },

    #[serde(rename = "Edm.Int32")]
    Int32 { precision: Option<u8>, default: Option<Vec<u8>> },

    #[serde(rename = "Edm.Int64")]
    Int64 { max_length: Option<u32>, fixed_length: Option<u32> },

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // String types
    #[serde(rename = "Edm.String")]
    String {
        precision: Option<u8>,
        max_length: Option<u32>,
        fixed_length: Option<u32>,
    },

    #[serde(rename = "Edm.Guid")]
    Guid { max_length: Option<u32>, fixed_length: Option<u32> },
}
