use std::fmt::Formatter;

use super::SAPAnnotationsProperty;

use crate::{
    parser::generate::{gen_bool_string, gen_opt_string_src, syntax_fragments::*},
    sap_annotations::OptionalAnnotationType,
    sap_semantics::OptionalSemanticType,
};

static MY_NAME: &str = "SAPAnnotationsProperty";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
enum SAPAnnotationsPropertyFieldNames {
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
impl SAPAnnotationsPropertyFieldNames {
    pub fn value(prop_name: SAPAnnotationsPropertyFieldNames) -> &'static str {
        match prop_name {
            SAPAnnotationsPropertyFieldNames::Label => "label",
            SAPAnnotationsPropertyFieldNames::Heading => "heading",
            SAPAnnotationsPropertyFieldNames::QuickInfo => "quick_info",
            SAPAnnotationsPropertyFieldNames::IsUnicode => "is_unicode",
            SAPAnnotationsPropertyFieldNames::Semantics => "semantics",
            SAPAnnotationsPropertyFieldNames::IsCreatable => "is_creatable",
            SAPAnnotationsPropertyFieldNames::IsUpdatable => "is_updatable",
            SAPAnnotationsPropertyFieldNames::IsSortable => "is_sortable",
            SAPAnnotationsPropertyFieldNames::IsFilterable => "is_filterable",
            SAPAnnotationsPropertyFieldNames::IsAddressable => "is_addressable",
            SAPAnnotationsPropertyFieldNames::IsRequiredInFilter => "is_required_in_filter",
            SAPAnnotationsPropertyFieldNames::FilterRestriction => "filter_restriction",
            SAPAnnotationsPropertyFieldNames::FilterFor => "filter_for",
            SAPAnnotationsPropertyFieldNames::Text => "text",
            SAPAnnotationsPropertyFieldNames::TextFor => "text_for",
            SAPAnnotationsPropertyFieldNames::Unit => "unit",
            SAPAnnotationsPropertyFieldNames::Precision => "precision",
            SAPAnnotationsPropertyFieldNames::IsVisible => "is_visible",
            SAPAnnotationsPropertyFieldNames::FieldControl => "field_control",
            SAPAnnotationsPropertyFieldNames::ValidationRegexp => "validation_regexp",
            SAPAnnotationsPropertyFieldNames::DisplayFormat => "display_format",
            SAPAnnotationsPropertyFieldNames::ValueList => "value_list",
            SAPAnnotationsPropertyFieldNames::LowerBoundary => "lower_boundary",
            SAPAnnotationsPropertyFieldNames::UpperBoundary => "upper_boundary",
            SAPAnnotationsPropertyFieldNames::AggregationRole => "aggregation_role",
            SAPAnnotationsPropertyFieldNames::SuperOrdinate => "super_ordinate",
            SAPAnnotationsPropertyFieldNames::AttributeFor => "attribute_for",
            SAPAnnotationsPropertyFieldNames::HierarchyNodeFor => "hierarchy_node_for",
            SAPAnnotationsPropertyFieldNames::HierarchyNodeExternalKeyFor => "hierarchy_node_external_key_for",
            SAPAnnotationsPropertyFieldNames::HierarchyLevelFor => "hierarchy_level_for",
            SAPAnnotationsPropertyFieldNames::HierarchyParentNodeFor => "hierarchy_parent_node_for",
            SAPAnnotationsPropertyFieldNames::HierarchyParentNavigationFor => "hierarchy_parent_navigation_for",
            SAPAnnotationsPropertyFieldNames::HierarchyDrillStateFor => "hierarchy_drill_state_for",
            SAPAnnotationsPropertyFieldNames::HierarchyNodeDescendantCountFor => "hierarchy_node_descendant_count_for",
            SAPAnnotationsPropertyFieldNames::HierarchyPreorderRankFor => "hierarchy_preorder_rank_for",
            SAPAnnotationsPropertyFieldNames::HierarchySiblingRankFor => "hierarchy_sibling_rank_for",
            SAPAnnotationsPropertyFieldNames::Parameter => "parameter",
            SAPAnnotationsPropertyFieldNames::IsAnnotation => "is_annotation",
            SAPAnnotationsPropertyFieldNames::UpdatablePath => "updatable_path",
            SAPAnnotationsPropertyFieldNames::PreserveFlagFor => "preserve_flag_for",
            SAPAnnotationsPropertyFieldNames::HasVariableScale => "has_variable_scale",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPAnnotationsProperty {
    // Check for any used subtypes
    pub fn used_subtypes(&self) -> Vec<&str> {
        let mut subtypes: Vec<&str> = vec![];

        if self.semantics.is_some() {
            subtypes.push(PATH_TO_SAP_SEMANTICS_PROPERTY)
        }
        if self.filter_restriction.is_some() {
            subtypes.push(PATH_TO_SAP_ANNOTATIONS_FILTER_RESTRICTION_PROPERTY)
        }
        if self.field_control.is_some() {
            subtypes.push(PATH_TO_SAP_ANNOTATIONS_FIELD_CONTROL_PROPERTY)
        }
        if self.display_format.is_some() {
            subtypes.push(PATH_TO_SAP_ANNOTATIONS_DISPLAY_FORMAT_PROPERTY)
        }
        if self.aggregation_role.is_some() {
            subtypes.push(PATH_TO_SAP_AGGREGATION_PROPERTY)
        }
        if self.parameter.is_some() {
            subtypes.push(PATH_TO_SAP_ANNOTATIONS_PARAMETER_PROPERTY)
        }

        subtypes
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn line_into(f: &mut Formatter<'_>, prop_md: SAPAnnotationsPropertyFieldNames, val: &str) -> std::fmt::Result {
    write!(
        f,
        "{}{}{}{}{}",
        SAPAnnotationsPropertyFieldNames::value(prop_md),
        COLON,
        val,
        COMMA,
        LINE_FEED
    )
}

impl std::fmt::Display for SAPAnnotationsProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{MY_NAME}")?;
        write!(f, "{OPEN_CURLY}")?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Label, &gen_opt_string_src(&self.label))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Heading, &gen_opt_string_src(&self.heading))?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::QuickInfo,
            &gen_opt_string_src(&self.quick_info),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsUnicode,
            &gen_bool_string(self.is_unicode),
        )?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Semantics, &self.semantics.opt_sem_type())?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsCreatable,
            &gen_bool_string(self.is_creatable),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsUpdatable,
            &gen_bool_string(self.is_updatable),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsSortable,
            &gen_bool_string(self.is_sortable),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsFilterable,
            &gen_bool_string(self.is_filterable),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsAddressable,
            &gen_bool_string(self.is_addressable),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsRequiredInFilter,
            &gen_bool_string(self.is_required_in_filter),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::FilterRestriction,
            &self.filter_restriction.opt_anno_type(),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::FilterFor,
            &gen_opt_string_src(&self.filter_for),
        )?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Text, &gen_opt_string_src(&self.text))?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::TextFor,
            &gen_opt_string_src(&self.text_for),
        )?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Unit, &gen_opt_string_src(&self.unit))?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::Precision,
            &gen_opt_string_src(&self.precision),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsVisible,
            &gen_bool_string(self.is_visible),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::FieldControl,
            &self.field_control.opt_anno_type(),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::ValidationRegexp,
            &gen_opt_string_src(&self.validation_regexp),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::DisplayFormat,
            &self.display_format.opt_anno_type(),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::ValueList,
            &gen_opt_string_src(&self.value_list),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::LowerBoundary,
            &gen_opt_string_src(&self.lower_boundary),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::UpperBoundary,
            &gen_opt_string_src(&self.upper_boundary),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::AggregationRole,
            &self.aggregation_role.opt_anno_type(),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::SuperOrdinate,
            &gen_opt_string_src(&self.super_ordinate),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::AttributeFor,
            &gen_opt_string_src(&self.attribute_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyNodeFor,
            &gen_opt_string_src(&self.hierarchy_node_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyNodeExternalKeyFor,
            &gen_opt_string_src(&self.hierarchy_node_external_key_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyLevelFor,
            &gen_opt_string_src(&self.hierarchy_level_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyParentNodeFor,
            &gen_opt_string_src(&self.hierarchy_parent_node_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyParentNavigationFor,
            &gen_opt_string_src(&self.hierarchy_parent_navigation_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyDrillStateFor,
            &gen_opt_string_src(&self.hierarchy_drill_state_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyNodeDescendantCountFor,
            &gen_opt_string_src(&self.hierarchy_node_descendant_count_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchyPreorderRankFor,
            &gen_opt_string_src(&self.hierarchy_preorder_rank_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HierarchySiblingRankFor,
            &gen_opt_string_src(&self.hierarchy_sibling_rank_for),
        )?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Parameter, &self.parameter.opt_anno_type())?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::IsAnnotation,
            &gen_bool_string(self.is_annotation),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::UpdatablePath,
            &gen_opt_string_src(&self.updatable_path),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::PreserveFlagFor,
            &gen_opt_string_src(&self.preserve_flag_for),
        )?;
        line_into(
            f,
            SAPAnnotationsPropertyFieldNames::HasVariableScale,
            &gen_bool_string(self.has_variable_scale),
        )?;
        write!(f, "{CLOSE_CURLY}")
    }
}
