use anyhow::Result;
use k9::{assert_matches_snapshot, assert_ok};

#[test]
fn test_assert_ok() -> Result<()> {
    super::setup_test_env();

    assert!(assert_ok!(Ok(1)).is_none());
    assert!(assert_ok!(Ok(9.8f64)).is_none());
    assert!(assert_ok!(Ok("Hola!")).is_none());
    assert!(assert_ok!(Ok(vec![1, 2, 3])).is_none());
    assert!(assert_ok!(Ok([1, 2, 3])).is_none());
    assert!(assert_ok!(Ok(&[1, 2, 3])).is_none());
    assert!(assert_ok!(Ok("Hello, World!"), "Expected Greeting").is_none());

    assert!(assert_ok!(Err("Invalid number")).is_some());
    if let Err(e) = "xc".parse::<u32>() {
        assert!(assert_ok!(Err(e)).is_some());
    }

    let failure_message = assert_ok!(Err("Parsing Error"))
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| a.panic());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_ok!(Err("Version Mismatch!"), "Expected Value to be Ok(T)")
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(err).map(|a| a.panic());
}
