#[cfg(feature = "parser")]
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::{
    sap_annotations::{
        de_str_to_bool, default_false, default_true, SAPAggregationRoleProperty, SAPDisplayFormatProperty,
        SAPFieldControlProperty, SAPFilterRestrictionProperty, SAPParameterProperty,
    },
    sap_semantics::property::SAPSemanticsProperty,
};
#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::{
    CLOSE_CURLY,
    COLON, COMMA, fragment_generators::{gen_bool_string, gen_opt_string}, LINE_FEED, OPEN_CURLY, PATH_TO_SAP_AGGREGATION_PROPERTY,
    PATH_TO_SAP_DISPLAY_FORMAT_PROPERTY, PATH_TO_SAP_FIELD_CONTROL_PROPERTY, PATH_TO_SAP_FILTER_RESTRICTION_PROPERTY,
    PATH_TO_SAP_PARAMETER_PROPERTY, PATH_TO_SAP_SEMANTICS_PROPERTY, SAP_ANNOTATIONS_PROPERTY,
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
pub enum SAPAnnotationsPropertyMetadata {
    Label,
    Heading,
    QuickInfo,
    IsUnicode,
    Semantics,
    IsCreatable,
    IsUpdatable,
    IsSortable,
    IsFilterable,
    IsAddressable,
    IsRequiredInFilter,
    FilterRestriction,
    FilterFor,
    Text,
    TextFor,
    Unit,
    Precision,
    IsVisible,
    FieldControl,
    ValidationRegexp,
    DisplayFormat,
    ValueList,
    LowerBoundary,
    UpperBoundary,
    AggregationRole,
    SuperOrdinate,
    AttributeFor,
    HierarchyNodeFor,
    HierarchyNodeExternalKeyFor,
    HierarchyLevelFor,
    HierarchyParentNodeFor,
    HierarchyParentNavigationFor,
    HierarchyDrillStateFor,
    HierarchyNodeDescendantCountFor,
    HierarchyPreorderRankFor,
    HierarchySiblingRankFor,
    Parameter,
    IsAnnotation,
    UpdatablePath,
    PreserveFlagFor,
    HasVariableScale,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
impl SAPAnnotationsPropertyMetadata {
    pub fn get_field_name(prop_name: SAPAnnotationsPropertyMetadata) -> Vec<u8> {
        let member = match prop_name {
            SAPAnnotationsPropertyMetadata::Label => "label",
            SAPAnnotationsPropertyMetadata::Heading => "heading",
            SAPAnnotationsPropertyMetadata::QuickInfo => "quick_info",
            SAPAnnotationsPropertyMetadata::IsUnicode => "is_unicode",
            SAPAnnotationsPropertyMetadata::Semantics => "semantics",
            SAPAnnotationsPropertyMetadata::IsCreatable => "is_creatable",
            SAPAnnotationsPropertyMetadata::IsUpdatable => "is_updatable",
            SAPAnnotationsPropertyMetadata::IsSortable => "is_sortable",
            SAPAnnotationsPropertyMetadata::IsFilterable => "is_filterable",
            SAPAnnotationsPropertyMetadata::IsAddressable => "is_addressable",
            SAPAnnotationsPropertyMetadata::IsRequiredInFilter => "is_required_in_filter",
            SAPAnnotationsPropertyMetadata::FilterRestriction => "filter_restriction",
            SAPAnnotationsPropertyMetadata::FilterFor => "filter_for",
            SAPAnnotationsPropertyMetadata::Text => "text",
            SAPAnnotationsPropertyMetadata::TextFor => "text_for",
            SAPAnnotationsPropertyMetadata::Unit => "unit",
            SAPAnnotationsPropertyMetadata::Precision => "precision",
            SAPAnnotationsPropertyMetadata::IsVisible => "is_visible",
            SAPAnnotationsPropertyMetadata::FieldControl => "field_control",
            SAPAnnotationsPropertyMetadata::ValidationRegexp => "validation_regexp",
            SAPAnnotationsPropertyMetadata::DisplayFormat => "display_format",
            SAPAnnotationsPropertyMetadata::ValueList => "value_list",
            SAPAnnotationsPropertyMetadata::LowerBoundary => "lower_boundary",
            SAPAnnotationsPropertyMetadata::UpperBoundary => "upper_boundary",
            SAPAnnotationsPropertyMetadata::AggregationRole => "aggregation_role",
            SAPAnnotationsPropertyMetadata::SuperOrdinate => "super_ordinate",
            SAPAnnotationsPropertyMetadata::AttributeFor => "attribute_for",
            SAPAnnotationsPropertyMetadata::HierarchyNodeFor => "hierarchy_node_for",
            SAPAnnotationsPropertyMetadata::HierarchyNodeExternalKeyFor => "hierarchy_node_external_key_for",
            SAPAnnotationsPropertyMetadata::HierarchyLevelFor => "hierarchy_level_for",
            SAPAnnotationsPropertyMetadata::HierarchyParentNodeFor => "hierarchy_parent_node_for",
            SAPAnnotationsPropertyMetadata::HierarchyParentNavigationFor => "hierarchy_parent_navigation_for",
            SAPAnnotationsPropertyMetadata::HierarchyDrillStateFor => "hierarchy_drill_state_for",
            SAPAnnotationsPropertyMetadata::HierarchyNodeDescendantCountFor => "hierarchy_node_descendant_count_for",
            SAPAnnotationsPropertyMetadata::HierarchyPreorderRankFor => "hierarchy_preorder_rank_for",
            SAPAnnotationsPropertyMetadata::HierarchySiblingRankFor => "hierarchy_sibling_rank_for",
            SAPAnnotationsPropertyMetadata::Parameter => "parameter",
            SAPAnnotationsPropertyMetadata::IsAnnotation => "is_annotation",
            SAPAnnotationsPropertyMetadata::UpdatablePath => "updatable_path",
            SAPAnnotationsPropertyMetadata::PreserveFlagFor => "preserve_flag_for",
            SAPAnnotationsPropertyMetadata::HasVariableScale => "has_variable_scale",
        };

        member.as_bytes().to_vec()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
impl SAPAnnotationsProperty {
    // Check for any used subtypes
    pub fn used_subtypes(&self) -> Vec<&[u8]> {
        let mut subtypes: Vec<&[u8]> = vec![];

        if self.semantics.is_some() {
            subtypes.push(PATH_TO_SAP_SEMANTICS_PROPERTY)
        }
        if self.filter_restriction.is_some() {
            subtypes.push(PATH_TO_SAP_FILTER_RESTRICTION_PROPERTY)
        }
        if self.field_control.is_some() {
            subtypes.push(PATH_TO_SAP_FIELD_CONTROL_PROPERTY)
        }
        if self.display_format.is_some() {
            subtypes.push(PATH_TO_SAP_DISPLAY_FORMAT_PROPERTY)
        }
        if self.aggregation_role.is_some() {
            subtypes.push(PATH_TO_SAP_AGGREGATION_PROPERTY)
        }
        if self.parameter.is_some() {
            subtypes.push(PATH_TO_SAP_PARAMETER_PROPERTY)
        }

        subtypes
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
fn line_from(prop_md: SAPAnnotationsPropertyMetadata, val: Vec<u8>) -> Vec<u8> {
    [
        SAPAnnotationsPropertyMetadata::get_field_name(prop_md),
        COLON.to_vec(),
        val,
        COMMA.to_vec(),
        LINE_FEED.to_vec(),
    ]
        .concat()
}

#[cfg(feature = "parser")]
impl std::fmt::Display for SAPAnnotationsProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out_buffer: Vec<u8> = [
            SAP_ANNOTATIONS_PROPERTY.to_vec(),
            OPEN_CURLY.to_vec(),
            line_from(SAPAnnotationsPropertyMetadata::Label, gen_opt_string(&self.label)),
            line_from(SAPAnnotationsPropertyMetadata::Heading, gen_opt_string(&self.heading)),
            line_from(SAPAnnotationsPropertyMetadata::QuickInfo, gen_opt_string(&self.quick_info)),
            line_from(SAPAnnotationsPropertyMetadata::IsUnicode, gen_bool_string(self.is_unicode)),
            line_from(
                SAPAnnotationsPropertyMetadata::Semantics,
                SAPSemanticsProperty::opt_annotation_type_src(&self.semantics),
            ),
            line_from(SAPAnnotationsPropertyMetadata::IsCreatable, gen_bool_string(self.is_creatable)),
            line_from(SAPAnnotationsPropertyMetadata::IsUpdatable, gen_bool_string(self.is_updatable)),
            line_from(SAPAnnotationsPropertyMetadata::IsSortable, gen_bool_string(self.is_sortable)),
            line_from(
                SAPAnnotationsPropertyMetadata::IsFilterable,
                gen_bool_string(self.is_filterable),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::IsAddressable,
                gen_bool_string(self.is_addressable),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::IsRequiredInFilter,
                gen_bool_string(self.is_required_in_filter),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::FilterRestriction,
                SAPFilterRestrictionProperty::opt_anno_type(&self.filter_restriction),
            ),
            line_from(SAPAnnotationsPropertyMetadata::FilterFor, gen_opt_string(&self.filter_for)),
            line_from(SAPAnnotationsPropertyMetadata::Text, gen_opt_string(&self.text)),
            line_from(SAPAnnotationsPropertyMetadata::TextFor, gen_opt_string(&self.text_for)),
            line_from(SAPAnnotationsPropertyMetadata::Unit, gen_opt_string(&self.unit)),
            line_from(SAPAnnotationsPropertyMetadata::Precision, gen_opt_string(&self.precision)),
            line_from(SAPAnnotationsPropertyMetadata::IsVisible, gen_bool_string(self.is_visible)),
            line_from(
                SAPAnnotationsPropertyMetadata::FieldControl,
                SAPFieldControlProperty::opt_anno_type(&self.field_control),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::ValidationRegexp,
                gen_opt_string(&self.validation_regexp),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::DisplayFormat,
                SAPDisplayFormatProperty::opt_anno_type(&self.display_format),
            ),
            line_from(SAPAnnotationsPropertyMetadata::ValueList, gen_opt_string(&self.value_list)),
            line_from(
                SAPAnnotationsPropertyMetadata::LowerBoundary,
                gen_opt_string(&self.lower_boundary),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::UpperBoundary,
                gen_opt_string(&self.upper_boundary),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::AggregationRole,
                SAPAggregationRoleProperty::opt_anno_type(&self.aggregation_role),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::SuperOrdinate,
                gen_opt_string(&self.super_ordinate),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::AttributeFor,
                gen_opt_string(&self.attribute_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyNodeFor,
                gen_opt_string(&self.hierarchy_node_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyNodeExternalKeyFor,
                gen_opt_string(&self.hierarchy_node_external_key_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyLevelFor,
                gen_opt_string(&self.hierarchy_level_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyParentNodeFor,
                gen_opt_string(&self.hierarchy_parent_node_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyParentNavigationFor,
                gen_opt_string(&self.hierarchy_parent_navigation_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyDrillStateFor,
                gen_opt_string(&self.hierarchy_drill_state_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyNodeDescendantCountFor,
                gen_opt_string(&self.hierarchy_node_descendant_count_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchyPreorderRankFor,
                gen_opt_string(&self.hierarchy_preorder_rank_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HierarchySiblingRankFor,
                gen_opt_string(&self.hierarchy_sibling_rank_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::Parameter,
                SAPParameterProperty::opt_anno_type(&self.parameter),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::IsAnnotation,
                gen_bool_string(self.is_annotation),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::UpdatablePath,
                gen_opt_string(&self.updatable_path),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::PreserveFlagFor,
                gen_opt_string(&self.preserve_flag_for),
            ),
            line_from(
                SAPAnnotationsPropertyMetadata::HasVariableScale,
                gen_bool_string(self.has_variable_scale),
            ),
            CLOSE_CURLY.to_vec(),
        ]
            .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}