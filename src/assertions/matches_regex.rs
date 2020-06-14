use crate::assertion_error::AssertionError;
use crate::utils::add_linebreaks;
use crate::Result;
use colored::*;
use regex::Regex;

/// Asserts that passed `&str` matches a regular expression.
/// Regular expressions are compiled using `regex` crate.
///
/// ```rust
/// use k9::assert_matches_regex;
///
/// assert_matches_regex!("1234-45", "\\d{4}-\\d{2}");
/// assert_matches_regex!("abc", "abc");
/// ````
#[macro_export]
macro_rules! assert_matches_regex {
    ($s:expr, $regex:expr) => {{
        $crate::assert_matches_regex_r!($s, $regex).unwrap();
    }};
    ($s:expr, $regex:expr, $context:expr) => {{
        $crate::assert_matches_regex_r!($s, $regex, $context).unwrap();
    }};
}

/// Same as `assert_matches_regex` but returns an assertion `Result` instead
#[macro_export]
macro_rules! assert_matches_regex_r {
    ($s:expr, $regex:expr) => {{
        $crate::assertions::matches_regex::assert_matches_regex_impl($s, $regex, None)
    }};
    ($s:expr, $regex:expr, $context:expr) => {{
        $crate::assertions::matches_regex::assert_matches_regex_impl($s, $regex, Some(context))
    }};
}

pub fn assert_matches_regex_impl(s: &str, regex: &str, context: Option<&str>) -> Result<()> {
    let r = Regex::new(regex).unwrap();

    if !r.is_match(s) {
        let message = format!(
            "
{context}{assertion_desc}({string_desc}, {regex_desc});\n
Expected string:\n  {string}\nto match regex:\n  `{regex}`",
            context = context.map(add_linebreaks).unwrap_or_else(|| "".into()),
            assertion_desc = "assert_matches_regex!".dimmed(),
            string_desc = "string".red(),
            regex_desc = "regex".green(),
            string = s.red(),
            regex = regex.green(),
        );
        Err(AssertionError::new(message))
    } else {
        Ok(())
    }
}
