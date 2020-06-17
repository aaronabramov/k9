use anyhow::Result;
use k9::{assert_contains, assert_matches_snapshot};

#[test]
fn test_assert_contains() -> Result<()> {
    super::setup_test_env();

    assert!(assert_contains!(vec![1, 2, 3, 4, 5], 5).is_none());
    assert!(assert_contains!(vec![9.8, 3.15], 2.5).is_some());
    assert!(assert_contains!(vec![9.8, 3.15], 9.8).is_none());
    assert!(assert_contains!(&[9.8, 3.15], &2.5).is_some());
    assert!(assert_contains!(&[9.8, 3.15], &9.8).is_none());
    assert!(assert_contains!(&[(1, 2), (2, 3)], &(3, 4)).is_some());
    assert!(assert_contains!(&[(1, 2), (2, 3), (3, 4)], &(3, 4)).is_none());

    assert!(assert_contains!(vec!["canine", "k9", "k-9"], "k-9").is_none());
    assert!(assert_contains!(vec!["a", "b", "c"], "A").is_some());

    let failure_message = assert_contains!(vec![1, 2], 5)
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| a.panic());

    assert!(assert_contains!(
        vec![98, 99],
        99,
        "Expected left value to contain right value"
    )
    .is_none());
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_contains!(
        vec![98, 99],
        100,
        "Expected left value to contain right value"
    )
    .expect("must fail")
    .get_failure_message();
    assert_matches_snapshot!(err).map(|a| a.panic());
}
