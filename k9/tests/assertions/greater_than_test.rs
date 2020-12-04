use anyhow::Result;
use k9::{assert_greater_than, assert_matches_snapshot};

#[test]
fn test_assert_greater_than() -> Result<()> {
    super::setup_test_env();

    assert_greater_than!(2, 1).map(|a| a.panic());
    assert!(assert_greater_than!(1, 1).is_some());
    assert!(assert_greater_than!(0, 1).is_some());
    assert!(assert_greater_than!("234", "cde").is_some());
    assert!(assert_greater_than!(std::f64::NAN, 1.0f64).is_some());

    let failure_message = assert_greater_than!(1, 2)
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| a.panic());

    assert!(assert_greater_than!(9.8, 3.15, "Expected left to be greater than right").is_none());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_greater_than!(1, 2, "Expected left to be greater than right")
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(err).map(|a| a.panic());
}
