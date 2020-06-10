use crate::assertion_error::AssertionError;
use crate::utils::add_linebreaks;
use crate::Result as K9Result;
use colored::*;
use regex::Regex;

/// Asserts that the passed `Result` argument is an `Err` and
/// and the debug string of that error matches provided regex.
/// Regular expressions are compiled using `regex` crate.
///
/// ```rust
/// use k9::assert_err_matches_regex;
///
/// let result: Result<(), &str> = Err("http request failed");
/// assert_err_matches_regex!(result, "http");
/// ```
#[macro_export]
macro_rules! assert_err_matches_regex {
    ($s:expr, $regex:expr) => {{
        $crate::assert_err_matches_regex_r!($s, $regex).unwrap();
    }};
    ($s:expr, $regex:expr, $context:expr) => {{
        $crate::assert_err_matches_regex_r!($s, $regex, $context).unwrap();
    }};
}

/// Same as assert_err_matches_regex! but returns an assertion Result instead
#[macro_export]
macro_rules! assert_err_matches_regex_r {
    ($s:expr, $regex:expr) => {{
        $crate::assertions::err_matches_regex::assert_err_matches_regex_impl($s, $regex, None)
    }};
    ($s:expr, $regex:expr, $context:expr) => {{
        $crate::assertions::err_matches_regex::assert_err_matches_regex_impl(
            $s,
            $regex,
            Some(context),
        )
    }};
}

pub fn assert_err_matches_regex_impl<A, T: std::fmt::Debug>(
    result: Result<A, T>,
    regex: &str,
    context: Option<&str>,
) -> K9Result<()> {
    let r = Regex::new(regex).unwrap();
    let assertion_desc = format!(
        "{}({}, {});\n",
        "assert_err_matches_regex!".dimmed(),
        "Result<T, E>".red(),
        "regex".green()
    );

    if let Err(err) = result {
        let s = format!("{:?}", err);
        if !r.is_match(&s) {
            let message = format!(
                "
{context}{assertion_desc}
Expected `{result_desc}` to be an `{err_desc}` that matches {regex}:
{error}\n\n",
                context = context.map(add_linebreaks).unwrap_or("".into()),
                assertion_desc = &assertion_desc,
                result_desc = "Result<T, E>".red(),
                err_desc = "Err(std::fmt::Debug)".red(),
                regex = regex.green(),
                error = s.red(),
            );
            Err(AssertionError::new(message))
        } else {
            Ok(())
        }
    } else {
        let message = format!(
            "
{context}{assertion_desc}
Expected `{result_desc}` to be an `{err_desc}` that matches {regex}
but it was `{ok_desc}`\n\n",
            context = context.map(add_linebreaks).unwrap_or("".into()),
            err_desc = "Err(std::fmt::Debug)".red(),
            regex = regex.green(),
            assertion_desc = &assertion_desc,
            result_desc = "Result<T, E>".red(),
            ok_desc = "Ok(T)".green(),
        );
        return Err(AssertionError::new(message));
    }
}
