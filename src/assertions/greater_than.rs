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
        let diff_string = greater_than_diff(&format!("{:#?}", &left), &format!("{:#?}", &right))
            .unwrap_or_else(|| format!("{:#?} is not greater than {:#?}", left, right));

        let message = format!(
            "
Expected `{left_desc}` to be greater than `{right_desc}` but it wasn't:
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

// assert_greater_than difference output if assertion fails
// Example:
// assert_greater_than!(1, 1);
//
// Assertion Failure!
//
// Expected `Left` to be greater than `Right` but it wasn't:
//
// Expected: > 1
// Received:   1
fn greater_than_diff(left: &str, right: &str) -> Option<String> {
    let mut result = String::new();

    if left > right {
        return None;
    }

    result.push_str("\n");
    result.push_str(&format!("Expected: {} {}\n", ">".green(), &right.green()));
    result.push_str(&format!("Received:   {}\n", &left.red()));

    Some(result)
}
