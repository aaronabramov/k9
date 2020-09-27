use colored::*;
use diff::{lines, Result};

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
                result.push_str(&format!("{} {}\n", "-".red(), &l.red()));
            }
            Result::Right(r) => {
                result.push_str(&format!("{} {}\n", "+".green(), &r.green()));
            }
            Result::Both(l, _r) => {
                result.push_str(&format!("  {}\n", &l.dimmed()));
            }
        }
    }
    Some(result)
}
