use crate::{
    parser::generate::syntax_fragments::{CLOSE_PAREN, COLON2, NONE, OPEN_PAREN, SOME},
    sap_semantics::{property::SAPSemanticsProperty, OptionalSemanticType, SemanticType},
};

static MY_NAME: &[u8] = "SAPSemanticsProperty".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl SemanticType for SAPSemanticsProperty {
    fn member_name(&self) -> &'static [u8] {
        match self {
            SAPSemanticsProperty::TelephoneNumber => b"TelephoneNumber",
            SAPSemanticsProperty::WorkCellphoneNumber => b"WorkCellphoneNumber",
            SAPSemanticsProperty::FaxNumber => b"FaxNumber",
            SAPSemanticsProperty::EmailAddress => b"EmailAddress",
            SAPSemanticsProperty::PreferredEmailAddress => b"PreferredEmailAddress",
            SAPSemanticsProperty::URL => b"URL",
            SAPSemanticsProperty::Fullname => b"Fullname",
            SAPSemanticsProperty::FirstOrGivenName => b"FirstOrGivenName",
            SAPSemanticsProperty::MiddleName => b"MiddleName",
            SAPSemanticsProperty::LastName => b"LastName",
            SAPSemanticsProperty::Nickname => b"Nickname",
            SAPSemanticsProperty::Title => b"Title",
            SAPSemanticsProperty::NameSuffix => b"NameSuffix",
            SAPSemanticsProperty::VCardNotes => b"VCardNotes",
            SAPSemanticsProperty::PhotoURL => b"PhotoURL",
            SAPSemanticsProperty::City => b"City",
            SAPSemanticsProperty::Street => b"Street",
            SAPSemanticsProperty::Country => b"Country",
            SAPSemanticsProperty::Region => b"Region",
            SAPSemanticsProperty::PostalCode => b"PostalCode",
            SAPSemanticsProperty::PostOfficeBox => b"PostOfficeBox",
            SAPSemanticsProperty::OrganizationName => b"OrganizationName",
            SAPSemanticsProperty::OrganizationalUnit => b"OrganizationalUnit",
            SAPSemanticsProperty::OrganizationalRole => b"OrganizationalRole",
            SAPSemanticsProperty::JobTitle => b"JobTitle",
            SAPSemanticsProperty::DateOfBirth => b"DateOfBirth",
            SAPSemanticsProperty::CalendarComponentSummary => b"CalendarComponentSummary",
            SAPSemanticsProperty::CalendarComponentDescription => b"CalendarComponentDescription",
            SAPSemanticsProperty::CalendarComponentCategories => b"CalendarComponentCategories",
            SAPSemanticsProperty::CalendarComponentStartDateTime => b"CalendarComponentStartDateTime",
            SAPSemanticsProperty::CalendarComponentEndDateTime => b"CalendarComponentEndDateTime",
            SAPSemanticsProperty::CalendarComponentDuration => b"CalendarComponentDuration",
            SAPSemanticsProperty::ToDoDueDateTime => b"ToDoDueDateTime",
            SAPSemanticsProperty::ToDoCompletedDateTime => b"ToDoCompletedDateTime",
            SAPSemanticsProperty::CalendarComponentPriority => b"CalendarComponentPriority",
            SAPSemanticsProperty::CalendarComponentAccessClassification => b"CalendarComponentAccessClassification",
            SAPSemanticsProperty::CalendarComponentStatus => b"CalendarComponentStatus",
            SAPSemanticsProperty::ToDoPercentComplete => b"ToDoPercentComplete",
            SAPSemanticsProperty::CalendarComponentContact => b"CalendarComponentContact",
            SAPSemanticsProperty::CalendarComponentVenue => b"CalendarComponentVenue",
            SAPSemanticsProperty::TransparentEvent => b"TransparentEvent",
            SAPSemanticsProperty::CalendarComponentFreeBusyTime => b"CalendarComponentFreeBusyTime",
            SAPSemanticsProperty::CalendarComponentOccupiesWholeDay => b"CalendarComponentOccupiesWholeDay",
            SAPSemanticsProperty::CalendarComponentYear => b"CalendarComponentYear",
            SAPSemanticsProperty::CalendarComponentYearMonth => b"CalendarComponentYearMonth",
            SAPSemanticsProperty::CalendarComponentYearMonthDay => b"CalendarComponentYearMonthDay",
            SAPSemanticsProperty::EmailFrom => b"EmailFrom",
            SAPSemanticsProperty::EmailSender => b"EmailSender",
            SAPSemanticsProperty::EmailToList => b"EmailToList",
            SAPSemanticsProperty::EmailCCList => b"EmailCCList",
            SAPSemanticsProperty::EmailBCCList => b"EmailBCCList",
            SAPSemanticsProperty::EmailSubject => b"EmailSubject",
            SAPSemanticsProperty::EmailBody => b"EmailBody",
            SAPSemanticsProperty::EmailKeywordList => b"EmailKeywordList",
            SAPSemanticsProperty::EmailDateTimeReceived => b"EmailDateTimeReceived",
            SAPSemanticsProperty::GeolocationLongitude => b"GeolocationLongitude",
            SAPSemanticsProperty::GeolocationLatitude => b"GeolocationLatitude",
            SAPSemanticsProperty::CurrencyCode => b"CurrencyCode",
            SAPSemanticsProperty::UnitOfMeasure => b"UnitOfMeasure",
            SAPSemanticsProperty::Count => b"Count",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl OptionalSemanticType for Option<SAPSemanticsProperty> {
    fn opt_sem_type<T: SemanticType>(&self, opt_self: &Option<T>) -> Vec<u8> {
        if let Some(anno_type) = opt_self {
            [SOME, OPEN_PAREN, MY_NAME, COLON2, anno_type.member_name(), CLOSE_PAREN].concat()
        } else {
            NONE.to_vec()
        }
    }
}
