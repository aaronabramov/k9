use crate::string_diff::colored_diff;
use colored::*;
use std::fmt::Debug;

pub fn assert_equal<T1: Debug + PartialEq, T2: Debug + PartialEq>(
    left: T1,
    right: T2,
    fail: bool,
) -> Option<String> {
    if fail {
        let diff_string = colored_diff(&format!("{:#?}", &left), &format!("{:#?}", &right))
            .unwrap_or_else(|| "no visual difference between values".to_string());

        let message = format!(
            "
Expected `{left_desc}` to equal `{right_desc}`:
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
