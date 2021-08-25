use anyhow::Result;
use k9::{assert_matches_snapshot, assert_ok};

#[test]
fn test_assert_ok() -> Result<()> {
    super::setup_test_env();

    assert!(assert_ok!(Result::<_, ()>::Ok(1)).is_none());
    assert!(assert_ok!(Result::<_, ()>::Ok(9.8f64)).is_none());
    assert!(assert_ok!(Result::<_, ()>::Ok("Hola!")).is_none());
    assert!(assert_ok!(Result::<_, ()>::Ok(vec![1, 2, 3])).is_none());
    assert!(assert_ok!(Result::<_, ()>::Ok([1, 2, 3])).is_none());
    assert!(assert_ok!(Result::<_, ()>::Ok(&[1, 2, 3])).is_none());
    assert!(assert_ok!(Result::<_, ()>::Ok("Hello, World!"), "Expected Greeting").is_none());

    assert!(assert_ok!(Result::<(), _>::Err("Invalid number")).is_some());
    if let Err(e) = "xc".parse::<u32>() {
        assert!(assert_ok!(Result::<(), _>::Err(e)).is_some());
    }

    let failure_message = assert_ok!(Result::<(), _>::Err("Parsing Error"))
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(failure_message).map(|a| panic!("{:?}", a));
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_ok!(
        Result::<(), _>::Err("Version Mismatch!"),
        "Expected Value to be Ok(T)"
    )
    .expect("must fail")
    .get_failure_message();
    assert_matches_snapshot!(err).map(|a| panic!("{:?}", a));
}
