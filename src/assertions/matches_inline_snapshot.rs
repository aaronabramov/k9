use crate::types;
use colored::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref SOURCE_FILES: Mutex<HashMap<types::FilePath, SourceFile>> =
        Mutex::new(HashMap::new());
}

pub fn matches_inline_snapshot(
    s: String,
    snapshot: Option<&str>,
    line: u32,
    column: u32,
    file: &str,
) -> Option<String> {
    match (snapshot, crate::config::CONFIG.update_mode) {
        (Some(snapshot), false) => snapshot_matching_message(&s, snapshot),
        (None, false) => empty_snapshot_message(),
        (_, true) => {
            let this_file_path = crate::paths::get_absolute_path(file);

            let mode = if let Some(snapshot) = snapshot {
                let need_updating = snapshot_matching_message(&s, snapshot).is_some();

                if need_updating {
                    UpdateInlineSnapshotMode::Replace
                } else {
                    UpdateInlineSnapshotMode::NoOp
                }
            } else {
                UpdateInlineSnapshotMode::Create
            };

            update_inline_snapshot(this_file_path, line, column, &s, mode).unwrap();
            None
        }
    }
}

use proc_macro2::{TokenStream, TokenTree};
use syn::visit::Visit;
use syn::{Macro, PathSegment};

#[derive(Debug)]
struct MacroVisitor {
    found: Option<TokenStream>,
    line: usize,
}

impl<'ast> Visit<'ast> for MacroVisitor {
    fn visit_macro(&mut self, m: &'ast Macro) {
        let last_path_segment = m.path.segments.last();
        if let Some(PathSegment { ident, .. }) = last_path_segment {
            if ident.to_string().as_str() == "assert_matches_inline_snapshot"
                && ident.span().start().line == self.line
            {
                self.found.replace(m.tokens.to_owned());
            }
        }
    }
}

#[derive(Clone, Copy)]
enum UpdateInlineSnapshotMode {
    Create,  // when there's no inline snapshot
    Replace, // when there's an existing inline snapshot
    NoOp,    // no need to update anything, current snapshot is valid
}
// struct that represents a modification to the source code. E.g. we added/updated inline snapshot
#[derive(Debug)]
pub struct Patch {
    location: LineColumn,
    line_shift: i32,
}

#[derive(Debug)]
struct LineColumn {
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Range {
    start: LineColumn,
    end: LineColumn,
}

pub struct SourceFile {
    pub original_content: Option<String>, // initial version of the file where line!() and column!() refer to
    pub content: Option<String>,
    pub patches: Vec<Patch>,

    path: types::FilePath,
}

impl SourceFile {
    fn new(path: types::FilePath) -> Self {
        Self {
            path,
            content: None,
            original_content: None,
            patches: vec![],
        }
    }

    // read source file content and panic if the file on disk changed
    pub fn read_and_compare(&mut self) {
        let read_content = std::fs::read_to_string(&self.path).expect("can't read source file");

        if let Some(content) = &self.content {
            if &read_content != content {
                panic!("File content was modified during test run")
            }
        } else {
            self.content.replace(read_content.clone());
            self.original_content.replace(read_content);
        }
    }

    pub fn write(&self) {
        std::fs::write(&self.path, self.content.as_ref().unwrap()).unwrap();
    }
}

pub fn with_source_file<F, T>(absolute_path: &str, f: F) -> Result<T, String>
where
    F: FnOnce(&mut SourceFile) -> Result<T, String>,
{
    let mut map = SOURCE_FILES.lock().expect("poisoned lock");
    let mut source_file = map
        .entry(absolute_path.to_string())
        .or_insert_with(|| SourceFile::new(absolute_path.to_string()));

    let result = f(&mut source_file);
    drop(map);
    result
}

fn snapshot_matching_message(s: &str, snapshot: &str) -> Option<String> {
    let diff = crate::string_diff::colored_diff(&snapshot, &s);

    diff.map(|diff| {
        format!(
            "Expected {string_desc} to match {snapshot_desc}

    Difference:
    {diff}
    
    {update_instructions}
    ",
            string_desc = "string".red(),
            snapshot_desc = "inline snapshot".green(),
            diff = diff,
            update_instructions = crate::constants::update_instructions(),
        )
    })
}

fn empty_snapshot_message() -> Option<String> {
    Some(format!(
        "Expected {string_desc} to match {snapshot_desc}

but that assertion did not have any inline snapshots.

{update_instructions}
",
        string_desc = "string".red(),
        snapshot_desc = "inline snapshot".green(),
        update_instructions = crate::constants::update_instructions(),
    ))
}

fn update_inline_snapshot(
    file_path: PathBuf,
    original_line_num: u32,
    original_column_num: u32,
    to_add: &str,
    mode: UpdateInlineSnapshotMode,
) -> Result<(), String> {
    if let UpdateInlineSnapshotMode::NoOp = mode {
        // no need to update anything, snapshot is up to date
        return Ok(());
    }

    with_source_file(&file_path.display().to_string(), |file| {
        file.read_and_compare();
        let content = file.content.take().expect("empty file content. read first");

        let mut new_line_num = original_line_num as i32;
        // for now we assume that columns don't change (unless there are two snapshots on the same line)
        let new_column_num = original_column_num;

        for patch in &file.patches {
            if patch.location.line <= original_line_num as usize {
                new_line_num += patch.line_shift;
            }
        }

        let range_to_replace =
            find_inline_snapshot_range(&content, new_line_num as u32, new_column_num, mode)
                .unwrap();

        let (before, after, lines_removed) = split_by_range(content, &range_to_replace).unwrap();

        let comma_separator = match mode {
            UpdateInlineSnapshotMode::Create => ", ",
            UpdateInlineSnapshotMode::Replace => "",
            UpdateInlineSnapshotMode::NoOp => panic!("unreachable"),
        };

        let replace_with = format!(
            "{comma_separator}\"{to_add}\"",
            comma_separator = comma_separator,
            to_add = escape_snapshot_string_literal(to_add),
        );

        let new_lines = replace_with.lines().count() - 1; // we had an existing line

        file.patches.push(Patch {
            location: LineColumn {
                line: original_line_num as usize,
                column: original_column_num as usize,
            },
            line_shift: new_lines as i32 - lines_removed as i32,
        });

        let new_content = format!(
            "{before}{replace_with}{after}",
            before = before,
            replace_with = replace_with,
            after = after
        );

        file.content.replace(new_content);
        file.write();

        Ok(())
    })
}

fn find_inline_snapshot_range(
    file_content: &str,
    line_num: u32,
    _column_num: u32,
    mode: UpdateInlineSnapshotMode,
) -> Result<Range, String> {
    let syntax = syn::parse_file(file_content).expect("Unable to parse file");

    let mut macro_visitor = MacroVisitor {
        found: None,
        line: line_num as usize,
    };

    macro_visitor.visit_file(&syntax);

    let tt = macro_visitor
        .found
        .expect("didnt find macro literal in ast");

    match mode {
        UpdateInlineSnapshotMode::Replace => {
            let literal = tt.into_iter().last();

            if let Some(TokenTree::Literal(literal)) = literal {
                Ok(Range {
                    start: LineColumn {
                        line: literal.span().start().line,
                        column: literal.span().start().column,
                    },
                    end: LineColumn {
                        line: literal.span().end().line,
                        column: literal.span().end().column + 1, // i can't explain what this 1 char is
                    },
                })
            } else {
                panic!("not a literal at the end of the macro? {:?}", literal)
            }
        }
        UpdateInlineSnapshotMode::Create => {
            let last = tt.into_iter().last().expect("must have last tokentree");
            let span = last.span();

            Ok(Range {
                start: LineColumn {
                    line: span.end().line,
                    column: span.end().column,
                },
                end: LineColumn {
                    line: span.end().line,
                    column: span.end().column,
                },
            })
        }
        _ => panic!("unreachable"),
    }
}

fn escape_snapshot_string_literal(snapshot_string: &str) -> String {
    let mut result = String::with_capacity(snapshot_string.len());

    // there must be a more performant way to do it, but generally snapshot should be pretty light
    for c in snapshot_string.chars() {
        match c {
            '"' => result.push_str(r#"\""#),
            _ => result.push(c),
        }
    }

    result
}

fn split_by_range(content: String, range: &Range) -> Result<(String, String, usize), String> {
    let mut before = String::new();
    let mut after = String::new();
    let mut lines_removed = 0;

    for (i, line) in content.lines().enumerate() {
        let line_number = i + 1;

        match line_number {
            n if n < range.start.line => {
                before.push_str(&line);
                before.push_str("\n");
            }
            n if n == range.start.line => {
                let chars = line.chars().collect::<Vec<_>>();

                for (i, char) in chars.into_iter().enumerate() {
                    let column = i + 1;
                    if column <= range.start.column {
                        before.push(char)
                    }

                    // if it's the same line, it won't match latter pattern
                    if n == range.end.line && column > range.end.column {
                        after.push(char)
                    }
                }
                if n == range.end.line {
                    after.push_str("\n");
                }
            }
            n if n > range.start.line && n < range.end.line => {
                lines_removed += 1;
            }
            n if n == range.end.line => {
                let chars = line.chars().collect::<Vec<_>>();

                for (i, char) in chars.into_iter().enumerate() {
                    let column = i + 1;

                    if column >= range.end.column {
                        after.push(char)
                    }
                }
                after.push_str("\n");
            }
            n if n > range.end.line => {
                after.push_str(&line);
                after.push_str("\n");
            }
            _ => panic!("invalid range or file. Range: {:?}", range),
        };
    }

    Ok((before, after, lines_removed))
}
