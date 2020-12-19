use crate::support::TestProject;
use anyhow::Result;

#[test]
fn inline_snapshots() -> Result<()> {
    let p = TestProject::new("inline_snapshots");

    p.write_file("Cargo.toml", crate::support::TEST_CARGO_TOML)?;

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
    k9::assert_matches_inline_snapshot!(format!("{}", std::f64::consts::E));
}

#[test]
fn passing() {}
"#,
    )?;

    let runner = p.run_tests().build().unwrap();
    let test_run = runner.run()?;

    assert!(!test_run.success);

    k9_released::assert_matches_inline_snapshot!(
        format!("{:?}", test_run.test_cases),
        "{\"basic_tests::inline_snapshot\": TestCaseResult { status: Fail }, \"basic_tests::passing\": TestCaseResult { status: Pass }}"
    );
    let runner = p.run_tests().update_snapshots(true).build().unwrap();
    let test_run = runner.run()?;
    assert!(test_run.success);

    let expected = "use k9::*;

#[test]
fn inline_snapshot() {
    assert_matches_inline_snapshot!(format!(\"{}\", std::f64::consts::E), r##\"2.718281828459045\"##);
    k9::assert_matches_inline_snapshot!(
        format!(\"{}\", std::f64::consts::E),
        r##\"2.718281828459045\"##
    );
}

#[test]
fn passing() {}
";

    // Inline snapshot must be updated in the source.
    // NOTE: we're using assert_equal! so we don't test inline snapshot feature
    // using inline snapshots macro. If it's broken, the test could be broken as well
    // and will give false positive.
    k9_local::assert_equal!(
        k9_local::MultilineString::new(p.read_file("basic_tests.rs")?.as_str()),
        k9_local::MultilineString::new(expected)
    );
    k9_local::assert_equal!(p.read_file("basic_tests.rs")?.as_str(), expected);
    k9_released::assert_equal!(p.read_file("basic_tests.rs")?.as_str(), expected);
    assert_eq!(p.read_file("basic_tests.rs")?.as_str(), expected);

    Ok(())
}

#[test]
fn json_serialization() {
    k9_local::snapshot!(
        r#"{"key": ["value1", "value2"]}"#,
        r#"{"key": ["value1", "value2"]}"#
    );
}
