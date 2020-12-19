use crate::support::TestProject;
use anyhow::Result;

#[test]
fn basic_fixture_project() -> Result<()> {
    let project = TestProject::fixture("basic_fixture", "./fixture_projects/basic");

    let test_run = project.run_matching_tests("basic")?;

    k9_released::snapshot!(
        test_run.test_cases,
        r#"
{
    "snapshots_basic::snapshot_test": TestCaseResult {
        status: Pass,
    },
}
"#
    );

    let test_run = project.run_matching_tests("experimental")?;

    k9_released::snapshot!(
        test_run.test_cases,
        r#"
{
    "snapshots_experimental::experimental_snapshot": TestCaseResult {
        status: Fail,
    },
}
"#
    );

    k9_released::snapshot!(
        test_run.stdout_sanitized,
        "

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 1 test
test snapshots_experimental::experimental_snapshot ... FAILED

failures:

---- snapshots_experimental::experimental_snapshot stdout ----
thread \'snapshots_experimental::experimental_snapshot\' panicked at \'
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
snapshot!(map);

Assertion Failure!

Expected string to match inline snapshot

but that assertion did not have any inline snapshots.

run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
\', <REPLACED>/src/assertions.rs:33:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    snapshots_experimental::experimental_snapshot

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out


"
    );

    let test_run = project
        .run_tests()
        .update_snapshots(true)
        .match_tests("experimental")
        .build()
        .unwrap()
        .run()?;

    test_run.assert_success()?;

    k9_released::snapshot!(
        test_run.test_cases,
        r#"
{
    "snapshots_experimental::experimental_snapshot": TestCaseResult {
        status: Pass,
    },
}
"#
    );

    k9_released::snapshot!(
        project
            .read_file("_tests/snapshots_experimental.rs")?
            .replace("#", "~"),
        r#"
use k9::*;
use std::collections::BTreeSet;

~[test]
fn experimental_snapshot() {
    snapshot!(
        "Hello\
World",
        "
Hello
World
"
    );

    let map: BTreeSet<u8> = vec![1, 2, 3, 0, 5, 8].into_iter().collect();

    snapshot!(
        map,
        "
{
    0,
    1,
    2,
    3,
    5,
    8,
}
"
    );

    snapshot!(
        "uses single quotes for literal",
        "uses single quotes for literal"
    );

    snapshot!(
        r~"should"not"use"quotes"in"literal"~,
        r~"should"not"use"quotes"in"literal"~
    );

    snapshot!(
        r~~~~"should use more than two "~~ for escaping"~~~~,
        r~~~"should use more than two "~~ for escaping"~~~
    );
}

"#
    );

    Ok(())
}
