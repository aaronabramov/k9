mod test_utils;

use anyhow::Result;
use regex::RegexBuilder;
use std::collections::BTreeMap;
use test_utils::TestProject;

#[derive(Debug)]
enum TestResult {
    PASS,
    FAIL,
}

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
    let regex = RegexBuilder::new("^test (?P<test>\\w+) \\.{3} (?P<result>FAILED|ok)$")
        .multi_line(true)
        .build()
        .unwrap();
    let output_str = &String::from_utf8(output.stdout)?;
    let captures = regex.captures_iter(output_str);
    let mut test_results: BTreeMap<String, TestResult> = BTreeMap::new();
    for capture in captures {
        if &capture["test"] == "ok" {
            test_results.insert(capture["test"].to_string(), TestResult::PASS);
        } else if &capture["test"] == "FAILED" {
            test_results.insert(capture["test"].to_string(), TestResult::FAIL);
        }
    }
    dbg!(test_results);
    Ok(())
}
