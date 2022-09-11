use colored::*;
use anyhow::Result;
use diff::lines;
use std::fmt::Write;

pub fn colored_diff(left: &str, right: &str) -> Result<Option<String>> {
    let mut result = String::new();

    if left == right {
        return Ok(None);
    }

    let lines = lines(left, right);

    result.push('\n');

    for line in lines {
        match line {
            diff::Result::Left(l) => {
                writeln!(result, "{} {}", "-".red(), &l.red())?;
            }
            
            diff::Result::Right(r) => {
                writeln!(result, "{} {}", "+".green(), &r.green())?;
            }

            diff:Result::Both(l, _r) => {
                writeln!(result, "  {}", &l.dimmed())?;
            }
        }
    }

    Ok(Some(result))
}
