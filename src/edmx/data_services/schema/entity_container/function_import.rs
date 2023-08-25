use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Parameter>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    #[serde(rename = "Name")]
    pub parameter_name: String,

    #[serde(rename = "Type")]
    pub parameter_type: String,

    pub mode: String,
    pub max_length: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<FunctionImport>` tag
///
/// # Child Nodes
/// `1:n Parameter`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionImport {
    pub name: String,
    pub return_type: String,
    pub entity_set: Option<String>,

    #[serde(rename = "m:HttpMethod")]
    pub http_method: String,

    #[serde(rename = "sap:action-for")]
    pub sap_action_for: Option<String>,

    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
