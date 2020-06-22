use anyhow::Result;
use k9::*;

#[test]
fn test_matches_inline_snapshot() -> Result<()> {
    assert_matches_inline_snapshot!("hello".chars().rev().collect::<String>(), "olleh")
        .map(|a| a.panic());
    Ok(())
}
