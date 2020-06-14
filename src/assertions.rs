use crate::utils;
use colored::*;

pub mod equal;
pub mod err_matches_regex;
pub mod matches_regex;
pub mod matches_snapshot;

#[derive(Debug)]
pub struct Assertion {
    // Description of what's being asserted to provide a bit more context in the error mesasge
    pub description: Option<String>,
    // the name of the assertion macro that wan invoked. e.g. `assert_equals`
    pub name: String,
    // string containing all arguments passed to the assertion macro. e.g. "1 + 1, my_var"
    pub args_str: String,
    // Assertion failure message, e.g. `expected blah blah but got blah`
    pub failure_message: Option<String>,
}

impl Assertion {
    pub fn assert(&self) -> Option<String> {
        self.failure_message.as_ref().map(|failure_message| {
            let message = format!(
                "
{separator}
{assertion_expression}
{description}
{failure_message}
{separator}",
                assertion_expression = self.assertion_expression(),
                description = utils::add_linebreaks(
                    self.description
                        .as_ref()
                        .unwrap_or(&"Assertion Failure!".to_string())
                ),
                failure_message = failure_message,
                separator = utils::terminal_separator_line().dimmed(),
            );

            if crate::config::should_panic() {
                panic!(message);
            }
            message
        })
    }

    pub fn assertion_expression(&self) -> String {
        format!(
            "{assertion_name}({args});",
            assertion_name = format!("{}!", self.name).yellow(),
            args = self.args_str
        )
    }
}

pub fn make_assertion(
    name: &str,
    args_str: String,
    failure_message: Option<String>,
    description: Option<&str>,
) -> Option<String> {
    if let Some(failure_message) = failure_message {
        (Assertion {
            description: description.map(|d| d.into()),
            failure_message: Some(failure_message),
            name: name.to_string(),
            args_str,
        })
        .assert()
    } else {
        None
    }
}

/// Asserts that two passed arguments are equal.
/// panics if they are not
///
/// ```rust
/// use k9::assert_equal;
///
/// // simple values
/// assert_equal!(1, 1);
///
/// #[derive(Debug, PartialEq)]
/// struct A {
///     name: &'static str
/// }
///
/// let a1 = A { name: "Kelly" };
/// let a2 = A { name: "Kelly" };
///
/// // this will print the visual difference between two structs
/// assert_equal!(a1, a2);
/// ```
#[macro_export]
macro_rules! assert_equal {
    ($left:expr, $right:expr) => {{
        use colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::assertions::make_assertion(
            "assert_equal",
            args_str,
            $crate::assertions::equal::assert_equal($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_equal",
            args_str,
            $crate::assertions::equal::assert_equal($left, $right),
            Some(&$description),
        )
    }};
}

/// Asserts that passed `&str` matches a regular expression.
/// Regular expressions are compiled using `regex` crate.
///
/// ```rust
/// use k9::assert_matches_regex;
///
/// assert_matches_regex!("1234-45", "\\d{4}-\\d{2}");
/// assert_matches_regex!("abc", "abc");
/// ````
#[macro_export]
macro_rules! assert_matches_regex {
    ($s:expr, $regex:expr) => {{
        use colored::*;
        let args_str = format!("{}, {}", stringify!($s).red(), stringify!($regex).green());
        $crate::assertions::make_assertion(
            "assert_matches_regex",
            args_str,
            $crate::assertions::matches_regex::assert_matches_regex($s, $regex),
            None,
        )
    }};
    ($s:expr, $regex:expr, $description:expr) => {{
        use colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($s).red(),
            stringify!($regex).green(),
            stringify!($description).dimmed()
        );
        $crate::assertions::make_assertion(
            "assert_matches_regex",
            args_str,
            $crate::assertions::matches_regex::assert_matches_regex($s, $regex),
            Some($description),
        )
    }};
}

/// Asserts that the passed `Result` argument is an `Err` and
/// and the debug string of that error matches provided regex.
/// Regular expressions are compiled using `regex` crate.
///
/// ```rust
/// use k9::assert_err_matches_regex;
/// // Borrowed from Rust by Example: https://doc.rust-lang.org/stable/rust-by-example/std/result.html
/// fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
/// if y == 0.0 {
/// // This operation would `fail`, instead let's return the reason of
/// // the failure wrapped in `Err`
/// Err("Cannot divide by 0.")
/// } else {
/// // This operation is valid, return the result wrapped in `Ok`
/// Ok(x / y)
/// }
/// }
/// let division_error = divide(4.0, 0.0);
/// assert_err_matches_regex!(division_error, "Cannot");
/// ```
#[macro_export]
macro_rules! assert_err_matches_regex {
    ($err:expr, $regex:expr) => {{
        use colored::*;
        let args_str = format!("{}, {}", stringify!($err).red(), stringify!($regex).green(),);
        $crate::assertions::make_assertion(
            "assert_err_matches_regex",
            args_str,
            $crate::assertions::err_matches_regex::assert_err_matches_regex($err, $regex),
            None,
        )
    }};
    ($s:expr, $regex:expr, $context:expr) => {{
        use colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($err).red(),
            stringify!($regex).green(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_err_matches_regex",
            args_str,
            $crate::assertions::err_matches_regex::assert_err_matches_regex($err, $regex),
            Some($description),
        )
    }};
}

/// Formats passed value and asserts that it matches existing snaphot.
/// If snapshot file for this test does not exist, test can be run with `K9_UPDATE_SNAPSHOTS=1`
/// environment variable to either create or replace existing snapshot file.
/// Snapshots will be written into `__k9_snapshots__` directory next to the test file.
///
/// ```rust
/// #[test]
/// fn my_test() {
///     struct A {
///         name: &'a str,
///         age: u32
///     }
///
///     let a = A { name: "Lance", age: 9 };
///
///     // When first run with `K9_UPDATE_SNAPSHOTS=1` it will
///     // create `__k9_snapshots__/my_test_file/my_test.snap` file
///     // with contents being the serialized value of `a`.
///     // Next time the test is run, if the newly serialized value of a
///     // is different from the value of that snapshot file, the assertion
///     // will fail.
///     assert_matches_snapshot!(a);
/// }
/// ```
#[macro_export]
macro_rules! assert_matches_snapshot {
    ($to_snap:expr) => {{
        use colored::*;
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!("{}", stringify!($to_snap).red(),);
        $crate::assertions::make_assertion(
            "assert_matches_snapshot",
            args_str,
            $crate::assertions::matches_snapshot::snap_internal($to_snap, line, column, file),
            None,
        )
    }};
    ($to_snap:expr, $description:expr) => {{
        use colored::*;
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!(
            "{}, {}",
            stringify!($to_snap).red(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_matches_snapshot",
            args_str,
            $crate::assertions::matches_snapshot::snap_internal($to_snap, line, column, file),
            Some($description),
        )
    }};
}
