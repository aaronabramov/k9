use colored::*;
use diff::{lines, Result};

pub fn colored_diff(left: &str, right: &str) -> Option<String> {
    let mut result = String::new();

    if left == right {
        return None;
    }

    let lines = lines(left, right);
    if lines.len() > 20 && !crate::config::CONFIG.expand_mode {
        result.push_str(&collapsed_diff(lines));
    } else {
        result.push_str(&uncollapsed_diff(lines));
    }
    Some(result)
}

fn uncollapsed_diff(lines: Vec<diff::Result<&str>>) -> String {
    let mut result = String::new();
    result.push_str("\n");
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
    result
}

fn collapsed_diff(lines: Vec<diff::Result<&str>>) -> String {
    let mut result = String::new();
    let mut less_significant_lines = String::new();
    result.push_str("\n");
    let mut collapsed_line_count = 0;
    let mut padding: bool = true;
    let mut padding_count = 0;
    for line in lines {
        match line {
            Result::Left(l) => {
                result.push_str(&less_significant_lines);
                result.push_str(&format!("{} {}\n", "-".red(), &l.red()));

                less_significant_lines = String::new();
                collapsed_line_count = 0;
            }
            Result::Right(r) => {
                result.push_str(&format!("{} {}\n", "+".green(), &r.green()));
                collapsed_line_count = 0;
                padding = true;
                padding_count = 0;
            }
            Result::Both(l, _r) => {
                if padding && padding_count < 2 {
                    result.push_str(&format!("  {}\n", &l.dimmed()));
                    padding_count += 1;
                } else {
                    padding = false;
                    padding_count = 0;
                    if count_newlines(&less_significant_lines) < 2 {
                        less_significant_lines.push_str(&format!("  {}\n", &l.dimmed()));
                    } else {
                        collapsed_line_count += count_newlines(&less_significant_lines);
                        if collapsed_line_count > 2 {
                            // Pop previously collapsed string with newline
                            result.truncate(result.len() - 45);
                        }
                        result.push_str(
                            &format!(
                                "{}   {} {}\n",
                                "...".dimmed(),
                                (collapsed_line_count).to_string().dimmed(),
                                "lines collapsed".dimmed()
                            )
                            .dimmed(),
                        );
                        less_significant_lines = String::new();
                        less_significant_lines.push_str(&format!("  {}\n", &l.dimmed()));
                    }
                }
            }
        }
    }
    result.push_str(&less_significant_lines);
    result
}

fn count_newlines(s: &str) -> usize {
    s.matches('\n').count()
}
