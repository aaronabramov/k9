use crate::snapshot::ast;
use crate::snapshot::source_code;
use crate::snapshot::source_code::Range;
use crate::types;
use anyhow::{Context, Result};
use colored::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

lazy_static! {
    static ref SOURCE_FILES: Mutex<Option<HashMap<types::FilePath, SourceFile>>> =
        Mutex::new(Some(HashMap::new()));
    static ref ATEXIT_HOOK_REGISTERED: AtomicBool = AtomicBool::new(false);
}

extern "C" fn libc_atexit_hook() {
    let files = SOURCE_FILES.lock().expect("poisoned lock").take().unwrap();

    for (_path, file) in files {
        update_inline_snapshots(file).expect("Failed to update snapshots");
    }
}

fn maybe_register_atexit_hook() {
    if !ATEXIT_HOOK_REGISTERED.swap(true, Ordering::SeqCst) {
        unsafe {
            libc::atexit(libc_atexit_hook);
        }
    }
}

pub fn snapshot(
    s: String,
    snapshot: Option<&str>,
    line: u32,
    _column: u32,
    file: &str,
) -> Option<String> {
    snapshot_internal(s, snapshot, line, file)
        .context("snapshot!() macro failed")
        .unwrap()
}

pub fn snapshot_internal(
    s: String,
    snapshot: Option<&str>,
    line: u32,
    file: &str,
) -> Result<Option<String>> {
    match (snapshot, crate::config::CONFIG.update_mode) {
        (Some(snapshot), false) => Ok(snapshot_matching_message(&s, snapshot)),
        (None, false) => Ok(Some(empty_snapshot_message())),
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

            Ok(None)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UpdateInlineSnapshotMode {
    Create,  // when there's no inline snapshot
    Replace, // when there's an existing inline snapshot
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
        .context("Missing source file")?
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

fn schedule_snapshot_update(
    file_path: PathBuf,
    original_line_num: usize,
    to_add: &str,
    mode: UpdateInlineSnapshotMode,
) -> Result<()> {
    maybe_register_atexit_hook();

    with_source_file(&file_path.display().to_string(), |file| {
        let range = ast::find_snapshot_literal_range(
            &file.content,
            "_snapshot",
            original_line_num,
            mode == UpdateInlineSnapshotMode::Replace,
        )
        .with_context(|| {
            format!(
                "Failed to find the origin of snapshot macro call in `{}`",
                &file_path.display()
            )
        })?;

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

    let parts = source_code::split_by_ranges(content, updates.iter().map(|u| &u.range).collect());

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
