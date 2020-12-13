use k9::*;

#[test]
fn experimental_snapshot() {
    assert_matches_inline_snapshot!("Hello");
}
