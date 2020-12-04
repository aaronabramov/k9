use colored::*;
use std::cmp::Ordering;
use std::fmt::Debug;

pub fn assert_lesser_than_or_equal<T: Debug + PartialOrd>(left: T, right: T) -> Option<String> {
    let cmp = left.partial_cmp(&right);
    match cmp {
        Some(Ordering::Less) | Some(Ordering::Equal) => None,
        _ => {
            let reason = if cmp.is_none() {
                ",\nbut these values can't be compared"
            } else {
                ""
            };

            Some(format!(
                "Expected {left_desc} value to be lesser than or equal to {right_desc} value{reason}

Left value:  {left}
Right value: {right}
",
                left_desc = "Left".red(),
                right_desc = "Right".green(),
                reason = reason,
                left = format!("{:#?}", left).red(),
                right = format!("{:#?}", right).green(),
            ))
        }
    }
}
