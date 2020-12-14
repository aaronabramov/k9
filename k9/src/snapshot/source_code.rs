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

/// Given a source code file content and a Vec<Range>, split content
/// into chunks while also removing the content within provided ranges, so that
/// it can later be replaced with something else.
pub fn split_by_ranges(content: String, ranges: Vec<&Range>) -> Vec<String> {
    let mut iter = ranges.iter().peekable();

    // ranges must be pre-sorted
    while let Some(range) = iter.next() {
        if let Some(next_range) = iter.peek() {
            if range.end.line >= next_range.start.line {
                panic!("overlapping ranges! can be only one inline snapshot macro per line");
            }
        }
    }

    let mut ranges_iter = ranges.into_iter();
    let mut chunks = vec![];
    let mut next_chunk = String::new();
    let mut next_range = ranges_iter.next();

    for (i, line) in content.lines().enumerate() {
        let line_number = i + 1;

        if let Some(range) = next_range {
            match line_number {
                n if n < range.start.line => {
                    next_chunk.push_str(&line);
                    next_chunk.push('\n');
                }
                n if n == range.start.line => {
                    let chars = line.chars().collect::<Vec<_>>();

                    let mut chars_before = chars;
                    let mut rest = chars_before.split_off(range.start.column - 1);
                    let str_before: String = chars_before.iter().collect();
                    next_chunk.push_str(&str_before);

                    // The range is in a single line
                    if n == range.end.line {
                        let chars_after = rest.split_off(range.end.column - 1 - chars_before.len());
                        let str_after: String = chars_after.iter().collect();

                        chunks.push(next_chunk);
                        next_chunk = String::new();
                        next_range = ranges_iter.next();

                        next_chunk.push_str(&str_after);
                        next_chunk.push('\n');
                    }
                }
                n if n > range.start.line && n < range.end.line => {}
                n if n == range.end.line => {
                    chunks.push(next_chunk);
                    next_chunk = String::new();
                    next_range = ranges_iter.next();

                    let mut chars = line.chars().collect::<Vec<_>>();
                    let after_chars = chars.split_off(range.end.column - 1);
                    let after_str: String = after_chars.iter().collect();
                    next_chunk.push_str(&after_str);
                    next_chunk.push('\n');
                }
                _ => panic!(
                    "invalid range or file. Line: `{}` Range: {:?}",
                    line_number, range
                ),
            };
        } else {
            next_chunk.push_str(&line);
            next_chunk.push('\n');
        }
    }
    chunks.push(next_chunk);
    chunks
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
