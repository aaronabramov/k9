use anyhow::Result;
use k9::*;

#[test]
fn test_assert_equal() -> Result<()> {
    super::setup_test_env();
    assert!(assert_matches_regex!("abc", r#"abc"#).is_none());
    assert!(assert_matches_regex!("123-234", "\\d{3}-\\d{3}").is_none());

    let err = assert_matches_regex!("123-234", "\\d{3}-\\d{5}")
        .expect("must fail")
        .get_failure_message();

    assert_matches_snapshot!(err).map(|a| panic!("{:?}", a));
    Ok(())
}
