mod equals_test;
mod err_matches_regex_test;
mod matches_regex_test;
mod matches_snapshot_test;

fn setup_test_env() {
    k9::config::set_panic(false);
    k9::config::set_terminal_with_override(100);
    colored::control::set_override(true);
}
