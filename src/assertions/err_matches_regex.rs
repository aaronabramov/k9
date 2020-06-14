use colored::*;
use regex::Regex;

pub fn assert_err_matches_regex<A, T: std::fmt::Debug>(
    result: Result<A, T>,
    regex: &str,
) -> Option<String> {
    let r = Regex::new(regex).unwrap();

    if let Err(err) = result {
        let s = format!("{:?}", err);
        if !r.is_match(&s) {
            let message = format!(
                "
Expected `{result_desc}` to be an `{err_desc}` that matches {regex}:
{error}\n\n",
                result_desc = "Result<T, E>".red(),
                err_desc = "Err(std::fmt::Debug)".red(),
                regex = regex.green(),
                error = s.red(),
            );
            Some(message)
        } else {
            None
        }
    } else {
        Some(format!(
            "
Expected `{result_desc}` to be an `{err_desc}` that matches {regex}
but it was `{ok_desc}`\n\n",
            err_desc = "Err(std::fmt::Debug)".red(),
            regex = regex.green(),
            result_desc = "Result<T, E>".red(),
            ok_desc = "Ok(T)".green(),
        ))
    }
}
