#![allow(clippy::if_then_panic)]

mod basic_fixture;
mod failure_messages;
mod inline_snap_test;
mod support;

use k9_released::*;

#[test]
fn smoke_test() {
    assert_equal!(1, 1);

    assert_err!(Result::<(), _>::Err("Error!"));

    assert_greater_than!(2, 1);

    assert_greater_than_or_equal!(1, 1);

    assert_lesser_than!(1, 2);

    assert_lesser_than_or_equal!(1, 1);

    assert_matches_regex!("abc", "abc");

    assert_ok!(Result::<_, ()>::Ok(2));

    snapshot!(format!("{:?}", Some(true)), "Some(true)");
}
