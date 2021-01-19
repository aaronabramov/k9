use crate::support::TestProject;
use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::Mutex;

use std::collections::BTreeMap;
pub fn my_function() -> BTreeMap<usize, String> {
    (3..=8)
        .map(|i| (i, "a".repeat(i).to_string()))
        .collect::<BTreeMap<_, _>>()
}

lazy_static! {
    // Can't run tests in the same directory in paraller, so we'll put it behind a mutex
    // to restrict test runs to 1 at a time only.
    static ref PROJECT: Mutex<TestProject> =
    Mutex::new(TestProject::fixture("failure_messages", "./fixture_projects/failure_messages"));
}

fn get_error_message(test_name: &str) -> Result<String> {
    let project = PROJECT.lock().expect("poisoned lock");
    let test_run = project.run_matching_tests(test_name)?;
    anyhow::ensure!(
        test_run.test_cases.len() == 1,
        "Can only run a single test here. Make sure you're providing a stirng that matches exactly one test name. 
        ===========================================================================
        Test cases: \n {:?}\n
        STDOUT: \n{}\n
        STDERR:\n{}\n
        ===========================================================================
        ",
        &test_run.test_cases,
        test_run.stdout,
        test_run.stderr,

    );
    anyhow::ensure!(
        !test_run.success,
        "Expecting test run to fail, but it passed?"
    );

    Ok(test_run.stdout_sanitized)
}

#[test]
fn assert_equals_basic() -> Result<()> {
    k9_released::snapshot!(
        get_error_message("assert_equal_basic")?,
        r"

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 1 test
test assert_equal_basic ... FAILED

failures:

---- assert_equal_basic stdout ----
thread 'assert_equal_basic' panicked at '
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(1, 2);

Assertion Failure!


Expected `Left` to equal `Right`:

- 1
+ 2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
', _tests/mod.rs:5:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    assert_equal_basic

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out


"
    );
    Ok(())
}

#[test]
fn assert_equal_multiline_string() -> Result<()> {
    k9_released::snapshot!(
        get_error_message("assert_equal_multiline_string")?,
        r#"

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 1 test
test assert_equal_multiline_string ... FAILED

failures:

---- assert_equal_multiline_string stdout ----
thread 'assert_equal_multiline_string' panicked at '
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!("hello\
world", "hello\
how are you?");

Assertion Failure!


Expected `Left` to equal `Right`:

- "hello\
world"
+ "hello\
how are you?"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
', _tests/mod.rs:10:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    assert_equal_multiline_string

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out


"#
    );
    Ok(())
}

#[test]
fn snapshot_basic() -> Result<()> {
    k9_released::snapshot!(
        get_error_message("snapshot_basic")?,
        r#"

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 1 test
test snapshot_basic ... FAILED

failures:

---- snapshot_basic stdout ----
thread 'snapshot_basic' panicked at '
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
snapshot!("a");

Assertion Failure!

Expected string to match inline snapshot

but that assertion did not have any inline snapshots.

run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
', _tests/mod.rs:15:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    snapshot_basic

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out


"#
    );
    Ok(())
}
