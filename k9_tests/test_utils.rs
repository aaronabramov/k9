use anyhow::{Context, Result};
use derive_builder::Builder;
use rand::prelude::*;
use regex::RegexBuilder;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

const E2E_TEMP_DIR: &str = "e2e_tmp_dir";
const CAPTURE_TEST_RESULT_RE: &str = "^test (?P<test>[:_\\w]+) \\.{3} (?P<result>FAILED|ok)$";

pub const TEST_CARGO_TOML: &str = r#"
[workspace]

[package]
name = "k9_e2e_test_project"
version = "0.1.0"
authors = ["Aaron Abramov <aaron@abramov.io>"]
edition = "2018"

[dependencies]
k9 = { path = "../../../k9" }

[lib]
name = "test"
path = "lib.rs"
"#;

#[derive(Builder, Default, Debug)]
#[builder(default, setter(into))]
pub struct TestRun {
    update_snapshots: bool,
    root_dir: PathBuf,
}

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

pub struct TestProject {
    pub root_dir: PathBuf,
}

impl TestProject {
    pub fn new() -> Self {
        let mut root_dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory"),
        );
        let mut rng = rand::thread_rng();

        let r: u64 = rng.gen();

        root_dir.push(E2E_TEMP_DIR);
        root_dir.push(format!("{}", r));

        Self { root_dir }
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let mut absolute_path = self.root_dir.clone();
        absolute_path.push(path);
        let dir = absolute_path.parent().unwrap();
        fs::create_dir_all(dir)?;
        fs::write(absolute_path, content)?;
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<String> {
        let mut absolute_path = self.root_dir.clone();
        absolute_path.push(path);
        fs::read_to_string(&absolute_path).context("can't read file")
    }

    pub fn run_tests(&self) -> TestRunBuilder {
        let mut builder = TestRunBuilder::default();
        builder.root_dir(self.root_dir.clone());
        builder
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        // could have been never cerated. don't care about result
        // let _result = fs::remove_dir_all(&self.root_dir);
    }
}
