use colored::*;
use std::cmp::Ordering;
use std::fmt::Debug;

pub fn assert_greater_than<T: Debug + PartialOrd>(left: T, right: T) -> Option<String> {
    let not_greater_than = match left.partial_cmp(&right) {
        None | Some(Ordering::Greater) => false,
        _ => true,
    };

    // If left is not greater than right
    if not_greater_than {
        let message = format!(
            "Expected {left_desc} value to be greater than {right_desc} value

Left value:  {left}
Right value: {right}
",
            left_desc = "Left".red(),
            right_desc = "Right".green(),
            left = format!("{:#?}", left).red(),
            right = format!("{:#?}", right).green(),
        );

        Some(message)
    } else {
        None
    }
}
