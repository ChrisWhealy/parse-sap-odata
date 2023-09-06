use serde::{Deserialize, Serialize};

use crate::utils::{de_str_to_bool, default_false, default_true};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum PropertySAPSemantics {
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
pub enum PropertySAPFieldControl {
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
pub enum PropertySAPDisplayFormat {
    #[serde(rename = "Date")]
    Date,

    #[serde(rename = "NonNegative")]
    NonNegative,

    #[serde(rename = "UpperCase")]
    UpperCase,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum PropertySAPFilterRestriction {
    #[serde(rename = "single-value")]
    SingleValue,

    #[serde(rename = "multi-value")]
    MultiValue,

    #[serde(rename = "interval")]
    Interval,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum PropertySAPAggregationRole {
    #[serde(rename = "dimension")]
    Dimension,

    #[serde(rename = "measure")]
    Measure,

    #[serde(rename = "totaled-properties-list")]
    TotalPropertiesList,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum PropertySAPParameter {
    #[serde(rename = "mandatory")]
    Mandatory,

    #[serde(rename = "optional")]
    Optional,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:Property`
///
/// See https://sap.github.io/odata-vocabularies/docs/v2-annotations.html#element-edmproperty
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertySAPAnnotations {
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
    pub semantics: Option<PropertySAPSemantics>,

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
    pub filter_restriction: Option<PropertySAPFilterRestriction>,

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
    pub field_control: Option<PropertySAPFieldControl>,

    #[serde(rename = "sap:validation-regexp")]
    pub validation_regexp: Option<String>,

    #[serde(rename = "sap:display-format")]
    pub display_format: Option<PropertySAPDisplayFormat>,

    #[serde(rename = "sap:value-list")]
    pub value_list: Option<String>,

    #[serde(rename = "sap:lower-boundary")]
    pub lower_boundary: Option<String>,

    #[serde(rename = "sap:upper-boundary")]
    pub upper_boundary: Option<String>,

    #[serde(rename = "sap:aggregation-role")]
    pub aggregation_role: Option<PropertySAPAggregationRole>,

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
    pub parameter: Option<PropertySAPParameter>,

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
