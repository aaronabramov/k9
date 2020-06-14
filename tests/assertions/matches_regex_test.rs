use anyhow::Result;
use k9::{assert_matches_regex_r, assert_matches_snapshot};

#[test]
fn test_assert_equal() -> Result<()> {
    super::setup_test_env();
    assert_matches_regex_r!("abc", r#"abc"#)?;
    assert_matches_regex_r!("123-234", "\\d{3}-\\d{3}")?;

    let err = assert_matches_regex_r!("123-234", "\\d{3}-\\d{5}").unwrap_err();

    assert_matches_snapshot!(err);
    Ok(())
}
