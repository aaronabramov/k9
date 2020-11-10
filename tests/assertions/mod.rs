mod custom_comparison_formatters_test;
mod equals_test;

#[cfg(feature = "regex")]
mod err_matches_regex_test;
#[cfg(feature = "regex")]
mod matches_regex_test;

mod err_test;
mod greater_than_or_equal_test;
mod greater_than_test;
mod lesser_than_or_equal_test;
mod lesser_than_test;
mod matches_inline_snapshot_test;
mod matches_snapshot_test;
mod ok_test;

fn setup_test_env() {
    k9::config::set_panic(false);
    k9::config::set_terminal_with_override(100);
    colored::control::set_override(true);
}
