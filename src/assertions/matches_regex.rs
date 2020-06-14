use colored::*;
use regex::Regex;

pub fn assert_matches_regex(s: &str, regex: &str) -> Option<String> {
    let r = Regex::new(regex).unwrap();

    if !r.is_match(s) {
        let message = format!(
            "
Expected string:\n  {string}\nto match regex:\n  `{regex}`",
            string = s.red(),
            regex = regex.green(),
        );
        Some(message)
    } else {
        None
    }
}
