use serde::{Deserialize, Serialize};

#[cfg(feature = "parser")]
use crate::parser::syntax_fragments::{CLOSE_PAREN, COLON2, NONE, OPEN_PAREN, SOME};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// SAP Annotations applicable to `edm:Property`
///
/// This enum can act as its own metadata
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(feature = "parser")]
impl SAPSemanticsProperty {
    pub fn opt_annotation_type_src(opt_self: &Option<SAPSemanticsProperty>) -> Vec<u8> {
        let own_name: &[u8] = "SAPSemanticsProperty".as_bytes();

        if let Some(anno_type) = opt_self {
            [SOME, OPEN_PAREN, own_name, COLON2, &*anno_type.as_enum_member(), CLOSE_PAREN].concat()
        } else {
            NONE.to_vec()
        }
    }

    fn as_enum_member(&self) -> Vec<u8> {
        let member = match self {
            SAPSemanticsProperty::TelephoneNumber => "TelephoneNumber",
            SAPSemanticsProperty::WorkCellphoneNumber => "WorkCellphoneNumber",
            SAPSemanticsProperty::FaxNumber => "FaxNumber",
            SAPSemanticsProperty::EmailAddress => "EmailAddress",
            SAPSemanticsProperty::PreferredEmailAddress => "PreferredEmailAddress",
            SAPSemanticsProperty::URL => "URL",
            SAPSemanticsProperty::Fullname => "Fullname",
            SAPSemanticsProperty::FirstOrGivenName => "FirstOrGivenName",
            SAPSemanticsProperty::MiddleName => "MiddleName",
            SAPSemanticsProperty::LastName => "LastName",
            SAPSemanticsProperty::Nickname => "Nickname",
            SAPSemanticsProperty::Title => "Title",
            SAPSemanticsProperty::NameSuffix => "NameSuffix",
            SAPSemanticsProperty::VCardNotes => "VCardNotes",
            SAPSemanticsProperty::PhotoURL => "PhotoURL",
            SAPSemanticsProperty::City => "City",
            SAPSemanticsProperty::Street => "Street",
            SAPSemanticsProperty::Country => "Country",
            SAPSemanticsProperty::Region => "Region",
            SAPSemanticsProperty::PostalCode => "PostalCode",
            SAPSemanticsProperty::PostOfficeBox => "PostOfficeBox",
            SAPSemanticsProperty::OrganizationName => "OrganizationName",
            SAPSemanticsProperty::OrganizationalUnit => "OrganizationalUnit",
            SAPSemanticsProperty::OrganizationalRole => "OrganizationalRole",
            SAPSemanticsProperty::JobTitle => "JobTitle",
            SAPSemanticsProperty::DateOfBirth => "DateOfBirth",
            SAPSemanticsProperty::CalendarComponentSummary => "CalendarComponentSummary",
            SAPSemanticsProperty::CalendarComponentDescription => "CalendarComponentDescription",
            SAPSemanticsProperty::CalendarComponentCategories => "CalendarComponentCategories",
            SAPSemanticsProperty::CalendarComponentStartDateTime => "CalendarComponentStartDateTime",
            SAPSemanticsProperty::CalendarComponentEndDateTime => "CalendarComponentEndDateTime",
            SAPSemanticsProperty::CalendarComponentDuration => "CalendarComponentDuration",
            SAPSemanticsProperty::ToDoDueDateTime => "ToDoDueDateTime",
            SAPSemanticsProperty::ToDoCompletedDateTime => "ToDoCompletedDateTime",
            SAPSemanticsProperty::CalendarComponentPriority => "CalendarComponentPriority",
            SAPSemanticsProperty::CalendarComponentAccessClassification => "CalendarComponentAccessClassification",
            SAPSemanticsProperty::CalendarComponentStatus => "CalendarComponentStatus",
            SAPSemanticsProperty::ToDoPercentComplete => "ToDoPercentComplete",
            SAPSemanticsProperty::CalendarComponentContact => "CalendarComponentContact",
            SAPSemanticsProperty::CalendarComponentVenue => "CalendarComponentVenue",
            SAPSemanticsProperty::TransparentEvent => "TransparentEvent",
            SAPSemanticsProperty::CalendarComponentFreeBusyTime => "CalendarComponentFreeBusyTime",
            SAPSemanticsProperty::CalendarComponentOccupiesWholeDay => "CalendarComponentOccupiesWholeDay",
            SAPSemanticsProperty::CalendarComponentYear => "CalendarComponentYear",
            SAPSemanticsProperty::CalendarComponentYearMonth => "CalendarComponentYearMonth",
            SAPSemanticsProperty::CalendarComponentYearMonthDay => "CalendarComponentYearMonthDay",
            SAPSemanticsProperty::EmailFrom => "EmailFrom",
            SAPSemanticsProperty::EmailSender => "EmailSender",
            SAPSemanticsProperty::EmailToList => "EmailToList",
            SAPSemanticsProperty::EmailCCList => "EmailCCList",
            SAPSemanticsProperty::EmailBCCList => "EmailBCCList",
            SAPSemanticsProperty::EmailSubject => "EmailSubject",
            SAPSemanticsProperty::EmailBody => "EmailBody",
            SAPSemanticsProperty::EmailKeywordList => "EmailKeywordList",
            SAPSemanticsProperty::EmailDateTimeReceived => "EmailDateTimeReceived",
            SAPSemanticsProperty::GeolocationLongitude => "GeolocationLongitude",
            SAPSemanticsProperty::GeolocationLatitude => "GeolocationLatitude",
            SAPSemanticsProperty::CurrencyCode => "CurrencyCode",
            SAPSemanticsProperty::UnitOfMeasure => "UnitOfMeasure",
            SAPSemanticsProperty::Count => "Count",
        };

        member.as_bytes().to_vec()
    }
}
