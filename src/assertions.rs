use crate::utils;
use colored::*;

#[cfg(feature = "regex")]
pub mod err_matches_regex;
#[cfg(feature = "regex")]
pub mod matches_regex;

pub mod equal;
pub mod err;
pub mod greater_than;
pub mod greater_than_or_equal;
pub mod lesser_than;
pub mod lesser_than_or_equal;
pub mod matches_inline_snapshot;
pub mod matches_snapshot;
pub mod ok;

#[derive(Debug)]
pub struct Assertion {
    /// Description of what's being asserted to provide a bit more context in the error mesasge
    pub description: Option<String>,
    /// the name of the assertion macro that was invoked. e.g. `assert_equals`
    pub name: String,
    /// string containing all arguments passed to the assertion macro. e.g. "1 + 1, my_var"
    pub args_str: String,
    /// Assertion failure message, e.g. `expected blah blah but got blah`
    pub failure_message: String,
}

impl Assertion {
    pub fn panic(&self) {
        panic!(self.get_failure_message());
    }

    pub fn get_failure_message(&self) -> String {
        let message = format!(
            "
{separator}
{assertion_expression}
{description}
{failure_message}
{separator}
",
            assertion_expression = self.assertion_expression(),
            description = utils::add_linebreaks(
                self.description
                    .as_ref()
                    .unwrap_or(&"Assertion Failure!".to_string())
            ),
            failure_message = self.failure_message,
            separator = utils::terminal_separator_line().dimmed(),
        );

        message
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
) -> Option<Assertion> {
    if let Some(failure_message) = failure_message {
        let assertion = Assertion {
            description: description.map(|d| d.into()),
            failure_message,
            name: name.to_string(),
            args_str,
        };
        if crate::config::should_panic() {
            assertion.panic();
        }
        Some(assertion)
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
        use $crate::__macros__::colored::*;
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
        use $crate::__macros__::colored::*;
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

/// Asserts if left is greater than right.
/// panics if they are not
///
/// ```rust
/// use k9::assert_greater_than;
///
/// assert_greater_than!(2, 1);
/// ```
#[macro_export]
macro_rules! assert_greater_than {
    ($left:expr, $right:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::assertions::make_assertion(
            "assert_greater_than",
            args_str,
            $crate::assertions::greater_than::assert_greater_than($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_greater_than",
            args_str,
            $crate::assertions::greater_than::assert_greater_than($left, $right),
            Some(&$description),
        )
    }};
}

/// Asserts if left greater than or equal to right.
/// panics if they are not
///
/// ```rust
/// use k9::assert_greater_than_or_equal;
///
/// assert_greater_than_or_equal!(2, 1);
/// assert_greater_than_or_equal!(1, 1);
/// ```
#[macro_export]
macro_rules! assert_greater_than_or_equal {
    ($left:expr, $right:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::assertions::make_assertion(
            "assert_greater_than_or_equal",
            args_str,
            $crate::assertions::greater_than_or_equal::assert_greater_than_or_equal($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_greater_than_or_equal",
            args_str,
            $crate::assertions::greater_than_or_equal::assert_greater_than_or_equal($left, $right),
            Some(&$description),
        )
    }};
}

/// Asserts if left is lesser than right.
/// panics if they are not
///
/// ```rust
/// use k9::assert_lesser_than;
///
/// assert_lesser_than!(1, 2);
/// ```
#[macro_export]
macro_rules! assert_lesser_than {
    ($left:expr, $right:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::assertions::make_assertion(
            "assert_lesser_than",
            args_str,
            $crate::assertions::lesser_than::assert_lesser_than($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_lesser_than",
            args_str,
            $crate::assertions::lesser_than::assert_lesser_than($left, $right),
            Some(&$description),
        )
    }};
}

/// Asserts if left lesser than or equal to right.
/// panics if they are not
///
/// ```rust
/// use k9::assert_lesser_than_or_equal;
///
/// assert_lesser_than_or_equal!(1, 2);
/// assert_lesser_than_or_equal!(1, 1);
/// ```
#[macro_export]
macro_rules! assert_lesser_than_or_equal {
    ($left:expr, $right:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::assertions::make_assertion(
            "assert_lesser_than_or_equal",
            args_str,
            $crate::assertions::lesser_than_or_equal::assert_lesser_than_or_equal($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_lesser_than_or_equal",
            args_str,
            $crate::assertions::lesser_than_or_equal::assert_lesser_than_or_equal($left, $right),
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
#[cfg(feature = "regex")]
#[macro_export]
macro_rules! assert_matches_regex {
    ($s:expr, $regex:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!("{}, {}", stringify!($s).red(), stringify!($regex).green());
        $crate::assertions::make_assertion(
            "assert_matches_regex",
            args_str,
            $crate::assertions::matches_regex::assert_matches_regex($s, $regex),
            None,
        )
    }};
    ($s:expr, $regex:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
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
///     if y == 0.0 {
///         // This operation would `fail`, instead let's return the reason of
///         // the failure wrapped in `Err`
///         Err("Cannot divide by 0.")
///     } else {
///         // This operation is valid, return the result wrapped in `Ok`
///         Ok(x / y)
///     }
/// }
/// let division_error = divide(4.0, 0.0);
/// assert_err_matches_regex!(division_error, "Cannot");
/// ```
#[cfg(feature = "regex")]
#[macro_export]
macro_rules! assert_err_matches_regex {
    ($err:expr, $regex:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!("{}, {}", stringify!($err).red(), stringify!($regex).green(),);
        $crate::assertions::make_assertion(
            "assert_err_matches_regex",
            args_str,
            $crate::assertions::err_matches_regex::assert_err_matches_regex($err, $regex),
            None,
        )
    }};
    ($s:expr, $regex:expr, $context:expr) => {{
        use $crate::__macros__::colored::*;
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
        use $crate::__macros__::colored::*;
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
        use $crate::__macros__::colored::*;
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

#[macro_export]
macro_rules! assert_matches_inline_snapshot {
    ($to_snap:expr) => {{
        use $crate::__macros__::colored::*;
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!("{}", stringify!($to_snap).red(),);
        let s: String = $to_snap.into();
        $crate::assertions::make_assertion(
            "assert_matches_inline_snapshot",
            args_str,
            $crate::assertions::matches_inline_snapshot::matches_inline_snapshot(
                s, None, line, column, file,
            ),
            None,
        )
    }};
    ($to_snap:expr, $inline_snap:literal) => {{
        use $crate::__macros__::colored::*;
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!(
            "{}, {}",
            stringify!($to_snap).red(),
            stringify!($inline_snap).green(),
        );
        let s: String = $to_snap.into();
        $crate::assertions::make_assertion(
            "assert_matches_inline_snapshot",
            args_str,
            $crate::assertions::matches_inline_snapshot::matches_inline_snapshot(
                s,
                Some($inline_snap),
                line,
                column,
                file,
            ),
            None,
        )
    }};
}

/// Asserts if value is Ok(T).
/// panics if it is not
///
/// ```rust
/// use k9::assert_ok;
///
/// assert_ok!(Ok(2));
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($left:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!("{}", stringify!($left).red());
        $crate::assertions::make_assertion(
            "assert_ok",
            args_str,
            $crate::assertions::ok::assert_ok($left),
            None,
        )
    }};
    ($left:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_ok",
            args_str,
            $crate::assertions::ok::assert_ok($left),
            Some(&$description),
        )
    }};
}

/// Asserts if value is Err(T).
/// panics if it is not
///
/// ```rust
/// use k9::assert_err;
///
/// assert_err!(Err("Invalid path"));
/// ```
#[macro_export]
macro_rules! assert_err {
    ($left:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!("{}", stringify!($left).red());
        $crate::assertions::make_assertion(
            "assert_err",
            args_str,
            $crate::assertions::err::assert_err($left),
            None,
        )
    }};
    ($left:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($description).dimmed(),
        );
        $crate::assertions::make_assertion(
            "assert_err",
            args_str,
            $crate::assertions::err::assert_err($left),
            Some(&$description),
        )
    }};
}
