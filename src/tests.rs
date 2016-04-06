use super::build_alias;

#[test]
fn test_build_alias() {
    assert_eq!(
        "alias producer@765pro.ductions producer@765pro.ductions",
        build_alias("From: producer@765pro.ductions")
    );

    assert_eq!(
        "alias decim Decim <decim@quindecim.jp>",
        build_alias("From: Decim <decim@quindecim.jp>")
    );

    assert_eq!(
        "alias farnsworth-hubert Hubert Farnsworth <professor@planetexpress.com>",
        build_alias("From: Hubert Farnsworth <professor@planetexpress.com>")
    );

    assert_eq!(
        "alias lab-harvard Harvard Innovation Lab <noreply@eventbrite.com>",
        build_alias("From: Harvard Innovation Lab <noreply@eventbrite.com>")
    );
}
