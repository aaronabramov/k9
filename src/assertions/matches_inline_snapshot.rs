use crate::types;
use colored::*;
use lazy_static::lazy_static;
use ra_syntax::{AstNode, SyntaxKind};
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

#[derive(Clone, Copy)]
enum UpdateInlineSnapshotMode {
    Create,  // when there's no inline snapshot
    Replace, // when there's an existing inline snapshot
    NoOp,    // no need to update anything, current snapshot is valid
}
// struct that represents a modification to the source code. E.g. we added/updated inline snapshot
#[derive(Debug)]
pub struct Patch {
    pub offset: u32, // offset in bytes at where the patch occurred
    pub shift: i32,  // how many bytes were added/removed after that offset
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
    line_num: u32,
    column_num: u32,
    to_add: &str,
    mode: UpdateInlineSnapshotMode,
) -> Result<(), String> {
    if let UpdateInlineSnapshotMode::NoOp = mode {
        // no need to update anything, snapshot is up to date
        return Ok(());
    }

    with_source_file(&file_path.display().to_string(), |file| {
        file.read_and_compare();
        let mut content = file.content.take().expect("empty file content. read first");

        let original_offset = crate::parsing::line_and_column_to_offset(
            file.original_content.as_ref().unwrap(),
            line_num,
            column_num,
        )
        .expect("failed to get char offset in the file from provided line!() and column!()")
            as i32;

        let mut new_offset = original_offset;

        for patch in &file.patches {
            if patch.offset <= original_offset as u32 {
                new_offset += patch.shift;
            }
        }

        let range_to_replace =
            find_inline_snapshot_range(&content, new_offset as usize, mode).unwrap();

        let mut rest = content.split_off(range_to_replace.0);
        let before = content;
        let after = rest.split_off(range_to_replace.1 - range_to_replace.0); // part we're replacing

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

        let bytes_before = range_to_replace.1 - range_to_replace.0;
        let bytes_after = replace_with.len();

        file.patches.push(Patch {
            offset: original_offset as u32,
            shift: bytes_after as i32 - bytes_before as i32,
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
    offset: usize,
    mode: UpdateInlineSnapshotMode,
) -> Result<(usize, usize), String> {
    let file = ra_syntax::File::parse(file_content);
    let mut root = file.ast().syntax();
    let mut needle = None;

    // Binary search the entire file's AST for that syntax node at an `offset` position
    loop {
        let range = root.range();
        let start = range.start().to_usize();
        let end = range.end().to_usize();

        if start == offset || start == offset + 1 {
            needle.replace(root);
            break; // that's the node!
        }

        if end < offset {
            panic!("while traversing AST, the root node end offset `{}` was lower than the offset we were looking for `{}`", end, offset);
        }

        if root.children().count() == 0 {
            panic!("cant' find the AST node of assert_matches_inline_snapshot! macro");
        }

        for child in root.children() {
            let range = child.range();
            let start = range.start().to_usize();
            let end = range.end().to_usize();

            if start <= offset && end > offset {
                root = child;
                break;
            }
        }
    }

    let needle = needle.expect("could not find the AST node for inline snapshot");

    assert_eq!(needle.kind(), SyntaxKind::EXPR_STMT);

    let macro_call = crate::parsing::ast_dfs_find_node(needle.owned(), |node| match node.kind() {
        SyntaxKind::MACRO_CALL => Ok(true),
        _ => Ok(false),
    })
    .expect("errored while searching for macro call AST node")
    .expect("Failed to find MACRO_CALL AST node for inline_snapshot");

    let path = macro_call
        .first_child()
        .expect("macro call must have a path");

    assert_eq!(&path.text().to_string(), "assert_matches_inline_snapshot");

    let token_tree = macro_call.last_child().unwrap();
    assert_eq!(token_tree.kind(), SyntaxKind::TOKEN_TREE);

    match mode {
        UpdateInlineSnapshotMode::Replace => {
            let mut children: Vec<_> = token_tree.children().collect();
            let closing_paren = children.pop().expect("must have closing paren");
            assert_eq!(closing_paren.kind(), SyntaxKind::R_PAREN);

            let mut needle = None;
            for node in children.into_iter().rev() {
                match node.kind() {
                    SyntaxKind::WHITESPACE | SyntaxKind::COMMENT => continue,
                    SyntaxKind::STRING => {
                        needle = Some((
                            node.range().start().to_usize(),
                            node.range().end().to_usize(),
                        ));
                        break;
                    }
                    _ => {
                        panic!(
                        "Unexpected token while parsing `assert_matches_inline_snapshot` macro call.
                        kind: `{:?}`, text: `{:?}`",
                        node.kind(),
                        node.text(),
                    );
                    }
                }
            }

            Ok(needle.expect("Could not find inline snapshot string literal in macro call"))
        }
        UpdateInlineSnapshotMode::Create => {
            let last_paren = token_tree.last_child().unwrap();
            let last_paren_start = last_paren.range().start();
            Ok((last_paren_start.to_usize(), last_paren_start.to_usize()))
        }
        UpdateInlineSnapshotMode::NoOp => panic!("unreachable"),
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
