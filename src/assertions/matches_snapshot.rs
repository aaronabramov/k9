use crate::string_diff::colored_diff;
use colored::*;
use std::path::{Path, PathBuf};

const SNAPSHOT_DIR: &str = "__k9_snapshots__";

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

pub fn snap_internal<T: std::fmt::Display>(
    thing: T,
    _line: u32,
    _column: u32,
    file: &str,
) -> Option<String> {
    let thing_str = thing.to_string();

    let snapshot_dir = get_snapshot_dir(file);
    let test_name = get_test_name();
    let relative_snap_path = get_test_snap_path(&snapshot_dir, &test_name)
        .display()
        .to_string();

    let absolute_snap_path = crate::paths::get_absolute_path(&relative_snap_path).unwrap();

    let string_desc = "string".red();
    let snapshot_desc = "snapshot".green();

    if crate::config::CONFIG.update_mode {
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
                file = relative_snap_path.green(),
                diff = diff,
                update_instructions = crate::constants::update_instructions(),
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
            file = relative_snap_path.green(),
            update_instructions = crate::constants::update_instructions(),
        ))
    }
}
