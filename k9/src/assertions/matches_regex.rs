use regex::Regex;
use anyhow::Result;
use colored::*;

pub fn assert_matches_regex(s: &str, regex: &str) -> Result<Option<String>> {
    let r = Regex::new(regex).context("Failed to compile RegExp using regex::RegEx");

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
