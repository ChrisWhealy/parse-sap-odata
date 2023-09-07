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
    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub content_version: String,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "sap:updatable",
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
        rename = "sap:message-scope-supported",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub message_scope_supported: bool,

    #[serde(
        rename = "sap:use-batch",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub use_batch: bool,

    #[serde(
        rename = "sap:supported-formats",
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
    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub content_version: String,

    #[serde(rename = "sap:semantics")]
    pub semantics: Option<SAPSemanticsEntitySet>,

    #[serde(rename = "sap:label")]
    pub label: Option<String>,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "sap:searchable",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_searchable: bool,

    #[serde(
        rename = "sap:pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_pageable: bool,

    #[serde(
        rename = "sap:topable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_topable: bool,

    #[serde(
        rename = "sap:countable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_countable: bool,

    #[serde(
        rename = "sap:addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_addressable: bool,

    #[serde(
        rename = "sap:requires-filter",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub requires_filter: bool,

    #[serde(
        rename = "sap:change-tracking",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub change_tracking_enabled: bool,

    #[serde(rename = "sap:maxpagesize")]
    pub max_page_size: Option<u32>,

    #[serde(rename = "sap:delta-link-validity")]
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
    #[serde(rename = "sap:label")]
    pub sap_label: Option<String>,

    #[serde(rename = "sap:semantics")]
    pub sap_semantics: Option<SAPSemanticsEntityType>,

    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:FunctionImport`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmfunctionimport
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SAPAnnotationsFunctionImportParameter {
    #[serde(rename = "sap:label")]
    pub label: Option<String>,

    #[serde(
        rename = "sap:variable-scale",
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
    #[serde(rename = "sap:label")]
    pub label: Option<String>,

    #[serde(rename = "sap:action-for")]
    pub action_for: Option<String>,

    #[serde(rename = "sap:applicable-path")]
    pub creatable_path: Option<String>,

    #[serde(rename = "sap:planning-function")]
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
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(rename = "sap:creatable-path")]
    pub creatable_path: Option<String>,

    #[serde(
        rename = "sap:filterable",
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
    #[serde(rename = "sap:label")]
    pub label: Option<String>,

    #[serde(rename = "sap:heading")]
    pub heading: Option<String>,

    #[serde(rename = "sap:quickinfo")]
    pub quick_info: Option<String>,

    #[serde(
        rename = "sap:unicode",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_unicode: bool,

    #[serde(rename = "sap:semantics")]
    pub semantics: Option<SAPSemanticsProperty>,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,

    #[serde(
        rename = "sap:sortable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_sortable: bool,

    #[serde(
        rename = "sap:filterable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_filterable: bool,

    #[serde(
        rename = "sap:addressable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_addressable: bool,

    #[serde(
        rename = "sap:required-in-filter",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_required_in_filter: bool,

    #[serde(rename = "sap:filter-restriction")]
    pub filter_restriction: Option<SAPFilterRestrictionProperty>,

    #[serde(rename = "sap:filter-for")]
    pub filter_for: Option<String>,

    #[serde(rename = "sap:text")]
    pub text: Option<String>,

    #[serde(rename = "sap:text-for")]
    pub text_for: Option<String>,

    #[serde(rename = "sap:unit")]
    pub unit: Option<String>,

    #[serde(rename = "sap:precision")]
    pub precision: Option<String>,

    #[serde(
        rename = "sap:visible",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_visible: bool,

    #[serde(rename = "sap:field-control")]
    pub field_control: Option<SAPFieldControlProperty>,

    #[serde(rename = "sap:validation-regexp")]
    pub validation_regexp: Option<String>,

    #[serde(rename = "sap:display-format")]
    pub display_format: Option<SAPDisplayFormatProperty>,

    #[serde(rename = "sap:value-list")]
    pub value_list: Option<String>,

    #[serde(rename = "sap:lower-boundary")]
    pub lower_boundary: Option<String>,

    #[serde(rename = "sap:upper-boundary")]
    pub upper_boundary: Option<String>,

    #[serde(rename = "sap:aggregation-role")]
    pub aggregation_role: Option<SAPAggregationRoleProperty>,

    #[serde(rename = "sap:super-ordinate")]
    pub super_ordinate: Option<String>,

    #[serde(rename = "sap:attribute-for")]
    pub attribute_for: Option<String>,

    #[serde(rename = "sap:hierarchy-node-for")]
    pub hierarchy_node_for: Option<String>,

    #[serde(rename = "sap:hierarchy-node-external-key-for")]
    pub hierarchy_node_external_key_for: Option<String>,

    #[serde(rename = "sap:hierarchy-level-for")]
    pub hierarchy_level_for: Option<String>,

    #[serde(rename = "sap:hierarchy-parent-node-for")]
    pub hierarchy_parent_node_for: Option<String>,

    #[serde(rename = "sap:hierarchy-parent-navigation-for")]
    pub hierarchy_parent_navigation_for: Option<String>,

    #[serde(rename = "sap:hierarchy-drill-state-for")]
    pub hierarchy_drill_state_for: Option<String>,

    #[serde(rename = "sap:hierarchy-node-descendant-count-for")]
    pub hierarchy_node_descendant_count_for: Option<String>,

    #[serde(rename = "sap:hierarchy-preorder-rank-for")]
    pub hierarchy_preorder_rank_for: Option<String>,

    #[serde(rename = "sap:hierarchy-sibling-rank-for")]
    pub hierarchy_sibling_rank_for: Option<String>,

    #[serde(rename = "sap:parameter")]
    pub parameter: Option<SAPParameterProperty>,

    #[serde(
        rename = "sap:is-annotation",
        deserialize_with = "de_str_to_bool",
        default = "default_false"
    )]
    pub is_annotation: bool,

    #[serde(rename = "sap:updatable-path")]
    pub updatable_path: Option<String>,

    #[serde(rename = "sap:preserve-flag-for")]
    pub preserve_flag_for: Option<String>,

    #[serde(
        rename = "sap:variable-scale",
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
    #[serde(rename = "sap:schema_version", default = "default_sap_schema_version")]
    pub schema_version: String,
}
