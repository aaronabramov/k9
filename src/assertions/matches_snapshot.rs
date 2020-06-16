use crate::string_diff::colored_diff;
use colored::*;
use std::path::{Path, PathBuf};

const SNAPSHOT_DIR: &str = "__k9_snapshots__";

fn get_project_root_path() -> PathBuf {
    let buck_build_id_present = std::env::var("BUCK_BUILD_ID").is_ok();

    // If there's a buck build id we'll grab the `pwd`, because we're probably running `buck test` from the root
    if buck_build_id_present {
        let pwd = std::env::var("PWD").expect(
            "
`BUCK_BUILD_ID` enviroment variable was present, which means this project is being built with
buck and relies on `PWD` env variable to contain the project root, but `PWD` wasn't there",
        );
        return PathBuf::from(pwd);
    }

    // otherwise ask cargo for project root
    let project_root =
        std::env::var("CARGO_MANIFEST_DIR").expect("Can't get project root directory");
    PathBuf::from(project_root)
}

pub fn get_snapshot_dir(source_file: &str) -> PathBuf {
    let mut c = Path::new(source_file).components();
    let source_file_name = c.next_back().unwrap().as_os_str().to_string_lossy();
    let mut p: PathBuf = c.collect();
    p.push(SNAPSHOT_DIR);
    p.push(source_file_name.replace(".rs", ""));
    p
}

pub fn get_test_name() -> String {
    let t = std::thread::current();
    t.name()
        .expect("Can't extract the test name")
        .to_string()
        .replace("::", "_")
}

pub fn get_test_snap_path(snapshot_dir: &Path, test_name: &str) -> PathBuf {
    let mut p = snapshot_dir.to_owned();
    p.push(format!("{}.snap", test_name));
    p
}

pub fn ensure_snap_dir_exists(snapshot_path: &Path) {
    std::fs::create_dir_all(snapshot_path.parent().unwrap()).unwrap();
}

fn is_update_mode() -> bool {
    // If runtime ENV variable is set, it takes precedence
    let runtime_var = std::env::var("K9_UPDATE_SNAPSHOTS").map_or(false, |_| true);

    if !runtime_var {
        // If not, we'll also check compile time variable. This is going to be the case with `buck`
        // when env variables are passed to `rustc` but not to the actual binary (when running `buck test ...`)
        //
        // NOTE: using compile time vars is a bit sketchy, because technically you can compile the test suite
        // once and re-run the compiled version multiple times in scenarious where you don't want to update
        if option_env!("K9_UPDATE_SNAPSHOTS").is_some() {
            return true;
        }
    }

    runtime_var
}

pub fn snap_internal<T: std::fmt::Display>(
    thing: T,
    _line: u32,
    _column: u32,
    file: &str,
) -> Option<String> {
    let thing_str = thing.to_string();

    let snapshot_dir = get_snapshot_dir(file);
    let test_name = get_test_name();
    let relative_snap_path = get_test_snap_path(&snapshot_dir, &test_name);

    let mut absolute_snap_path = get_project_root_path();
    absolute_snap_path.push(&relative_snap_path);

    let string_desc = "string".red();
    let snapshot_desc = "snapshot".green();
    let update_instructions =
        "run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots".yellow();

    if is_update_mode() {
        ensure_snap_dir_exists(&absolute_snap_path);
        std::fs::write(&absolute_snap_path, thing_str).unwrap();
        None
    } else if absolute_snap_path.exists() {
        let snapshot_content = std::fs::read_to_string(&absolute_snap_path.display().to_string())
            .expect("can't read snapshot file");
        let diff = colored_diff(&snapshot_content, &thing_str);

        diff.map(|diff| {
            format!(
                "Expected {string_desc} to match {snapshot_desc} stored in
{file}

Difference:
{diff}

{update_instructions}
",
                string_desc = string_desc,
                snapshot_desc = snapshot_desc,
                file = relative_snap_path.display().to_string().green(),
                diff = diff,
                update_instructions = update_instructions,
            )
        })
    } else {
        Some(format!(
            "Expected {string_desc} to match {snapshot_desc} stored in
{file}

but that snapshot file does not exist.

{update_instructions}
",
            string_desc = string_desc,
            snapshot_desc = snapshot_desc,
            file = relative_snap_path.display().to_string().green(),
            update_instructions = update_instructions,
        ))
    }
}
