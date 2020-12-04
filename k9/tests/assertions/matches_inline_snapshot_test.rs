use crate::assert_matches_inline_snapshot;

#[test]
fn test_matches_inline_snapshot() {
    assert_matches_inline_snapshot!(
        "1234567890".repeat(5).chars().rev().collect::<String>(),
        r##"09876543210987654321098765432109876543210987654321"##
    );

    assert_matches_inline_snapshot!(
        r#"escaped string \": \"{\\"namespace\\":\\"www\\",\\"tim"#,
        r##"escaped string \": \"{\\"namespace\\":\\"www\\",\\"tim"##
    );

    assert_matches_inline_snapshot!(
        "escaped string  #r\"blahblah\"#",
        r##"escaped string  #r"blahblah"#"##
    );
}
