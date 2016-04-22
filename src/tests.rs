use super::{Alias, AliasSearchError, find_alias_in_file};

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
fn find_alias_in_file_email_already_exists() {
    assert_eq!(
        Err(AliasSearchError::EmailExists),
        find_alias_in_file(
            &Alias {
                alias: "farnsworth-hubert".to_owned(),
                name: "Hubert Farnsworth".to_owned(),
                email: "<professor@planetexpress.com>".to_owned()
            },
            "./testdata/aliases"
        )
    );
}

#[test]
fn find_alias_in_file_alias_is_new() {
    assert_eq!(
        Err(AliasSearchError::NotFound),
        find_alias_in_file(
            &Alias {
                alias: "fry-philip".to_owned(),
                name: "Philip Fry".to_owned(),
                email: "<fry@planetexpress.com>".to_owned()
            },
            "./testdata/aliases"
        )
    );
}

#[test]
fn find_alias_in_file_finds_a_match() {
    assert_eq!(
        Ok(vec![
            "farnsworth-hubert".to_owned(),
            "farnsworth-hubert-2".to_owned()
        ]),
        find_alias_in_file(
            &Alias {
                alias: "farnsworth-hubert".to_owned(),
                name: "Hubert Farnsworth".to_owned(),
                email: "<goodnewseveryone@planetexpress.com>".to_owned()
            },
            "./testdata/aliases"
        )
    );
}


#[test]
fn update_alias_id_does_nothing_given_an_empty_vector() {
    let alias_identifier = "hooves-derpy";
    let mut alias = Alias {
        alias: alias_identifier.to_owned(),
        name: "Derpy Hooves".to_owned(),
        email: "derpyhooves@postmaster.pv".to_owned()
    };
    alias.update_alias_id(vec![]);

    assert_eq!(alias_identifier, &alias.alias);
}

#[test]
fn update_alias_id_increments_alias() {
    let alias_identifier = "hooves-derpy";
    let mut alias = Alias {
        alias: alias_identifier.to_owned(),
        name: "Derpy Hooves".to_owned(),
        email: "derpyhooves@postmaster.pv".to_owned()
    };
    alias.update_alias_id(vec![
        "hooves-derpy".to_owned(),
        "hooves-derpy-2".to_owned(),
        "hooves-derpy-3".to_owned(),
        "hooves-derpy-4".to_owned()
    ]);

    assert_eq!("hooves-derpy-5", &alias.alias);
}
