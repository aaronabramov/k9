use k9::*;

#[test]
fn assert_equal_basic() {
    assert_equal!(1, 2);
}

#[test]
fn assert_equal_multiline_string() {
    assert_equal!("hello\nworld", "hello\nhow are you?");
}

#[test]
fn snapshot_basic() {
    _snapshot!("a");
}
