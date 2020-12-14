use super::{TestRunBuilder, TestRunResult};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

const E2E_TEMP_DIR: &str = "__E2E_TEMP_DIR__";

pub struct TestProject {
    pub root_dir: PathBuf,
}

impl TestProject {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let root_dir = prepare_temp_dir(name);
        Self { root_dir }
    }

    pub fn fixture<S: Into<String>>(name: S, fixture_path: S) -> Self {
        let mut fixture_root = cargo_root();
        fixture_root.push(fixture_path.into());
        let fixture_root = fixture_root.canonicalize().unwrap_or_else(|e| {
            panic!(
                "failed to resolve fixture_path; `{}`. \n{:?}\n",
                &fixture_root.display().to_string(),
                e
            )
        });

        let root_dir = prepare_temp_dir(name);

        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;

        fs_extra::dir::copy(fixture_root, &root_dir, &options)
            .expect("failed to copy fixture project to a temp dir");

        let mut target_dir = root_dir.clone();
        target_dir.push("target");
        // Copying target messes up permissions and `cargo` blows up
        std::fs::remove_dir_all(target_dir).ok();

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

    pub fn run_matching_tests<S: Into<String>>(&self, pattern: S) -> Result<TestRunResult> {
        self.run_tests()
            .match_tests(pattern.into())
            .build()
            .unwrap()
            .run()
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        // could have been never cerated. don't care about result
        // let _result = fs::remove_dir_all(&self.root_dir);
    }
}

fn prepare_temp_dir<S: Into<String>>(name: S) -> PathBuf {
    let mut root_dir = cargo_root();
    root_dir.push(E2E_TEMP_DIR);
    root_dir.push(name.into());
    // Remove anything from previous test run
    fs::remove_dir_all(&root_dir).ok();
    root_dir
}

fn cargo_root() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory"))
}
