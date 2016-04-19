use super::{Alias, is_alias_in_file};

#[test]
fn new_alias_with_only_email() {
    assert_eq!(
        "alias producer@765pro.ductions producer@765pro.ductions",
        Alias::new("From: producer@765pro.ductions").to_string()
    );
}

#[test]
fn new_alias_with_1_name() {
    assert_eq!(
        "alias decim Decim <decim@quindecim.jp>",
        Alias::new("From: Decim <decim@quindecim.jp>").to_string()
    );
}

#[test]
fn new_alias_with_2_names() {
    assert_eq!(
        "alias farnsworth-hubert Hubert Farnsworth <professor@planetexpress.com>",
        Alias::new("From: Hubert Farnsworth <professor@planetexpress.com>").to_string()
    );
}

#[test]
fn new_alias_with_3_names() {
    assert_eq!(
        "alias lab-harvard Harvard Innovation Lab <noreply@eventbrite.com>",
        Alias::new("From: Harvard Innovation Lab <noreply@eventbrite.com>").to_string()
    );
}

#[test]
fn new_alias_with_special_characters() {
    assert_eq!(
        "alias celty-ostrulson \"O'Strulson, Celty\" <celty@dollars.co>",
        Alias::new("From: \"O'Strulson, Celty\" <celty@dollars.co>").to_string()
    );
}


#[test]
fn is_alias_in_file_finds_a_match() {
    is_alias_in_file(
        &Alias {
            alias: "farnsworth-hubert".to_owned(),
            name: "Hubert Farnsworth".to_owned(),
            email: "<professor@planetexpress.com>".to_owned()
        },
        "./testdata/aliases"
    );
}
