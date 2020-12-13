use k9::*;

#[test]
fn snapshot_test() {
    assert_matches_snapshot!("Hello");
    assert_matches_inline_snapshot!("hello", r##"hello"##);
}
