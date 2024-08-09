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
    pub fn value(prop_name: SAPAnnotationsPropertyFieldNames) -> Vec<u8> {
        let member = match prop_name {
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
fn line_from(prop_md: SAPAnnotationsPropertyFieldNames, val: Vec<u8>) -> Vec<u8> {
    [
        SAPAnnotationsPropertyFieldNames::value(prop_md),
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
            &*line_from(SAPAnnotationsPropertyFieldNames::Label, gen_opt_string(&self.label)),
            &*line_from(SAPAnnotationsPropertyFieldNames::Heading, gen_opt_string(&self.heading)),
            &*line_from(SAPAnnotationsPropertyFieldNames::QuickInfo, gen_opt_string(&self.quick_info)),
            &*line_from(SAPAnnotationsPropertyFieldNames::IsUnicode, gen_bool_string(self.is_unicode)),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::Semantics,
                self.semantics.opt_sem_type(&self.semantics),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::IsCreatable,
                gen_bool_string(self.is_creatable),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::IsUpdatable,
                gen_bool_string(self.is_updatable),
            ),
            &*line_from(SAPAnnotationsPropertyFieldNames::IsSortable, gen_bool_string(self.is_sortable)),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::IsFilterable,
                gen_bool_string(self.is_filterable),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::IsAddressable,
                gen_bool_string(self.is_addressable),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::IsRequiredInFilter,
                gen_bool_string(self.is_required_in_filter),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::FilterRestriction,
                self.filter_restriction.opt_anno_type(&self.filter_restriction),
            ),
            &*line_from(SAPAnnotationsPropertyFieldNames::FilterFor, gen_opt_string(&self.filter_for)),
            &*line_from(SAPAnnotationsPropertyFieldNames::Text, gen_opt_string(&self.text)),
            &*line_from(SAPAnnotationsPropertyFieldNames::TextFor, gen_opt_string(&self.text_for)),
            &*line_from(SAPAnnotationsPropertyFieldNames::Unit, gen_opt_string(&self.unit)),
            &*line_from(SAPAnnotationsPropertyFieldNames::Precision, gen_opt_string(&self.precision)),
            &*line_from(SAPAnnotationsPropertyFieldNames::IsVisible, gen_bool_string(self.is_visible)),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::FieldControl,
                self.field_control.opt_anno_type(&self.field_control),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::ValidationRegexp,
                gen_opt_string(&self.validation_regexp),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::DisplayFormat,
                self.display_format.opt_anno_type(&self.display_format),
            ),
            &*line_from(SAPAnnotationsPropertyFieldNames::ValueList, gen_opt_string(&self.value_list)),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::LowerBoundary,
                gen_opt_string(&self.lower_boundary),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::UpperBoundary,
                gen_opt_string(&self.upper_boundary),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::AggregationRole,
                self.aggregation_role.opt_anno_type(&self.aggregation_role),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::SuperOrdinate,
                gen_opt_string(&self.super_ordinate),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::AttributeFor,
                gen_opt_string(&self.attribute_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyNodeFor,
                gen_opt_string(&self.hierarchy_node_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyNodeExternalKeyFor,
                gen_opt_string(&self.hierarchy_node_external_key_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyLevelFor,
                gen_opt_string(&self.hierarchy_level_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyParentNodeFor,
                gen_opt_string(&self.hierarchy_parent_node_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyParentNavigationFor,
                gen_opt_string(&self.hierarchy_parent_navigation_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyDrillStateFor,
                gen_opt_string(&self.hierarchy_drill_state_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyNodeDescendantCountFor,
                gen_opt_string(&self.hierarchy_node_descendant_count_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchyPreorderRankFor,
                gen_opt_string(&self.hierarchy_preorder_rank_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HierarchySiblingRankFor,
                gen_opt_string(&self.hierarchy_sibling_rank_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::Parameter,
                self.parameter.opt_anno_type(&self.parameter),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::IsAnnotation,
                gen_bool_string(self.is_annotation),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::UpdatablePath,
                gen_opt_string(&self.updatable_path),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::PreserveFlagFor,
                gen_opt_string(&self.preserve_flag_for),
            ),
            &*line_from(
                SAPAnnotationsPropertyFieldNames::HasVariableScale,
                gen_bool_string(self.has_variable_scale),
            ),
            CLOSE_CURLY,
        ]
            .concat();

        write!(f, "{}", String::from_utf8(out_buffer).unwrap())
    }
}
