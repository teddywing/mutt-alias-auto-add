use super::build_alias;

#[test]
fn build_alias_with_only_email() {
    assert_eq!(
        "alias producer@765pro.ductions producer@765pro.ductions",
        build_alias("From: producer@765pro.ductions")
    );
}

#[test]
fn build_alias_with_1_name() {
    assert_eq!(
        "alias decim Decim <decim@quindecim.jp>",
        build_alias("From: Decim <decim@quindecim.jp>")
    );
}

#[test]
fn build_alias_with_2_names() {
    assert_eq!(
        "alias farnsworth-hubert Hubert Farnsworth <professor@planetexpress.com>",
        build_alias("From: Hubert Farnsworth <professor@planetexpress.com>")
    );
}

#[test]
fn build_alias_with_3_names() {
    assert_eq!(
        "alias lab-harvard Harvard Innovation Lab <noreply@eventbrite.com>",
        build_alias("From: Harvard Innovation Lab <noreply@eventbrite.com>")
    );
}

#[test]
fn build_alias_with_special_characters() {
    assert_eq!(
        "alias celty-ostrulson \"O'Strulson, Celty\" <celty@dollars.co>",
        build_alias("From: \"O'Strulson, Celty\" <celty@dollars.co>")
    );
}
