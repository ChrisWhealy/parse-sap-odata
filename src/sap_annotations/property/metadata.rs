use std::fmt::Formatter;

use super::SAPAnnotationsProperty;

use crate::{
    parser::generate::{gen_bool_string, gen_opt_string, syntax_fragments::*},
    sap_annotations::OptionalAnnotationType,
    sap_semantics::OptionalSemanticType,
};

static MY_NAME: &[u8] = "SAPAnnotationsProperty".as_bytes();

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
    pub fn value(prop_name: SAPAnnotationsPropertyFieldNames) -> &'static [u8] {
        match prop_name {
            SAPAnnotationsPropertyFieldNames::Label => b"label",
            SAPAnnotationsPropertyFieldNames::Heading => b"heading",
            SAPAnnotationsPropertyFieldNames::QuickInfo => b"quick_info",
            SAPAnnotationsPropertyFieldNames::IsUnicode => b"is_unicode",
            SAPAnnotationsPropertyFieldNames::Semantics => b"semantics",
            SAPAnnotationsPropertyFieldNames::IsCreatable => b"is_creatable",
            SAPAnnotationsPropertyFieldNames::IsUpdatable => b"is_updatable",
            SAPAnnotationsPropertyFieldNames::IsSortable => b"is_sortable",
            SAPAnnotationsPropertyFieldNames::IsFilterable => b"is_filterable",
            SAPAnnotationsPropertyFieldNames::IsAddressable => b"is_addressable",
            SAPAnnotationsPropertyFieldNames::IsRequiredInFilter => b"is_required_in_filter",
            SAPAnnotationsPropertyFieldNames::FilterRestriction => b"filter_restriction",
            SAPAnnotationsPropertyFieldNames::FilterFor => b"filter_for",
            SAPAnnotationsPropertyFieldNames::Text => b"text",
            SAPAnnotationsPropertyFieldNames::TextFor => b"text_for",
            SAPAnnotationsPropertyFieldNames::Unit => b"unit",
            SAPAnnotationsPropertyFieldNames::Precision => b"precision",
            SAPAnnotationsPropertyFieldNames::IsVisible => b"is_visible",
            SAPAnnotationsPropertyFieldNames::FieldControl => b"field_control",
            SAPAnnotationsPropertyFieldNames::ValidationRegexp => b"validation_regexp",
            SAPAnnotationsPropertyFieldNames::DisplayFormat => b"display_format",
            SAPAnnotationsPropertyFieldNames::ValueList => b"value_list",
            SAPAnnotationsPropertyFieldNames::LowerBoundary => b"lower_boundary",
            SAPAnnotationsPropertyFieldNames::UpperBoundary => b"upper_boundary",
            SAPAnnotationsPropertyFieldNames::AggregationRole => b"aggregation_role",
            SAPAnnotationsPropertyFieldNames::SuperOrdinate => b"super_ordinate",
            SAPAnnotationsPropertyFieldNames::AttributeFor => b"attribute_for",
            SAPAnnotationsPropertyFieldNames::HierarchyNodeFor => b"hierarchy_node_for",
            SAPAnnotationsPropertyFieldNames::HierarchyNodeExternalKeyFor => b"hierarchy_node_external_key_for",
            SAPAnnotationsPropertyFieldNames::HierarchyLevelFor => b"hierarchy_level_for",
            SAPAnnotationsPropertyFieldNames::HierarchyParentNodeFor => b"hierarchy_parent_node_for",
            SAPAnnotationsPropertyFieldNames::HierarchyParentNavigationFor => b"hierarchy_parent_navigation_for",
            SAPAnnotationsPropertyFieldNames::HierarchyDrillStateFor => b"hierarchy_drill_state_for",
            SAPAnnotationsPropertyFieldNames::HierarchyNodeDescendantCountFor => b"hierarchy_node_descendant_count_for",
            SAPAnnotationsPropertyFieldNames::HierarchyPreorderRankFor => b"hierarchy_preorder_rank_for",
            SAPAnnotationsPropertyFieldNames::HierarchySiblingRankFor => b"hierarchy_sibling_rank_for",
            SAPAnnotationsPropertyFieldNames::Parameter => b"parameter",
            SAPAnnotationsPropertyFieldNames::IsAnnotation => b"is_annotation",
            SAPAnnotationsPropertyFieldNames::UpdatablePath => b"updatable_path",
            SAPAnnotationsPropertyFieldNames::PreserveFlagFor => b"preserve_flag_for",
            SAPAnnotationsPropertyFieldNames::HasVariableScale => b"has_variable_scale",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SAPAnnotationsProperty {
    // Check for any used subtypes
    pub fn used_subtypes(&self) -> Vec<&[u8]> {
        let mut subtypes: Vec<&[u8]> = vec![];

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
fn line_into(f: &mut Formatter<'_>, prop_md: SAPAnnotationsPropertyFieldNames, val: &[u8]) -> std::fmt::Result {
    for s in [SAPAnnotationsPropertyFieldNames::value(prop_md), COLON, val, COMMA, LINE_FEED] {
        write!(f, "{}", std::str::from_utf8(s).unwrap())?;
    }
    Ok(())
}

impl std::fmt::Display for SAPAnnotationsProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(MY_NAME).unwrap())?;
        write!(f, "{}", std::str::from_utf8(OPEN_CURLY).unwrap())?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Label, &gen_opt_string(&self.label))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Heading, &gen_opt_string(&self.heading))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::QuickInfo, &gen_opt_string(&self.quick_info))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsUnicode, &gen_bool_string(self.is_unicode))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Semantics, &self.semantics.opt_sem_type(&self.semantics))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsCreatable, &gen_bool_string(self.is_creatable))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsUpdatable, &gen_bool_string(self.is_updatable))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsSortable, &gen_bool_string(self.is_sortable))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsFilterable, &gen_bool_string(self.is_filterable))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsAddressable, &gen_bool_string(self.is_addressable))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsRequiredInFilter, &gen_bool_string(self.is_required_in_filter))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::FilterRestriction, &self.filter_restriction.opt_anno_type(&self.filter_restriction))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::FilterFor, &gen_opt_string(&self.filter_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Text, &gen_opt_string(&self.text))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::TextFor, &gen_opt_string(&self.text_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Unit, &gen_opt_string(&self.unit))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Precision, &gen_opt_string(&self.precision))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsVisible, &gen_bool_string(self.is_visible))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::FieldControl, &self.field_control.opt_anno_type(&self.field_control))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::ValidationRegexp, &gen_opt_string(&self.validation_regexp))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::DisplayFormat, &self.display_format.opt_anno_type(&self.display_format))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::ValueList, &gen_opt_string(&self.value_list))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::LowerBoundary, &gen_opt_string(&self.lower_boundary))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::UpperBoundary, &gen_opt_string(&self.upper_boundary))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::AggregationRole, &self.aggregation_role.opt_anno_type(&self.aggregation_role))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::SuperOrdinate, &gen_opt_string(&self.super_ordinate))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::AttributeFor, &gen_opt_string(&self.attribute_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyNodeFor, &gen_opt_string(&self.hierarchy_node_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyNodeExternalKeyFor, &gen_opt_string(&self.hierarchy_node_external_key_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyLevelFor, &gen_opt_string(&self.hierarchy_level_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyParentNodeFor, &gen_opt_string(&self.hierarchy_parent_node_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyParentNavigationFor, &gen_opt_string(&self.hierarchy_parent_navigation_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyDrillStateFor, &gen_opt_string(&self.hierarchy_drill_state_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyNodeDescendantCountFor, &gen_opt_string(&self.hierarchy_node_descendant_count_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchyPreorderRankFor, &gen_opt_string(&self.hierarchy_preorder_rank_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HierarchySiblingRankFor, &gen_opt_string(&self.hierarchy_sibling_rank_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::Parameter, &self.parameter.opt_anno_type(&self.parameter))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::IsAnnotation, &gen_bool_string(self.is_annotation))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::UpdatablePath, &gen_opt_string(&self.updatable_path))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::PreserveFlagFor, &gen_opt_string(&self.preserve_flag_for))?;
        line_into(f, SAPAnnotationsPropertyFieldNames::HasVariableScale, &gen_bool_string(self.has_variable_scale))?;
        write!(f, "{}", std::str::from_utf8(CLOSE_CURLY).unwrap())
    }
}
