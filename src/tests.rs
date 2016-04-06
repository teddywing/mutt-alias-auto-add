use super::build_alias;

#[test]
fn test_build_alias() {
    assert_eq!(
        "alias farnsworth-hubert Hubert Farnsworth <professor@planetexpress.com>",
        build_alias("From: Hubert Farnsworth <professor@planetexpress.com>")
    );

    assert_eq!(
        "alias lab-harvard Harvard Innovation Lab <noreply@eventbrite.com>",
        build_alias("From: Harvard Innovation Lab <noreply@eventbrite.com>")
    );
}
