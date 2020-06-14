use crate::string_diff::colored_diff;
use crate::utils::add_linebreaks;
use crate::AssertionError;
use colored::*;
use std::path::{Path, PathBuf};

const SNAPSHOT_DIR: &str = "__k9_snapshots__";
const UPDATE_ENV_VAR: &str = "K9_UPDATE_SNAPSHOTS";

/// Formats passed value and asserts that it matches existing snaphot.
/// If snapshot file for this test does not exist, test can be run with `K9_UPDATE_SNAPSHOTS=1`
/// environment variable to either create or replace existing snapshot file.
/// Snapshots will be written into `__k9_snapshots__` directory next to the test file.
///
/// ```rust
/// #[test]
/// fn my_test() {
///     struct A {
///         name: &'a str,
///         age: u32
///     }
///
///     let a = A { name: "Lance", age: 9 };
///
///     // When first run with `K9_UPDATE_SNAPSHOTS=1` it will
///     // create `__k9_snapshots__/my_test_file/my_test.snap` file
///     // with contents being the serialized value of `a`.
///     // Next time the test is run, if the newly serialized value of a
///     // is different from the value of that snapshot file, the assertion
///     // will fail.
///     assert_matches_snapshot!(a);
/// }
/// ```
#[macro_export]
macro_rules! assert_matches_snapshot {
    ($thing:expr) => {{
        let line = line!();
        let column = column!();
        let file = file!();

        $crate::assertions::matches_snapshot::snap_internal($thing, None, line, column, file).unwrap();
    }};
    ($thing:expr, $regex:expr, $context:expr) => {{
        let line = line!();
        let column = column!();
        let file = file!();

        $crate::assertions::matches_snapshot::snap_internal($thing, Some(context) line, column, file).unwrap();
    }};
}

/// Same as assert_matches_snapshot! but returns an assertion Result instead
/// ```rust
/// #[test]
/// fn my_test() {
/// let result: Result<(), &str> = Err("http request fail. code 123");
/// assert_matches_snapshot_r!(result);
/// }
/// ```
#[macro_export]
macro_rules! assert_matches_snapshot_r {
    ($thing:expr) => {{
        let line = line!();
        let column = column!();
        let file = file!();

        $crate::assertions::matches_snapshot::snap_internal($thing, None, line, column, file);
    }};
    ($thing:expr, $regex:expr, $context:expr) => {{
        let line = line!();
        let column = column!();
        let file = file!();

        $crate::assertions::matches_snapshot::snap_internal($thing, Some(context) line, column, file);
    }};
}

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

pub fn get_source_file_path(file: &str) -> PathBuf {
    let mut p = get_project_root_path();
    p.push(file);
    p
}

pub fn get_snapshot_dir(source_file: &Path) -> PathBuf {
    let mut c = source_file.components();
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
        if option_env!("K9_UPDATE_SNAPSHOTS").is_some() {
            return true;
        }
    }

    runtime_var
}

pub fn snap_internal<T: std::fmt::Display>(
    thing: T,
    context: Option<&str>,
    _line: u32,
    _column: u32,
    file: &str,
) -> crate::Result<()> {
    let thing_str = thing.to_string();
    let assertion_desc = format!(
        "{}({}{});\n",
        "assert_matches_snapshot!".dimmed(),
        "thing".red(),
        context
            .as_ref()
            .map(|_| format!(", {}", "context".red()))
            .unwrap_or_else(|| "".into()),
    );

    let this_file_path = get_source_file_path(file);
    let snapshot_dir = get_snapshot_dir(&this_file_path);
    let test_name = get_test_name();
    let snap_path = get_test_snap_path(&snapshot_dir, &test_name);

    if is_update_mode() {
        ensure_snap_dir_exists(&snap_path);
        std::fs::write(&snap_path, thing_str).unwrap();
        Ok(())
    } else {
        let exists = snap_path.exists();

        let snap_content = if exists {
            std::fs::read_to_string(&snap_path).expect("can't read snapshot file")
        } else {
            String::new()
        };

        let diff = colored_diff(&snap_content, &thing_str);
        if let Some(diff) = diff {
            let message = format!(
                "
{context}{assertion_desc}
Expected `{thing_desc}` to match `{snapshot_desc}`:
{diff}

{update_instructions}
",
                context = context.map(add_linebreaks).unwrap_or_else(|| "".into()),
                assertion_desc = &assertion_desc,
                thing_desc = "thing".red(),
                snapshot_desc = "snapshot".green(),
                diff = diff,
                update_instructions =
                    "run with `K9_UPDATE_SNAPSHOTS=1` to update snapshots".yellow(),
            );
            return Err(AssertionError::new(message));
        } else if exists {
            Ok(())
        } else {
            return Err(AssertionError::new("Snapshot file doesn't exist".into()));
        }
    }
}
