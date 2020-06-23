use crate::assert_matches_inline_snapshot;

#[test]
fn test_matches_inline_snapshot() {
    assert_matches_inline_snapshot!(
        "1234567890".repeat(5).chars().rev().collect::<String>(),
        "09876543210987654321098765432109876543210987654321"
    );
}
