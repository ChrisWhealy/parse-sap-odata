#[cfg(feature = "parser")]
pub mod metadata;

use serde::{Deserialize, Serialize};

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
