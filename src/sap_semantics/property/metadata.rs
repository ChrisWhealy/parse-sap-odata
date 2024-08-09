use crate::{
    parser::syntax_fragments::{CLOSE_PAREN, COLON2, NONE, OPEN_PAREN, SOME},
    sap_semantics::{property::SAPSemanticsProperty, OptionalSemanticType, SemanticType}
};

static MY_NAME: &[u8] = "SAPSemanticsProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SemanticType for SAPSemanticsProperty {
    fn member_name(&self) -> Vec<u8> {
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalSemanticType for Option<SAPSemanticsProperty> {
    fn opt_sem_type<T: SemanticType>(&self, opt_self: &Option<T>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            [SOME, OPEN_PAREN, MY_NAME, COLON2, &*anno_type.member_name(), CLOSE_PAREN].concat()
        } else {
            NONE.to_vec()
        }
    }
}
