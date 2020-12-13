use anyhow::{Context, Result};
use derive_builder::Builder;
use regex::RegexBuilder;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

const CAPTURE_TEST_RESULT_RE: &str = "^test (?P<test>[:_\\w]+) \\.{3} (?P<result>FAILED|ok)$";

#[derive(Builder, Default, Debug)]
#[builder(default, setter(into))]
pub struct TestRun {
    update_snapshots: bool,
    root_dir: PathBuf,
}

pub type _TestRunBuilder = TestRunBuilder;

impl TestRun {
    pub fn run(&self) -> Result<TestRunResult> {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.root_dir).arg("test");

        if self.update_snapshots {
            cmd.env("K9_UPDATE_SNAPSHOTS", "1");
        } else {
            // to make sure that we don't propagate parent process setting
            cmd.env_remove("K9_UPDATE_SNAPSHOTS");
        }

        let output = cmd.output()?;

        let exit_code = output.status.code();
        let success = output.status.success();

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;
        let regex = RegexBuilder::new(CAPTURE_TEST_RESULT_RE)
            .multi_line(true)
            .build()
            .unwrap();
        let captures = regex.captures_iter(&stdout);
        let mut test_cases: BTreeMap<String, TestCaseResult> = BTreeMap::new();
        for capture in captures {
            test_cases.insert(
                capture["test"].to_string(),
                TestCaseResult {
                    status: TestCaseStatus::from_str(&capture["result"])
                        .context("can't parse status")?,
                },
            );
        }

        Ok(TestRunResult {
            exit_code,
            stderr,
            stdout,
            success,
            test_cases,
        })
    }
}

#[derive(Debug)]
pub struct TestRunResult {
    pub exit_code: Option<i32>,
    pub stderr: String,
    pub stdout: String,
    pub success: bool,
    pub test_cases: BTreeMap<String, TestCaseResult>,
}

#[derive(Debug, PartialEq)]
pub struct TestCaseResult {
    pub status: TestCaseStatus,
}

// Represent a single test case. e.g. `#[test]\nfn my_test() {}`
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TestCaseStatus {
    Pass,
    Fail,
}

impl FromStr for TestCaseStatus {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<TestCaseStatus> {
        match input {
            "ok" => Ok(TestCaseStatus::Pass),
            "FAILED" => Ok(TestCaseStatus::Fail),
            _ => Err(anyhow::anyhow!("Unknown test status: `{}`", input)),
        }
    }
}
