#[derive(Debug, PartialEq, Eq)]
// Struct that represents a range of text in a source code file content
pub struct Range {
    pub start: LineColumn,
    pub end: LineColumn,
}

/// Lines and Columns start form 1
///
/// 12345678910   15   20   25...
/// hello_world
///
/// {line: 1, column: 1} -> {line: 1, column: 6} => "hello"
/// {line: 1, column: 2} -> {line: 1, column: 6} => "ello"
/// {line: 1, column: 1} -> {line: 1, column: 1} => ""
#[derive(Debug, PartialEq, Eq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

/// Given a source file, extract a substring from it at the given range
pub fn extract_range(s: &str, range: &Range) -> String {
    let mut result = String::new();
    for (i, line) in s.lines().enumerate() {
        let line_number = i + 1;

        if line_number >= range.start.line && line_number <= range.end.line {
            if !result.is_empty() {
                result.push('\n');
            }

            for (j, char) in line.chars().enumerate() {
                let column = j + 1;
                if !((line_number == range.start.line && column < range.start.column)
                    || (line_number == range.end.line && column >= range.end.column))
                {
                    result.push(char)
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r##"
Hello World
Random Subset
1234567
"##;

    #[test]
    fn extracting_range() {
        k9_stable::assert_equal!(
            extract_range(
                CONTENT,
                &Range {
                    start: LineColumn { line: 2, column: 1 },
                    end: LineColumn { line: 2, column: 6 }
                }
            )
            .as_str(),
            "Hello"
        );
    }

    #[test]
    fn empty_range() {
        k9_stable::assert_equal!(
            extract_range(
                CONTENT,
                &Range {
                    start: LineColumn { line: 2, column: 1 },
                    end: LineColumn { line: 2, column: 1 }
                }
            )
            .as_str(),
            ""
        );
    }

    #[test]
    fn overflow() {
        k9_stable::assert_equal!(
            extract_range(
                CONTENT,
                &Range {
                    start: LineColumn {
                        line: 99,
                        column: 1
                    },
                    end: LineColumn {
                        line: 199,
                        column: 1
                    }
                }
            )
            .as_str(),
            ""
        );
    }
}
