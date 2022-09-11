use regex::Regex;
use anyhow::{Context, Result};
use colored::*;

pub fn assert_err_matches_regex<A, T: std::fmt::Debug>(
    result: Result<A, T>,
    regex: &str,
) -> Result<Option<String>> {
    let r = Regex::new(regex).context("Failed to compile RegularExpression using regex::RegEx")?;
    let result_desc = "Result<T, E>".red();
    let err_desc = "Err(E)".red();
    let format_desc = "format!(\"{:?}\", error)".yellow();
    let regex_desc = "regex".green();

    if let Err(err) = result {
        let s = format!("{:?}", err);
        if !r.is_match(&s) {
            let message = format!(
                "Expected {result_desc} to be {err_desc} that matches 
{regex_desc} when formatted with `{format_desc}`, 

Regex: {regex}
Formatted error: {error}
",
                result_desc = result_desc,
                err_desc = err_desc,
                format_desc = format_desc,
                regex_desc = regex_desc,
                regex = regex.green(),
                error = s.red(),
            );
            Some(message)
        } else {
            None
        }
    } else {
        Some(format!(
            "Expected {result_desc} to be {err_desc} that matches {regex_desc} when 
formatted with `{format_desc}`, but it was {ok_desc}

Regex: {regex}
",
            err_desc = err_desc,
            regex = regex.green(),
            result_desc = result_desc,
            ok_desc = "Ok(T)".green(),
            regex_desc = regex_desc,
            format_desc = format_desc,
        ))
    }
}
