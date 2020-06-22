use colored::*;

pub fn update_instructions() -> colored::ColoredString {
    "run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots".yellow()
}
