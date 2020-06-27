mod test_utils;

use anyhow::Result;
use test_utils::TestProject;

#[test]
fn inline_snapshots() -> Result<()> {
    let p = TestProject::new();
    p.write_file("Cargo.toml", test_utils::TEST_CARGO_TOML)?;
    p.write_file(
        "lib.rs",
        r#"
use k9::*;

#[test]
fn inline_snapshot() {
    assert_matches_inline_snapshot!(format!("{}", std::f64::consts::E));
}

#[test]
fn failing() {
    assert_equal!(1, 2);
}

#[test]
fn passing() {}
"#,
    )?;

    let output = p.run("cargo", &["test"])?;

    println!("{}", String::from_utf8(output.stdout)?);
    Ok(())
}
