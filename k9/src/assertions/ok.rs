use colored::*;
use std::fmt::Debug;

pub fn assert_ok<T: Debug>(value: Result<T, T>) -> Option<String> {
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
