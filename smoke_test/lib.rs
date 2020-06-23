#[test]
fn smoke_test() {
    use k9_published::*;

    assert_equal!(1, 1);

    assert_err!(Err("Error!"));

    assert_greater_than!(2, 1);

    assert_greater_than_or_equal!(1, 1);

    assert_lesser_than!(1, 2);

    assert_lesser_than_or_equal!(1, 1);

    assert_matches_regex!("abc", "abc");

    assert_ok!(Ok(2));

    assert_matches_inline_snapshot!(format!("{:?}", Some(true)), "Some(true)");
}
