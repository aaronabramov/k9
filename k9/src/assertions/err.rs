use colored::*;
use std::fmt::Debug;

pub fn assert_err<T: Debug, E: Debug>(value: Result<T, E>) -> Option<String> {
    if value.is_err() {
        None
    } else {
        Some(format!(
            "Expected {value_desc} to be {type_desc}

Got: {value}
        ",
            value_desc = "Value".red(),
            type_desc = "Err(E)".green(),
            value = format!("{:?}", value).red(),
        ))
    }
}
