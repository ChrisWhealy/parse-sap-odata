#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::{
        aggregation_role::SAPAggregationRoleProperty, de_str_to_bool, default_false, default_true,
        display_format::SAPDisplayFormatProperty, field_control::SAPFieldControlProperty,
        filter_restriction::SAPFilterRestrictionProperty, parameter::SAPParameterProperty,
    },
    sap_semantics::property::SAPSemanticsProperty,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsProperty {
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@heading")]
    pub heading: Option<String>,
    #[serde(rename = "@quickinfo")]
    pub quick_info: Option<String>,
    #[serde(
        rename = "@unicode",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_unicode: bool,
    #[serde(rename = "@semantics")]
    pub semantics: Option<SAPSemanticsProperty>,
    #[serde(
        rename = "@creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,
    #[serde(
        rename = "@updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,
    #[serde(
        rename = "@sortable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_sortable: bool,
    #[serde(
        rename = "@filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_filterable: bool,
    #[serde(
        rename = "@addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_addressable: bool,
    #[serde(
        rename = "@required-in-filter",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_required_in_filter: bool,
    #[serde(rename = "@filter-restriction")]
    pub filter_restriction: Option<SAPFilterRestrictionProperty>,
    #[serde(rename = "@filter-for")]
    pub filter_for: Option<String>,
    #[serde(rename = "@text")]
    pub text: Option<String>,
    #[serde(rename = "@text-for")]
    pub text_for: Option<String>,
    #[serde(rename = "@unit")]
    pub unit: Option<String>,
    #[serde(rename = "@precision")]
    pub precision: Option<String>,
    #[serde(
        rename = "@visible",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_visible: bool,
    #[serde(rename = "@field-control")]
    pub field_control: Option<SAPFieldControlProperty>,
    #[serde(rename = "@validation-regexp")]
    pub validation_regexp: Option<String>,
    #[serde(rename = "@display-format")]
    pub display_format: Option<SAPDisplayFormatProperty>,
    #[serde(rename = "@value-list")]
    pub value_list: Option<String>,
    #[serde(rename = "@lower-boundary")]
    pub lower_boundary: Option<String>,
    #[serde(rename = "@upper-boundary")]
    pub upper_boundary: Option<String>,
    #[serde(rename = "@aggregation-role")]
    pub aggregation_role: Option<SAPAggregationRoleProperty>,
    #[serde(rename = "@super-ordinate")]
    pub super_ordinate: Option<String>,
    #[serde(rename = "@attribute-for")]
    pub attribute_for: Option<String>,
    #[serde(rename = "@hierarchy-node-for")]
    pub hierarchy_node_for: Option<String>,
    #[serde(rename = "@hierarchy-node-external-key-for")]
    pub hierarchy_node_external_key_for: Option<String>,
    #[serde(rename = "@hierarchy-level-for")]
    pub hierarchy_level_for: Option<String>,
    #[serde(rename = "@hierarchy-parent-node-for")]
    pub hierarchy_parent_node_for: Option<String>,
    #[serde(rename = "@hierarchy-parent-navigation-for")]
    pub hierarchy_parent_navigation_for: Option<String>,
    #[serde(rename = "@hierarchy-drill-state-for")]
    pub hierarchy_drill_state_for: Option<String>,
    #[serde(rename = "@hierarchy-node-descendant-count-for")]
    pub hierarchy_node_descendant_count_for: Option<String>,
    #[serde(rename = "@hierarchy-preorder-rank-for")]
    pub hierarchy_preorder_rank_for: Option<String>,
    #[serde(rename = "@hierarchy-sibling-rank-for")]
    pub hierarchy_sibling_rank_for: Option<String>,
    #[serde(rename = "@parameter")]
    pub parameter: Option<SAPParameterProperty>,
    #[serde(
        rename = "@is-annotation",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_annotation: bool,
    #[serde(rename = "@updatable-path")]
    pub updatable_path: Option<String>,
    #[serde(rename = "@preserve-flag-for")]
    pub preserve_flag_for: Option<String>,
    #[serde(
        rename = "@variable-scale",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub has_variable_scale: bool,
}
