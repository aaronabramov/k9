use colored::*;
use std::fmt::Debug;

pub fn assert_contains<V: PartialEq + Debug, I: IntoIterator<Item = V> + Debug + Clone>(
    left: I,
    right: V,
) -> Option<String> {
    if left.clone().into_iter().any(|item| item == right) {
        None
    } else {
        Some(format!(
            "Expected {left_desc} value to contain {right_desc} value

Left value: {left}
Right value: {right}
    ",
            left_desc = "Left".red(),
            right_desc = "Right".green(),
            left = format!("{:#?}", left).red(),
            right = format!("{:#?}", right).green(),
        ))
    }
}
