use std::fmt::Formatter;

use super::SAPAnnotationsProperty;

use crate::{
    parser::syntax_fragments::{
        fragment_generators::{gen_bool_string, gen_opt_string},
        CLOSE_CURLY, COLON, COMMA, LINE_FEED, OPEN_CURLY, PATH_TO_SAP_AGGREGATION_PROPERTY,
        PATH_TO_SAP_ANNOTATIONS_DISPLAY_FORMAT_PROPERTY, PATH_TO_SAP_ANNOTATIONS_FIELD_CONTROL_PROPERTY,
        PATH_TO_SAP_ANNOTATIONS_FILTER_RESTRICTION_PROPERTY, PATH_TO_SAP_ANNOTATIONS_PARAMETER_PROPERTY,
        PATH_TO_SAP_SEMANTICS_PROPERTY,
    },
    sap_annotations::{
        aggregation_role::SAPAggregationRoleProperty, display_format::SAPDisplayFormatProperty,
        field_control::SAPFieldControlProperty, filter_restriction::SAPFilterRestrictionProperty,
        parameter::SAPParameterProperty,
    },
    sap_semantics::property::SAPSemanticsProperty,
};

static MY_NAME: &[u8] = "SAPAnnotationsProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

impl std::fmt::Display for SAPAnnotationsProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let out_buffer: Vec<u8> = [
            MY_NAME,
            OPEN_CURLY,
            &*line_from(SAPAnnotationsPropertyMetadata::Label, gen_opt_string(&self.label)),
            &*line_from(SAPAnnotationsPropertyMetadata::Heading, gen_opt_string(&self.heading)),
            &*line_from(SAPAnnotationsPropertyMetadata::QuickInfo, gen_opt_string(&self.quick_info)),
            &*line_from(SAPAnnotationsPropertyMetadata::IsUnicode, gen_bool_string(self.is_unicode)),
            &*line_from(
                SAPAnnotationsPropertyMetadata::Semantics,
                SAPSemanticsProperty::opt_annotation_type_src(&self.semantics),
            ),
            &*line_from(SAPAnnotationsPropertyMetadata::IsCreatable, gen_bool_string(self.is_creatable)),
            &*line_from(SAPAnnotationsPropertyMetadata::IsUpdatable, gen_bool_string(self.is_updatable)),
            &*line_from(SAPAnnotationsPropertyMetadata::IsSortable, gen_bool_string(self.is_sortable)),
            &*line_from(
                SAPAnnotationsPropertyMetadata::IsFilterable,
                gen_bool_string(self.is_filterable),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::IsAddressable,
                gen_bool_string(self.is_addressable),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::IsRequiredInFilter,
                gen_bool_string(self.is_required_in_filter),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::FilterRestriction,
                SAPFilterRestrictionProperty::opt_anno_type(&self.filter_restriction),
            ),
            &*line_from(SAPAnnotationsPropertyMetadata::FilterFor, gen_opt_string(&self.filter_for)),
            &*line_from(SAPAnnotationsPropertyMetadata::Text, gen_opt_string(&self.text)),
            &*line_from(SAPAnnotationsPropertyMetadata::TextFor, gen_opt_string(&self.text_for)),
            &*line_from(SAPAnnotationsPropertyMetadata::Unit, gen_opt_string(&self.unit)),
            &*line_from(SAPAnnotationsPropertyMetadata::Precision, gen_opt_string(&self.precision)),
            &*line_from(SAPAnnotationsPropertyMetadata::IsVisible, gen_bool_string(self.is_visible)),
            &*line_from(
                SAPAnnotationsPropertyMetadata::FieldControl,
                SAPFieldControlProperty::opt_fc_prop(&self.field_control),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::ValidationRegexp,
                gen_opt_string(&self.validation_regexp),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::DisplayFormat,
                SAPDisplayFormatProperty::opt_anno_type(&self.display_format),
            ),
            &*line_from(SAPAnnotationsPropertyMetadata::ValueList, gen_opt_string(&self.value_list)),
            &*line_from(
                SAPAnnotationsPropertyMetadata::LowerBoundary,
                gen_opt_string(&self.lower_boundary),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::UpperBoundary,
                gen_opt_string(&self.upper_boundary),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::AggregationRole,
                SAPAggregationRoleProperty::opt_anno_type(&self.aggregation_role),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::SuperOrdinate,
                gen_opt_string(&self.super_ordinate),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::AttributeFor,
                gen_opt_string(&self.attribute_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyNodeFor,
                gen_opt_string(&self.hierarchy_node_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyNodeExternalKeyFor,
                gen_opt_string(&self.hierarchy_node_external_key_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyLevelFor,
                gen_opt_string(&self.hierarchy_level_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyParentNodeFor,
                gen_opt_string(&self.hierarchy_parent_node_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyParentNavigationFor,
                gen_opt_string(&self.hierarchy_parent_navigation_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyDrillStateFor,
                gen_opt_string(&self.hierarchy_drill_state_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyNodeDescendantCountFor,
                gen_opt_string(&self.hierarchy_node_descendant_count_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchyPreorderRankFor,
                gen_opt_string(&self.hierarchy_preorder_rank_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HierarchySiblingRankFor,
                gen_opt_string(&self.hierarchy_sibling_rank_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::Parameter,
                SAPParameterProperty::opt_anno_type(&self.parameter),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::IsAnnotation,
                gen_bool_string(self.is_annotation),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::UpdatablePath,
                gen_opt_string(&self.updatable_path),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::PreserveFlagFor,
                gen_opt_string(&self.preserve_flag_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyMetadata::HasVariableScale,
                gen_bool_string(self.has_variable_scale),
            ),
            CLOSE_CURLY,
        ]
        .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
