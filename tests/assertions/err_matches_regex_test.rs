use anyhow::Result;
use k9::{assert_err_matches_regex, assert_err_matches_regex_r, assert_matches_snapshot};

#[test]
fn test_assert_equal() -> Result<()> {
    super::setup_test_env();
    let result: Result<()> = Err(anyhow::anyhow!("123 error message"));
    assert_err_matches_regex!(result, r#"123"#);
    let result: Result<()> = Ok(());
    let assertion_result = assert_err_matches_regex_r!(result, r#"123"#);
    assert!(assertion_result.is_err());

    let result: Result<()> = Err(anyhow::anyhow!("123 error message"));
    let err = assert_err_matches_regex_r!(result, "\\d{3}-\\d{5}").unwrap_err();

    assert_matches_snapshot!(err);
    Ok(())
}
