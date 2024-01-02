#[test]
fn test_matches_inline_snapshot() {
    k9_stable::snapshot!(
        "1234567890".repeat(5).chars().rev().collect::<String>(),
        r##"09876543210987654321098765432109876543210987654321"##
    );

    k9_stable::snapshot!(
        r#"escaped string \": \"{\\"namespace\\":\\"www\\",\\"tim"#,
        r#"escaped string \\": \\"{\\\\"namespace\\\\":\\\\"www\\\\",\\\\"tim"#
    );

    k9_stable::snapshot!(
        "escaped string  #r\"blahblah\"#",
        r##"escaped string  #r"blahblah"#"##
    );
}
