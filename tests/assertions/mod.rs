mod equals_test;
mod err_matches_regex_test;
mod matches_regex_test;
mod matches_snapshot_test;

fn setup_test_env() {
    k9::assertions::set_panic(false);
    colored::control::set_override(true);
}
