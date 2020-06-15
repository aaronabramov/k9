use anyhow::Result;
use k9::{assert_greater_than, assert_matches_snapshot};

#[test]
fn test_assert_greater_than() -> Result<()> {
    super::setup_test_env();

    assert!(assert_greater_than!(2, 1).is_none());
    assert!(assert_greater_than!(1, 1).is_some());

    let failure_message = assert_greater_than!(1, 2).expect("must fail");
    assert_matches_snapshot!(failure_message);

    assert!(assert_greater_than!(9.8, 3.14, "Expected left to be greater than right").is_none());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err =
        assert_greater_than!(1, 2, "Expected left to be greater than right").expect("must fail");
    assert_matches_snapshot!(err);
}
