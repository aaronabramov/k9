use crate::test_utils::TestProject;
use anyhow::Result;
use k9_released::*;

#[test]
fn inline_snapshots() -> Result<()> {
    let p = TestProject::new();

    p.write_file("Cargo.toml", crate::test_utils::TEST_CARGO_TOML)?;

    p.write_file(
        "lib.rs",
        r#"
#[cfg(test)]
mod basic_tests;
"#,
    )?;

    p.write_file(
        "basic_tests.rs",
        r#"
use k9::*;

#[test]
fn inline_snapshot() {
    assert_matches_inline_snapshot!(format!("{}", std::f64::consts::E));
}

#[test]
fn passing() {}
    "#,
    )?;

    let runner = p.run_tests().build().unwrap();
    let test_run = runner.run()?;

    assert!(!test_run.success);

    dbg!(&test_run);
    assert_matches_inline_snapshot!(
        format!("{:?}", test_run.test_cases),
        "{\"basic_tests::inline_snapshot\": TestCaseResult { status: Fail }, \"basic_tests::passing\": TestCaseResult { status: Pass }}"
    );
    let runner = p.run_tests().update_snapshots(true).build().unwrap();
    let test_run = runner.run()?;
    assert!(test_run.success);

    // Inline snapshot must be updated in the source.
    // NOTE: we're using assert_equal! so we don't test inline snapshot feature
    // using inline snapshots macro. If it's broken, the test could be broken as well
    // and will give false positive.
    assert_eq!(
        p.read_file("basic_tests.rs")?.as_str(),
        "use k9::*;

#[test]
fn inline_snapshot() {
    assert_matches_inline_snapshot!(format!(\"{}\", std::f64::consts::E), r##\"2.718281828459045\"##);
}

#[test]
fn passing() {}
"
    );

    Ok(())
}
