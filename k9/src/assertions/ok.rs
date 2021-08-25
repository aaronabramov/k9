use colored::*;
use std::fmt::Debug;

pub fn assert_ok<T: Debug, E: Debug>(value: Result<T, E>) -> Option<String> {
    if value.is_ok() {
        None
    } else {
        Some(format!(
            "Expected {value_desc} to be {type_desc}

Got: {value}
        ",
            value_desc = "Value".red(),
            type_desc = "Ok(T)".green(),
            value = format!("{:?}", value).red(),
        ))
    }
}
