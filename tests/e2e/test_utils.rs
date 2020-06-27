use anyhow::Result;
use rand::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const E2E_TEMP_DIR: &str = "e2e_tmp_dir";

pub const TEST_CARGO_TOML: &str = r#"
[package]
name = "k9_e2e_test_project"
version = "0.1.0"
authors = ["Aaron Abramov <aaron@abramov.io>"]
edition = "2018"

[dependencies]
k9 = { path = "../../" }

[lib]
name = "test"
path = "lib.rs"
"#;

pub struct TestProject {
    pub root_dir: PathBuf,
}

impl TestProject {
    pub fn new() -> Self {
        let mut root_dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory"),
        );
        let mut rng = rand::thread_rng();

        let r: i64 = rng.gen();

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

    pub fn run(&self, cmd: &str, args: &[&str]) -> Result<std::process::Output> {
        let output = Command::new(cmd)
            .current_dir(&self.root_dir)
            .args(args)
            .output()?;
        Ok(output)
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        // could have been never cerated. don't care about result
        // let _result = fs::remove_dir_all(&self.root_dir);
    }
}
