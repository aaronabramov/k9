use crate::support::TestProject;
use anyhow::Result;

#[test]
fn basic_fixture_project() -> Result<()> {
    let project = TestProject::fixture("basic_fixture", "./fixture_projects/basic");

    let test_run = project.run_matching_tests("basic")?;

    k9_released::assert_matches_inline_snapshot!(
        format!("\n{:?}\n", test_run.test_cases),
        r##"
{"snapshots_basic::snapshot_test": TestCaseResult { status: Pass }}
"##
    );

    let test_run = project.run_matching_tests("experimental")?;

    k9_released::assert_matches_inline_snapshot!(
        format!("\n{:?}\n", test_run.test_cases),
        r##"
{"snapshots_experimental::experimental_snapshot": TestCaseResult { status: Fail }}
"##
    );

    k9_released::assert_matches_inline_snapshot!(
        format!("\n{}\n", test_run.stdout_sanitized),
        r##"

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 1 test
test snapshots_experimental::experimental_snapshot ... FAILED

failures:

---- snapshots_experimental::experimental_snapshot stdout ----
thread 'snapshots_experimental::experimental_snapshot' panicked at '
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
snapshot!("Hello\nWorld");

Assertion Failure!

Expected string to match inline snapshot

but that assertion did not have any inline snapshots.

run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
', <REPLACED>/src/assertions.rs:34:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    snapshots_experimental::experimental_snapshot

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out


"##
    );

    let test_run = project
        .run_tests()
        .update_snapshots(true)
        .match_tests("experimental")
        .build()
        .unwrap()
        .run()?;

    test_run.assert_success()?;

    k9_released::assert_matches_inline_snapshot!(
        format!("\n{:?}\n", test_run.test_cases),
        r##"
{"snapshots_experimental::experimental_snapshot": TestCaseResult { status: Pass }}
"##
    );

    k9_released::assert_matches_inline_snapshot!(
        project
            .read_file("_tests/snapshots_experimental.rs")?
            .replace("##", "~~"),
        r##"use k9::*;

#[test]
fn experimental_snapshot() {
    _snapshot!(
        "Hello\nWorld",
        r~~"Hello
World"~~
    );
}
"##
    );

    Ok(())
}
