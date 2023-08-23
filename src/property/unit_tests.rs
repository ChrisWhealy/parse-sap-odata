#[test]
fn should_convert_unsafe_property_name() {
    use check_keyword::CheckKeyword;
    let kw1 = "type";
    let kw2 = "match";

    assert!(kw1.is_keyword());
    assert!(kw2.is_keyword());
    assert_eq!(kw1.into_safe(), "r#type");
    assert_eq!(kw2.into_safe(), "r#match");
}

#[test]
fn should_not_convert_safe_property_name() {
    use check_keyword::CheckKeyword;
    let kw = "wibble";

    assert!(!kw.is_keyword());
    assert_eq!(kw.into_safe(), "wibble");
}
