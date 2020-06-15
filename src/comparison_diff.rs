use colored::*;

pub fn greater_than_diff(left: &str, right: &str) -> Option<String> {
    let mut result = String::new();

    if left > right {
        return None;
    }

    result.push_str("\n");
    result.push_str(&format!("Expected: {} {}\n", ">".green(), &right.green()));
    result.push_str(&format!("Received:   {}\n", &left.red()));

    Some(result)
}
