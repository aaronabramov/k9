use colored::*;
use regex::Regex;

pub fn assert_matches_regex(s: &str, regex: &str) -> Option<String> {
    let r = Regex::new(regex).unwrap();

    if !r.is_match(s) {
        let message = format!(
            "Expected {string_desc} to match {regex_desc}

Regex: {regex}
Received string: {string}
",
            string_desc = "string".red(),
            regex_desc = "regex".green(),
            string = s.red(),
            regex = regex.green(),
        );
        Some(message)
    } else {
        None
    }
}
