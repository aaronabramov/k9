use anyhow::Result;
use k9::{assert_err_matches_regex, assert_matches_snapshot};

#[test]
fn test() -> Result<()> {
    super::setup_test_env();
    let result: Result<()> = Err(anyhow::anyhow!("123 error message"));
    assert!(assert_err_matches_regex!(result, r#"123"#).is_none());

    let result: Result<()> = Err(anyhow::anyhow!("123 error message"));
    let err = assert_err_matches_regex!(result, "\\d{3}-\\d{5}")
        .expect("must fail")
        .get_failure_message();

    assert_matches_snapshot!(err).map(|a| panic!(a));
    Ok(())
}

#[test]
fn test_when_ok() {
    super::setup_test_env();
    let result: Result<()> = Ok(());
    let err = assert_err_matches_regex!(result, r#"123"#)
        .expect("must_fail")
        .get_failure_message();

    assert_matches_snapshot!(err).map(|a| panic!(a));
}
