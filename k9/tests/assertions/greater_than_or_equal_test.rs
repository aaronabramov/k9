use crate::assertion_message;
use anyhow::Result;
use k9::assert_greater_than_or_equal;

#[test]
fn test_assert_greater_than_or_equal() -> Result<()> {
    super::setup_test_env();

    assert_greater_than_or_equal!(2, 1).map(|a| panic!("{:?}", a));
    assert!(assert_greater_than_or_equal!(1, 1).is_none());
    assert!(assert_greater_than_or_equal!(0, 1).is_some());
    assert!(assert_greater_than_or_equal!("234", "cde").is_some());
    assert!(assert_greater_than_or_equal!("123", "123").is_none());
    assert!(assert_greater_than_or_equal!("abc", "abc").is_none());
    assert!(assert_greater_than_or_equal!(std::f64::NAN, 1.0f64).is_some());
    assert!(assert_greater_than_or_equal!(1.0f64, 1.0f64).is_none());
    assert!(assert_greater_than_or_equal!(9.8f64, 1.0f64).is_none());

    k9_stable::assert_matches_inline_snapshot!(
        assertion_message(assert_greater_than_or_equal!(1, 2)),
        "
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_greater_than_or_equal!(1, 2);

Assertion Failure!

Expected Left value to be greater than or equal to Right value

Left value:  1
Right value: 2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"
    );

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
    k9_stable::assert_matches_inline_snapshot!(
        assertion_message(assert_greater_than_or_equal!(
            1,
            2,
            "Expected left to greater than or equal to right"
        )),
        "
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_greater_than_or_equal!(1, 2, \"Expected left to greater than or equal to right\");

Expected left to greater than or equal to right

Expected Left value to be greater than or equal to Right value

Left value:  1
Right value: 2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"
    );
}
