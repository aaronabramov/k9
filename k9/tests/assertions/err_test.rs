use anyhow::Result;
use k9::{assert_err, assert_matches_snapshot};

#[test]
fn test_assert_err() -> Result<()> {
    super::setup_test_env();

    assert!(assert_err!(Err("Invalid number")).is_none());
    assert!(assert_err!(Err("Invalid path"), "Expected to fail").is_none());

    assert!(assert_err!(Ok(1)).is_some());
    assert!(assert_err!(Ok("Hola!")).is_some());
    assert!(assert_err!(Ok(vec![1, 2, 3])).is_some());

    let failure_message = assert_err!(Ok([1, 2, 3]))
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| a.panic());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_err!(Ok(1), "Expected Value to be Err(T)")
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(err).map(|a| a.panic());
}
