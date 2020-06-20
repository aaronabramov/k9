extern crate k9_published;

#[test]
fn smoke_test() {
    k9_published::assert_equal!(1, 1);

    k9_published::assert_err!(Err("Error!"));

    k9_published::assert_greater_than!(2, 1);

    k9_published::assert_greater_than_or_equal!(1, 1);

    k9_published::assert_lesser_than!(1, 2);

    k9_published::assert_lesser_than_or_equal!(1, 1);

    k9_published::assert_matches_regex!("abc", "abc");

    k9_published::assert_ok!(Ok(2));
}
