use crate::string_diff::colored_diff;
use colored::*;
use std::fmt::Debug;

pub fn assert_greater_than<T: Debug + PartialOrd>(left: T, right: T) -> Option<String> {
    // If left is not greater than right
    if !(left > right) {
        let diff_string = colored_diff(&format!("{:#?}", &left), &format!("{:#?}", &right))
            .unwrap_or_else(|| format!("{:#?} is not greater than {:#?}", left, right).to_string());

        let message = format!(
            "
Expected `{left_desc}` to be greater than `{right_desc}`:
{diff_string}",
            left_desc = "Left".red(),
            right_desc = "Right".green(),
            diff_string = &diff_string
        );

        Some(message)
    } else {
        None
    }
}
