use colored::*;
use std::cmp::Ordering;
use std::fmt::Debug;

pub fn assert_lesser_than<T: Debug + PartialOrd>(left: T, right: T) -> Option<String> {
    let cmp = left.partial_cmp(&right);

    if let Some(Ordering::Less) = cmp {
        None
    } else {
        let reason = match cmp {
            None => ",\nbut these values can't be compared",
            Some(Ordering::Equal) => ",\nbut they were equal",
            _ => "",
        };

        Some(format!(
            "Expected {left_desc} value to be lesser than {right_desc} value{reason}

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
