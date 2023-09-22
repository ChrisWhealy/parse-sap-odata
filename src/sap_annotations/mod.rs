use crate::utils::{de_str_to_bool, de_str_to_list, default_false, default_true};
use serde::{Deserialize, Serialize};

static ONE: &str = "1";
static ATOM: &str = "atom";
static JSON: &str = "json";

pub fn default_sap_content_version() -> String {
    String::from(ONE)
}

pub fn default_sap_schema_version() -> String {
    String::from(ONE)
}

pub fn default_entity_container_supported_formats() -> Vec<String> {
    vec![String::from(ATOM), String::from(JSON)]
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:AssociationSet`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmassociationset
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsAssociationSet {
    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub content_version: String,

    #[serde(
        rename = "@creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "@deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "@updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntityContainer`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentitycontainer
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsEntityContainer {
    #[serde(
        rename = "@message-scope-supported",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub message_scope_supported: bool,

    #[serde(
        rename = "@use-batch",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub use_batch: bool,

    #[serde(
        rename = "@supported-formats",
        deserialize_with = "de_str_to_list",
        default = "default_entity_container_supported_formats"
    )]
    pub supported_formats: Vec<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntitySet`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentityset
#[derive(Debug, Serialize, Deserialize)]
pub enum SAPSemanticsEntitySet {
    #[serde(rename = "aggregate")]
    Aggregate,

    #[serde(rename = "timeseries")]
    TimeSeries,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsEntitySet {
    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub content_version: String,

    #[serde(rename = "@semantics")]
    pub semantics: Option<SAPSemanticsEntitySet>,

    #[serde(rename = "@label")]
    pub label: Option<String>,

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
        rename = "@deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "@searchable",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_searchable: bool,

    #[serde(
        rename = "@pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_pageable: bool,

    #[serde(
        rename = "@topable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_topable: bool,

    #[serde(
        rename = "@countable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_countable: bool,

    #[serde(
        rename = "@addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_addressable: bool,

    #[serde(
        rename = "@requires-filter",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub requires_filter: bool,

    #[serde(
        rename = "@change-tracking",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub change_tracking_enabled: bool,

    #[serde(rename = "@maxpagesize")]
    pub max_page_size: Option<u32>,

    #[serde(rename = "@delta-link-validity")]
    pub delta_link_validity: Option<u32>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:EntityType`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmentitytype
#[derive(Debug, Serialize, Deserialize)]
pub enum SAPSemanticsEntityType {
    #[serde(rename = "vcard")]
    VCard,

    #[serde(rename = "vevent")]
    VEvent,

    #[serde(rename = "vtodo")]
    VToDo,

    #[serde(rename = "parameters")]
    Paramaters,

    #[serde(rename = "aggregate")]
    Aggregate,

    #[serde(rename = "variant")]
    Variant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsEntityType {
    #[serde(rename = "@label")]
    pub sap_label: Option<String>,

    #[serde(rename = "@semantics")]
    pub sap_semantics: Option<SAPSemanticsEntityType>,

    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:FunctionImport`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmfunctionimport
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsFunctionImportParameter {
    #[serde(rename = "@label")]
    pub label: Option<String>,

    #[serde(
        rename = "@variable-scale",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub has_variable_scale: bool,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:FunctionImport`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmfunctionimport
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsFunctionImport {
    #[serde(rename = "@label")]
    pub label: Option<String>,

    #[serde(rename = "@action-for")]
    pub action_for: Option<String>,

    #[serde(rename = "@applicable-path")]
    pub creatable_path: Option<String>,

    #[serde(rename = "@planning-function")]
    pub planning_function: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:NavigationProperty`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmnavigationproperty
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsNavigationProperty {
    #[serde(
        rename = "@creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(rename = "@creatable-path")]
    pub creatable_path: Option<String>,

    #[serde(
        rename = "@filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_filterable: bool,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:Property`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmproperty
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPSemanticsProperty {
    #[serde(rename = "tel")]
    TelephoneNumber,

    #[serde(rename = "tel;type=cell,work")]
    WorkCellphoneNumber,

    #[serde(rename = "tel;type=fax")]
    FaxNumber,

    #[serde(rename = "email")]
    EmailAddress,

    #[serde(rename = "email;type=pref")]
    PreferredEmailAddress,

    #[serde(rename = "url")]
    URL,

    #[serde(rename = "name")]
    Fullname,

    #[serde(rename = "givenname")]
    FirstOrGivenName,

    #[serde(rename = "middlename")]
    MiddleName,

    #[serde(rename = "familyname")]
    LastName,

    #[serde(rename = "nickname")]
    Nickname,

    #[serde(rename = "honorific")]
    Title,

    #[serde(rename = "suffix")]
    NameSuffix,

    #[serde(rename = "note")]
    VCardNotes,

    #[serde(rename = "photo")]
    PhotoURL,

    #[serde(rename = "city")]
    City,

    #[serde(rename = "street")]
    Street,

    #[serde(rename = "country")]
    Country,

    #[serde(rename = "region")]
    Region,

    #[serde(rename = "zip")]
    PostalCode,

    #[serde(rename = "pobox")]
    PostOfficeBox,

    #[serde(rename = "ord")]
    OrganizationName,

    #[serde(rename = "org-unit")]
    OrganizationalUnit,

    #[serde(rename = "org-role")]
    OrganizationalRole,

    #[serde(rename = "title")]
    JobTitle,

    #[serde(rename = "bday")]
    DateOfBirth,

    #[serde(rename = "summary")]
    CalendarComponentSummary,

    #[serde(rename = "description")]
    CalendarComponentDescription,

    #[serde(rename = "categories")]
    CalendarComponentCategories,

    #[serde(rename = "dtstart")]
    CalendarComponentStartDateTime,

    #[serde(rename = "dtend")]
    CalendarComponentEndDateTime,

    #[serde(rename = "duration")]
    CalendarComponentDuration,

    #[serde(rename = "due")]
    ToDoDueDateTime,

    #[serde(rename = "completed")]
    ToDoCompletedDateTime,

    #[serde(rename = "priority")]
    CalendarComponentPriority,

    #[serde(rename = "class")]
    CalendarComponentAccessClassification,

    #[serde(rename = "status")]
    CalendarComponentStatus,

    #[serde(rename = "percent-complete")]
    ToDoPercentComplete,

    #[serde(rename = "contact")]
    CalendarComponentContact,

    #[serde(rename = "location")]
    CalendarComponentVenue,

    #[serde(rename = "transp")]
    TransparentEvent,

    #[serde(rename = "fbtype")]
    CalendarComponentFreeBusyTime,

    #[serde(rename = "wholeday")]
    CalendarComponentOccupiesWholeDay,

    #[serde(rename = "year")]
    CalendarComponentYear,

    #[serde(rename = "yearmonth")]
    CalendarComponentYearMonth,

    #[serde(rename = "yearmonthday")]
    CalendarComponentYearMonthDay,

    #[serde(rename = "from")]
    EmailFrom,

    #[serde(rename = "sender")]
    EmailSender,

    #[serde(rename = "to")]
    EmailToList,

    #[serde(rename = "cc")]
    EmailCCList,

    #[serde(rename = "bcc")]
    EmailBCCList,

    #[serde(rename = "subject")]
    EmailSubject,

    #[serde(rename = "body")]
    EmailBody,

    #[serde(rename = "keywords")]
    EmailKeywordList,

    #[serde(rename = "received")]
    EmailDateTimeReceived,

    #[serde(rename = "geo-lon")]
    GeolocationLongitude,

    #[serde(rename = "geo-lat")]
    GeolocationLatitude,

    #[serde(rename = "currency-code")]
    CurrencyCode,

    #[serde(rename = "unit-of-measure")]
    UnitOfMeasure,

    #[serde(rename = "count")]
    Count,
}

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPDisplayFormatProperty {
    #[serde(rename = "Date")]
    Date,

    #[serde(rename = "NonNegative")]
    NonNegative,

    #[serde(rename = "UpperCase")]
    UpperCase,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPFilterRestrictionProperty {
    #[serde(rename = "single-value")]
    SingleValue,

    #[serde(rename = "multi-value")]
    MultiValue,

    #[serde(rename = "interval")]
    Interval,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPAggregationRoleProperty {
    #[serde(rename = "dimension")]
    Dimension,

    #[serde(rename = "measure")]
    Measure,

    #[serde(rename = "totaled-properties-list")]
    TotalPropertiesList,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum SAPParameterProperty {
    #[serde(rename = "mandatory")]
    Mandatory,

    #[serde(rename = "optional")]
    Optional,
}

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
/// SAP Annotations applicable to `edm:Schema`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmschema
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsSchema {
    #[serde(rename = "@schema_version", default = "default_sap_schema_version")]
    pub schema_version: String,
}
