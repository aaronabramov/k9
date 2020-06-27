mod assertions;
mod e2e;

// a macro-hack to trick test env by giving the macro with the same name that will
// actually panic in test environment
#[macro_export]
macro_rules! assert_matches_inline_snapshot {
    ( $( $arg:expr ),* ) => {{
        k9::assert_matches_inline_snapshot!( $( $arg ),* ).map(|a| a.panic())
    }};
}

pub fn strip_ansi(s: &str) -> String {
    String::from_utf8(
        strip_ansi_escapes::strip(s).expect("Cant strip ANSI escape characters from a string"),
    )
    .expect("not a utf8 string")
}

pub fn assertion_message(m: Option<k9::assertions::Assertion>) -> String {
    strip_ansi(&m.expect("Assertion must fail").get_failure_message())
}
