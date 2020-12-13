mod test_project;
mod test_run;

pub use test_project::TestProject;
pub use test_run::{TestRun, TestRunResult, _TestRunBuilder as TestRunBuilder};

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
