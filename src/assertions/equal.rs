use crate::string_diff::colored_diff;
use colored::*;
use std::fmt::Debug;

pub fn assert_equal<T: Debug + PartialEq>(left: T, right: T) -> Option<String> {
    if left != right {
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
