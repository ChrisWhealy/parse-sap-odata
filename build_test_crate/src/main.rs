use chrono::Utc;

include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

fn main() {
    let now = Utc::now().naive_utc();
    let bp = BusinessPartner {
        address: Address {
            address_type: Some(String::from("Dummy address type")),
            building: Some(String::from("Dummy building")),
            city: Some(String::from("Dummy city")),
            country: Some(String::from("Dummy country")),
            postal_code: Some(String::from("Dummy postal code")),
            street: Some(String::from("Dummy street")),
        },
        business_partner_id: String::from("Dummy business partner id"),
        business_partner_role: String::from("Dummy business partner role"),
        changed_at: Some(now),
        company_name: String::from("Dummy company name"),
        created_at: Some(now),
        currency_code: String::from("GBP"),
        email_address: String::from("Dummy email address"),
        fax_number: Some(String::from("Dummy fax number")),
        legal_form: Some(String::from("Dummy legal form")),
        phone_number: Some(String::from("0123456789")),
        web_address: Some(String::from("Dummy website address")),
    };
    println!("{:#?}", bp);
}
