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
        r##"
use k9::*;

#[test]
fn inline_snapshot() {
    snapshot!(format!("{}", std::f64::consts::E));
    k9::snapshot!(format!("{}", std::f64::consts::E));
    snapshot!(format!("much to escape--> \"{} <--in here", "#".repeat(7)));
}

#[test]
fn passing() {}
"##,
    )?;

    let runner = p.run_tests().build().unwrap();
    let test_run = runner.run()?;

    assert!(!test_run.success);

    k9_released::snapshot!(
        test_run.test_cases,
        r#"
{
    "basic_tests::inline_snapshot": TestCaseResult {
        status: Fail,
    },
    "basic_tests::passing": TestCaseResult {
        status: Pass,
    },
}
"#
    );
    let runner = p.run_tests().update_snapshots(true).build().unwrap();
    let test_run = runner.run()?;
    assert!(test_run.success);

    let expected = r#########"use k9::*;

#[test]
fn inline_snapshot() {
    snapshot!(format!("{}", std::f64::consts::E), "2.718281828459045");
    k9::snapshot!(format!("{}", std::f64::consts::E), "2.718281828459045");
    snapshot!(
        format!("much to escape--> \"{} <--in here", "#".repeat(7)),
        r########"much to escape--> "####### <--in here"########
    );
}

#[test]
fn passing() {}
"#########;

    // Inline snapshot must be updated in the source.
    // NOTE: we're using assert_equal! so we don't test inline snapshot feature
    // using inline snapshots macro. If it's broken, the test could be broken as well
    // and will give false positive.
    k9_released::assert_equal!(
        k9_local::MultilineString::new(p.read_file("basic_tests.rs")?.as_str()),
        k9_local::MultilineString::new(expected)
    );
    k9_released::assert_equal!(p.read_file("basic_tests.rs")?.as_str(), expected);
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

#[test]
fn single_quotes_r_str() {
    k9_local::snapshot!(
        r#"this is 'wrapped' in single quotes in # string"#,
        "this is 'wrapped' in single quotes in # string"
    );
}

#[test]
fn escaped_single_quotes() {
    k9_local::snapshot!(
        "this is \'wrapped\' in escaped single quotes",
        "this is 'wrapped' in escaped single quotes"
    );
}

#[test]
fn single_quotes() {
    k9_local::snapshot!(
        "this is 'wrapped' in single quotes",
        "this is 'wrapped' in single quotes"
    );
}

#[test]
fn escaped_double_quotes() {
    k9_local::snapshot!(
        "this is an escaped \" double quote ",
        r#"this is an escaped " double quote "#
    );
}

#[test]
fn double_quotes_r_str() {
    k9_local::snapshot!(
        r#"this is an double " qote inside # stirng"#,
        r#"this is an double " qote inside # stirng"#
    );
}

#[test]
fn newline_escaping_serialization() {
    k9_local::snapshot!(
        r#" escaped nl char \n"#,
        // This is an annoying side effect, since having \n here makes it bouble
        // escaped, and unescaping it leaves trailing \
        r"
 escaped nl char \

"
    );
}
