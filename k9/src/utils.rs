pub fn add_linebreaks(s: &str) -> String {
    format!("\n{}\n", s)
}

pub fn terminal_separator_line() -> String {
    let width_override = crate::config::terminal_width_override();
    let width = if width_override != 0 {
        width_override
    } else if let Some((width, _)) = terminal_size::terminal_size() {
        width.0 as usize
    } else {
        100 // default width if we can't determine terminal width
    };

    "━".repeat(width)
}
