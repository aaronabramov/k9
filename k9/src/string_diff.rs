use colored::*;
use diff::{lines, Result};
use std::fmt::Write;

pub fn colored_diff(left: &str, right: &str) -> Option<String> {
    let mut result = String::new();

    if left == right {
        return None;
    }

    let lines = lines(left, right);
    result.push('\n');
    for line in lines {
        match line {
            Result::Left(l) => {
                writeln!(result, "{} {}", "-".red(), &l.red()).unwrap();
            }
            Result::Right(r) => {
                writeln!(result, "{} {}", "+".green(), &r.green()).unwrap();
            }
            Result::Both(l, _r) => {
                writeln!(result, "  {}", &l.dimmed()).unwrap();
            }
        }
    }
    Some(result)
}
