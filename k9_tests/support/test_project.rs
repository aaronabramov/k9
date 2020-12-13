use super::TestRunBuilder;
use anyhow::{Context, Result};
use rand::prelude::*;
use std::fs;
use std::path::PathBuf;

const E2E_TEMP_DIR: &str = "e2e_tmp_dir";

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
