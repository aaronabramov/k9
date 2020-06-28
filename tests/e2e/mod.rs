mod test_utils;

use anyhow::Result;
use k9::assert_equal;
use regex::RegexBuilder;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;
use test_utils::TestProject;

const CAPTURE_TEST_RESULT_RE: &str = "^test (?P<test>\\w+) \\.{3} (?P<result>FAILED|ok)$";

#[derive(Debug, PartialEq, Clone)]
enum TestResult {
    PASS,
    FAIL,
}

impl FromStr for TestResult {
    type Err = ();

    fn from_str(input: &str) -> Result<TestResult, Self::Err> {
        match input {
            "ok" => Ok(TestResult::PASS),
            _ => Ok(TestResult::FAIL),
        }
    }
}

#[test]
fn inline_snapshots() -> Result<()> {
    let assertions: HashMap<&str, TestResult> = [
        ("inline_snapshot", TestResult::FAIL),
        ("failing", TestResult::FAIL),
        ("passing", TestResult::PASS),
    ]
    .iter()
    .cloned()
    .collect();

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
    let output_str = &String::from_utf8(output.stdout)?;
    let regex = RegexBuilder::new(CAPTURE_TEST_RESULT_RE)
        .multi_line(true)
        .build()
        .unwrap();
    let captures = regex.captures_iter(output_str);
    let mut test_results: BTreeMap<String, TestResult> = BTreeMap::new();
    for capture in captures {
        test_results.insert(
            capture["test"].to_string(),
            TestResult::from_str(&capture["result"]).unwrap(),
        );
    }

    // Assertions
    for (key, value) in assertions.into_iter() {
        if let Some(test_result) = test_results.get(key) {
            assert_equal!(&value, test_result);
        }
    }
    Ok(())
}
