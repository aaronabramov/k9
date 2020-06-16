use anyhow::Result;
use k9::{assert_greater_than_or_equal, assert_matches_snapshot};

#[test]
fn test_assert_greater_than_or_equal() -> Result<()> {
    super::setup_test_env();

    assert_greater_than_or_equal!(2, 1).map(|a| a.panic());
    assert!(assert_greater_than_or_equal!(1, 1).is_none());
    assert!(assert_greater_than_or_equal!(0, 1).is_some());
    assert!(assert_greater_than_or_equal!("234", "cde").is_some());
    assert!(assert_greater_than_or_equal!("123", "123").is_none());
    assert!(assert_greater_than_or_equal!("abc", "abc").is_none());
    assert!(assert_greater_than_or_equal!(std::f64::NAN, 1.0f64).is_some());
    assert!(assert_greater_than_or_equal!(1.0f64, 1.0f64).is_none());
    assert!(assert_greater_than_or_equal!(9.8f64, 1.0f64).is_none());

    let failure_message = assert_greater_than_or_equal!(1, 2)
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| a.panic());

    assert!(assert_greater_than_or_equal!(
        9.8,
        3.15,
        "Expected left to greater than or equal to right"
    )
    .is_none());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err =
        assert_greater_than_or_equal!(1, 2, "Expected left to greater than or equal to right")
            .expect("must fail")
            .get_failure_message();
    assert_matches_snapshot!(err).map(|a| a.panic());
}
