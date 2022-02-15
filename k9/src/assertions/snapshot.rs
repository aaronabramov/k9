use crate::snapshot::ast;
use crate::snapshot::source_code;
use crate::snapshot::source_code::Range;
use crate::types;
use anyhow::{Context, Result};
use colored::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Debug;
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

pub fn snapshot<V: Debug>(
    value: V,
    snapshot: Option<&str>,
    line: u32,
    _column: u32,
    file: &str,
) -> Option<String> {
    snapshot_internal(value, snapshot, line, file)
        .context("snapshot!() macro failed")
        .unwrap()
}

pub fn snapshot_internal<V: Debug>(
    value: V,
    snapshot: Option<&str>,
    line: u32,
    file: &str,
) -> Result<Option<String>> {
    let value_str = value_to_string(value);
    match (snapshot, crate::config::CONFIG.update_mode) {
        (Some(snapshot), false) => Ok(snapshot_matching_message(&value_str, snapshot)),
        (None, false) => Ok(Some(empty_snapshot_message(&value_str))),
        (_, true) => {
            let line = line as usize;

            let crate_root =
                crate::paths::find_crate_root(file).context("Failed to find crate root")?;

            let mut this_file_path = crate_root;
            this_file_path.push(file);

            if let Some(snapshot) = snapshot {
                let need_updating = snapshot_matching_message(&value_str, snapshot).is_some();

                if need_updating {
                    let mode = UpdateInlineSnapshotMode::Replace;
                    schedule_snapshot_update(this_file_path, line, &value_str, mode).unwrap();
                }
            } else {
                let mode = UpdateInlineSnapshotMode::Create;
                schedule_snapshot_update(this_file_path, line, &value_str, mode).unwrap();
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
    let source_file = map
        .as_mut()
        .context("Missing source file")?
        .entry(absolute_path.to_string())
        .or_insert_with(|| SourceFile::new(absolute_path.to_string()).unwrap());

    f(source_file)
}

fn snapshot_matching_message(s: &str, snapshot: &str) -> Option<String> {
    let diff = crate::string_diff::colored_diff(snapshot, s);

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

fn empty_snapshot_message(s: &str) -> String {
    format!(
        "Expected {string_desc} to match {snapshot_desc}

but that assertion did not have any inline snapshots.

Received value:
{received_value}

{update_instructions}
",
        string_desc = "string".red(),
        snapshot_desc = "inline snapshot".green(),
        update_instructions = crate::config::update_instructions(),
        received_value = s.green(),
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
            "snapshot",
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

                let literal = make_literal(&update.new_value)?;

                let update_string = format!(
                    "{comma_separator}{to_add}",
                    comma_separator = comma_separator,
                    to_add = literal
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

/// Format the value passed into snapshot macro to a snapshot, which
/// can be either compared to existing snapshot or used as a value to
/// update snapshots to.
fn value_to_string<V: Debug>(value: V) -> String {
    let mut s = format!("{:#?}", value);

    // Undebug string newlines.
    // Formatting string as `Debug` escapes all newline characters
    // with `\\n` (escaped newlines), so they actually get printed as `\n`
    // which is super hard to read and it defeats the purpose of multiline
    // snapshots. This will replace them back to be displayed as newlines
    s = s.replace(r#"\n"#, "\n");

    // Debug representation of a string also has quotes escaped, which can get
    // pretty noisy. We'll unescape them too.
    s = s.replace(r#"\""#, r#"""#);
    s = s.replace(r#"\'"#, r#"'"#);

    let mut chars = s.chars();

    // `Debug` of a string also wraps the printed value in leading and trailing "
    // We'll trim these quotes in this case. This is a bit risky, since
    // not only `String` dbg can produce leading and trailing ". But currently
    // there's no other easy way to apply certain formatting to `String`s only
    if let (Some('"'), Some('"')) = (chars.next(), chars.next_back()) {
        s = chars.collect();
    }

    if s.contains('\n') {
        // If it's a multiline string, we always add a leading and trailing `\n`
        // to avoid awkward macros like
        //      snapshot!("hello
        // world");
        s = format!("\n{}\n", s);
    }
    s
}

fn make_literal(s: &str) -> Result<String> {
    // If snapshot doesn't contain any of these characters characters
    // wrap the string in "" and use it as a literal
    // Otherwise we'd need to use r#""# literals to avoid crazy escaping rules
    if !s.contains('"') && !s.contains('\'') && !s.contains('\\') {
        return Ok(format!(r#""{}""#, s));
    }

    // Otherwise try incrementing the number of # and see if
    // that results in a properly escaped string.
    for i in 0..5 {
        let esc = "#".repeat(i);
        let end = format!("\"{}", &esc);

        if !s.contains(&end) {
            return Ok(format!(r#"r{}"{}"{}"#, esc, s, esc));
        }
    }

    anyhow::bail!("Failed to create snapshot string literal")
}
