use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

use alias::{Alias, AliasSearchError};

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
fn alias_find_in_file_email_already_exists() {
    let alias = Alias {
        alias: "farnsworth-hubert".to_owned(),
        name: "Hubert Farnsworth".to_owned(),
        email: "<professor@planetexpress.com>".to_owned()
    };

    assert_eq!(
        Err(AliasSearchError::EmailExists),
        alias.find_in_file("./testdata/aliases")
    );
}

#[test]
fn alias_find_in_file_alias_is_new() {
    let alias = Alias {
        alias: "fry-philip".to_owned(),
        name: "Philip Fry".to_owned(),
        email: "<fry@planetexpress.com>".to_owned()
    };

    assert_eq!(
        Err(AliasSearchError::NotFound),
        alias.find_in_file("./testdata/aliases")
    );
}

#[test]
fn alias_find_in_file_finds_a_match() {
    let alias = Alias {
        alias: "farnsworth-hubert".to_owned(),
        name: "Hubert Farnsworth".to_owned(),
        email: "<goodnewseveryone@planetexpress.com>".to_owned()
    };

    assert_eq!(
        Ok(vec![
            "farnsworth-hubert".to_owned(),
            "farnsworth-hubert-2".to_owned()
        ]),
        alias.find_in_file("./testdata/aliases")
    );
}


const UPDATE_ALIAS_ID_ALIAS_IDENTIFIER: &'static str = "hooves-derpy";

fn update_alias_id_sample_alias() -> Alias {
    Alias {
        alias: UPDATE_ALIAS_ID_ALIAS_IDENTIFIER.to_owned(),
        name: "Derpy Hooves".to_owned(),
        email: "derpyhooves@postmaster.pv".to_owned()
    }
}

#[test]
fn update_alias_id_does_nothing_given_an_empty_vector() {
    let mut alias = update_alias_id_sample_alias();
    alias.update_alias_id(vec![]);

    assert_eq!(UPDATE_ALIAS_ID_ALIAS_IDENTIFIER, &alias.alias);
}

#[test]
fn update_alias_id_increments_alias() {
    let mut alias = update_alias_id_sample_alias();
    alias.update_alias_id(vec![
        "hooves-derpy".to_owned(),
        "hooves-derpy-2".to_owned(),
        "hooves-derpy-3".to_owned(),
        "hooves-derpy-4".to_owned()
    ]);

    assert_eq!("hooves-derpy-5", &alias.alias);
}


fn alias_write_to_file_helper(alias: &mut Alias) -> String {
    // Create a new test file
    let test_file = "./testdata/write_to_file";
    fs::copy("./testdata/aliases", test_file).expect("Alias file copy failed");

    // Write a duplicate alias so that `write_to_file` is able to append a
    // new one
    let mut f = OpenOptions::new().append(true).open(test_file)
        .expect("Failed to open test file for appending");
    writeln!(f, "{}", Alias { email: "derpy@home.pv".to_owned(), .. alias.clone() }
            .to_string())
        .expect("Failed to append matching alias");

    // Write our new alias to the file
    alias.write_to_file(test_file).expect("`write_to_file` failed");

    // Get the file's contents for testing
    let mut f = File::open(test_file).expect("Failed to open test file");
    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents).expect("Failed to read test file contents");
    let file_contents: Vec<&str> = file_contents.split('\n').collect();
    fs::remove_file(test_file).expect("Failed to delete test file");

    file_contents[file_contents.len() - 2].to_string()
}

#[test]
fn alias_write_to_file_must_write_given_alias_to_file() {
    let mut alias = update_alias_id_sample_alias();
    let alias_line = alias_write_to_file_helper(&mut alias);

    assert_eq!(alias.to_string(), alias_line);
}
