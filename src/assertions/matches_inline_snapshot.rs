use colored::*;
use ra_syntax::{AstNode, SyntaxKind};
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum UpdateInlineSnapshotMode {
    Create,  // when there's no inline snapshot
    Replace, // when there's an existing inline snapshot
    NoOp,    // no need to update anything, current snapshot is valid
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

    let mut file_content =
        std::fs::read_to_string(file_path.display().to_string()).expect("can't read snapshot file");

    let range_to_replace =
        find_inline_snapshot_range(&file_content, line_num, column_num, mode).unwrap();

    let mut rest = file_content.split_off(range_to_replace.0);
    let before = file_content;
    let after = rest.split_off(range_to_replace.1 - range_to_replace.0); // part we're replacing

    let comma_separator = match mode {
        UpdateInlineSnapshotMode::Create => ", ",
        UpdateInlineSnapshotMode::Replace => "",
        UpdateInlineSnapshotMode::NoOp => panic!("unreachable"),
    };

    let new_content = format!(
        "{before}{comma_separator} \"{to_add}\"{after}",
        before = before,
        comma_separator = comma_separator,
        to_add = escape_snapshot_string_literal(to_add),
        after = after
    );

    std::fs::write(&file_path, new_content).unwrap();
    Ok(())
}

fn find_inline_snapshot_range(
    file_content: &str,
    line_num: u32,
    column_num: u32,
    mode: UpdateInlineSnapshotMode,
) -> Result<(usize, usize), String> {
    let offset = crate::parsing::line_and_column_to_offset(file_content, line_num, column_num)
        .expect("failed to get char offset in the file from provided line!() and column!()")
        as usize;

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
                    SyntaxKind::WHITESPACE => continue,
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
