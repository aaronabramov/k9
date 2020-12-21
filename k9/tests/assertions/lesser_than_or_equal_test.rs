use anyhow::Result;
use k9::{assert_lesser_than_or_equal, assert_matches_snapshot};

#[test]
fn test_assert_lesser_than_or_equal() -> Result<()> {
    super::setup_test_env();

    assert_lesser_than_or_equal!(1, 2).map(|a| panic!(a));
    assert!(assert_lesser_than_or_equal!(1, 1).is_none());
    assert!(assert_lesser_than_or_equal!(1, 0).is_some());
    assert!(assert_lesser_than_or_equal!("cde", "234").is_some());
    assert!(assert_lesser_than_or_equal!("123", "123").is_none());
    assert!(assert_lesser_than_or_equal!("abc", "abc").is_none());
    assert!(assert_lesser_than_or_equal!(std::f64::NAN, 1.0f64).is_some());
    assert!(assert_lesser_than_or_equal!(1.0f64, 1.0f64).is_none());
    assert!(assert_lesser_than_or_equal!(1.0f64, 9.8f64).is_none());

    let failure_message = assert_lesser_than_or_equal!(2, 1)
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| panic!(a));

    assert!(assert_lesser_than_or_equal!(
        3.15,
        9.8,
        "Expected left to lesser than or equal to right"
    )
    .is_none());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_lesser_than_or_equal!(2, 1, "Expected left to lesser than or equal to right")
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(err).map(|a| panic!(a));
}
