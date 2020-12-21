use anyhow::Result;
use k9::{assert_lesser_than, assert_matches_snapshot};

#[test]
fn test_assert_lesser_than() -> Result<()> {
    super::setup_test_env();

    assert_lesser_than!(1, 2).map(|a| panic!(a));
    assert!(assert_lesser_than!(1, 1).is_some());
    assert!(assert_lesser_than!(1, 0).is_some());
    assert!(assert_lesser_than!("cde", "234").is_some());
    assert!(assert_lesser_than!(std::f64::NAN, 1.0f64).is_some());

    let failure_message = assert_lesser_than!(2, 1)
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| panic!(a));

    assert!(assert_lesser_than!(3.15, 9.8, "Expected left to be lesser than right").is_none());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_lesser_than!(2, 1, "Expected left to be lesser than right")
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(err).map(|a| panic!(a));
}
