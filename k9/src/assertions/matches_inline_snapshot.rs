use crate::types;
use anyhow::{Context, Result};
use colored::*;
use lazy_static::lazy_static;
use proc_macro2::{TokenStream, TokenTree};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use syn::visit::Visit;
use syn::{Macro, PathSegment};

lazy_static! {
    static ref SOURCE_FILES: Mutex<Option<HashMap<types::FilePath, SourceFile>>> =
        Mutex::new(Some(HashMap::new()));
    static ref ATEXIT_HOOK_REGISTERED: AtomicBool = AtomicBool::new(false);
}

pub fn matches_inline_snapshot(
    s: String,
    snapshot: Option<&str>,
    line: u32,
    _column: u32,
    file: &str,
) -> Option<String> {
    match (snapshot, crate::config::CONFIG.update_mode) {
        (Some(snapshot), false) => snapshot_matching_message(&s, snapshot),
        (None, false) => Some(empty_snapshot_message()),
        (_, true) => {
            let line = line as usize;

            let crate_root = crate::paths::find_crate_root(file).unwrap();

            let mut this_file_path = crate_root;
            this_file_path.push(file);

            if let Some(snapshot) = snapshot {
                let need_updating = snapshot_matching_message(&s, snapshot).is_some();

                if need_updating {
                    let mode = UpdateInlineSnapshotMode::Replace;
                    schedule_snapshot_update(this_file_path, line, &s, mode).unwrap();
                }
            } else {
                let mode = UpdateInlineSnapshotMode::Create;
                schedule_snapshot_update(this_file_path, line, &s, mode).unwrap();
            };

            None
        }
    }
}

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

#[derive(Clone, Copy, Debug)]
enum UpdateInlineSnapshotMode {
    Create,  // when there's no inline snapshot
    Replace, // when there's an existing inline snapshot
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

#[derive(Debug)]
pub struct InlineSnapshotUpdate {
    range: Range,
    new_value: String,
    mode: UpdateInlineSnapshotMode,
}

#[derive(Debug)]
pub struct SourceFile {
    pub content: String,
    pub updates: Vec<InlineSnapshotUpdate>,
    pub path: types::FilePath,
}

impl SourceFile {
    fn new(path: types::FilePath) -> Result<Self> {
        let content = Self::read(&path)?;
        Ok(Self {
            path,
            content,
            updates: vec![],
        })
    }

    // read source file content and panic if the file on disk changed
    pub fn read_and_compare(&self) -> Result<()> {
        let read_content = Self::read(&self.path)?;

        if read_content != self.content {
            anyhow::bail!("File content was modified during test run");
        }
        Ok(())
    }

    pub fn read(absolute_path: &str) -> Result<String> {
        std::fs::read_to_string(absolute_path)
            .with_context(|| format!("Can't read source file. File path: {}", absolute_path))
    }

    pub fn write(&self) {
        std::fs::write(&self.path, &self.content).unwrap();
    }

    pub fn format(&mut self) {
        use std::process::Command;
        // Don't blow up if failed to format. TODO: find a way to
        // print a message about broken rustfmt
        let _output = Command::new("rustfmt").arg(&self.path).output();
    }
}

pub fn with_source_file<F, T>(absolute_path: &str, f: F) -> Result<T>
where
    F: FnOnce(&mut SourceFile) -> Result<T>,
{
    let mut map = SOURCE_FILES.lock().expect("poisoned lock");
    let mut source_file = map
        .as_mut()
        .unwrap()
        .entry(absolute_path.to_string())
        .or_insert_with(|| SourceFile::new(absolute_path.to_string()).unwrap());

    f(&mut source_file)
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
            update_instructions = crate::config::update_instructions(),
        )
    })
}

fn empty_snapshot_message() -> String {
    format!(
        "Expected {string_desc} to match {snapshot_desc}

but that assertion did not have any inline snapshots.

{update_instructions}
",
        string_desc = "string".red(),
        snapshot_desc = "inline snapshot".green(),
        update_instructions = crate::config::update_instructions(),
    )
}

extern "C" fn libc_atexit_hook() {
    let files = SOURCE_FILES.lock().expect("poisoned lock").take().unwrap();

    for (_path, file) in files {
        update_inline_snapshots(file).expect("failed to update snapshots");
    }
}

fn maybe_register_atexit_hook() {
    // https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
    if !ATEXIT_HOOK_REGISTERED.swap(true, Ordering::SeqCst) {
        unsafe {
            libc::atexit(libc_atexit_hook);
        }
    }
}

fn schedule_snapshot_update(
    file_path: PathBuf,
    original_line_num: usize,
    to_add: &str,
    mode: UpdateInlineSnapshotMode,
) -> Result<()> {
    maybe_register_atexit_hook();

    with_source_file(&file_path.display().to_string(), |file| {
        let range = find_inline_snapshot_range(&file.content, original_line_num, mode).unwrap();

        file.updates.push(InlineSnapshotUpdate {
            range,
            new_value: to_add.to_string(),
            mode,
        });

        Ok(())
    })
}

fn update_inline_snapshots(mut file: SourceFile) -> Result<()> {
    file.read_and_compare()?;
    let content = file.content.clone();

    let mut updates = file.updates.iter().collect::<Vec<_>>();
    updates.sort_by(|a, b| a.range.start.line.cmp(&b.range.start.line));

    let parts = split_by_ranges(content, updates.iter().map(|u| &u.range).collect());

    assert_eq!(parts.len(), updates.len() + 1);

    let mut parts_iter = parts.into_iter();
    let mut updates_iter = updates.into_iter();
    let mut result = String::new();

    loop {
        match (parts_iter.next(), updates_iter.next()) {
            (Some(part), Some(update)) => {
                result.push_str(&part);

                let comma_separator = match update.mode {
                    UpdateInlineSnapshotMode::Create => ", ",
                    UpdateInlineSnapshotMode::Replace => "",
                };

                let update_string = format!(
                    "{comma_separator}r##\"{to_add}\"##",
                    comma_separator = comma_separator,
                    to_add = &update.new_value,
                );

                result.push_str(&update_string);
            }
            (Some(part), None) => {
                result.push_str(&part);
            }
            (None, None) => {
                break;
            }
            _ => panic!("unreachable"),
        }
    }
    file.content = result;
    file.write();
    file.format();
    Ok(())
}

fn find_inline_snapshot_range(
    file_content: &str,
    line_num: usize,
    mode: UpdateInlineSnapshotMode,
) -> Result<Range, String> {
    let syntax = syn::parse_file(file_content).expect("Unable to parse file");

    let mut macro_visitor = MacroVisitor {
        found: None,
        line: line_num,
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
                        // columns might be 0 based? i'm not sure
                        column: literal.span().start().column + 1,
                    },
                    end: LineColumn {
                        line: literal.span().end().line,
                        column: literal.span().end().column + 1,
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
                    column: span.end().column + 1,
                },
                end: LineColumn {
                    line: span.end().line,
                    column: span.end().column + 1,
                },
            })
        }
    }
}

fn split_by_ranges(content: String, ranges: Vec<&Range>) -> Vec<String> {
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
