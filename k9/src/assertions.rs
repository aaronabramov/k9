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
pub mod matches_snapshot;
pub mod ok;
pub mod snapshot;

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

#[macro_export]
macro_rules! make_assertion {
    ($name:expr, $args_str:expr, $failure_message:expr, $description:expr,) => {{
        let assertion = $crate::assertions::make_assertion_impl(
            $name,
            $args_str,
            $failure_message,
            $description,
        );
        if let Some(assertion) = &assertion {
            if $crate::config::should_panic() {
                panic!("{}", assertion.get_failure_message());
            }
        }
        assertion
    }};
}

pub fn make_assertion_impl(
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
        Some(assertion)
    } else {
        None
    }
}

pub fn initialize_colors() {
    if crate::config::CONFIG.force_enable_colors {
        colored::control::set_override(true);
    }
}

/// Asserts that two passed arguments are equal.
/// Panics if they're not, using a pretty printed difference of
/// [Debug](std::fmt::Debug) representations of the passed arguments.
///
/// This is a drop-in replacement for [assert_eq][assert_eq] macro
///
/// ```
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
/// assert_equal!(a1, a2);
/// ```
///
/// ```should_panic
/// # use k9::assert_equal;
/// # #[derive(Debug, PartialEq)]
/// # struct A {
/// #     name: &'static str
/// # }
/// let a1 = A { name: "Kelly" };
/// let a2 = A { name: "Rob" };
///
/// // this will print the visual difference between two structs
/// assert_equal!(a1, a2);
/// ```
#[macro_export]
macro_rules! assert_equal {
    ($left:expr, $right:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );

        match  (&$left, &$right) {
            (left, right) => {
                let fail = *left != *right;
                $crate::make_assertion!(
                    "assert_equal",
                    args_str,
                    $crate::assertions::equal::assert_equal(left, right, fail),
                    None,
                )
            }
        }
    }};
    ($left:expr, $right:expr, $($description:expr),*) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let description = format!($( $description ),*);
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($( $description ),* ).dimmed(),
        );
        match  (&$left, &$right) {
            (left, right) => {
                let fail = *left != *right;
                $crate::make_assertion!(
                    "assert_equal",
                    args_str,
                    $crate::assertions::equal::assert_equal(left, right, fail),
                    Some(&description),
                )
            }
        }
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
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::make_assertion!(
            "assert_greater_than",
            args_str,
            $crate::assertions::greater_than::assert_greater_than($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
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
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::make_assertion!(
            "assert_greater_than_or_equal",
            args_str,
            $crate::assertions::greater_than_or_equal::assert_greater_than_or_equal($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
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
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::make_assertion!(
            "assert_lesser_than",
            args_str,
            $crate::assertions::lesser_than::assert_lesser_than($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
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
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
        );
        $crate::make_assertion!(
            "assert_lesser_than_or_equal",
            args_str,
            $crate::assertions::lesser_than_or_equal::assert_lesser_than_or_equal($left, $right),
            None,
        )
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}, {}",
            stringify!($left).red(),
            stringify!($right).green(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
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
        $crate::assertions::initialize_colors();
        let args_str = format!("{}, {}", stringify!($s).red(), stringify!($regex).green());
        $crate::make_assertion!(
            "assert_matches_regex",
            args_str,
            $crate::assertions::matches_regex::assert_matches_regex($s, $regex),
            None,
        )
    }};
    ($s:expr, $regex:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}, {}",
            stringify!($s).red(),
            stringify!($regex).green(),
            stringify!($description).dimmed()
        );
        $crate::make_assertion!(
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
        $crate::assertions::initialize_colors();
        let args_str = format!("{}, {}", stringify!($err).red(), stringify!($regex).green(),);
        $crate::make_assertion!(
            "assert_err_matches_regex",
            args_str,
            $crate::assertions::err_matches_regex::assert_err_matches_regex($err, $regex),
            None,
        )
    }};
    ($err:expr, $regex:expr, $context:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}, {}",
            stringify!($err).red(),
            stringify!($regex).green(),
            stringify!($context).dimmed(),
        );
        $crate::make_assertion!(
            "assert_err_matches_regex",
            args_str,
            $crate::assertions::err_matches_regex::assert_err_matches_regex($err, $regex),
            Some($context),
        )
    }};
}

/// Same as [snapshot!()](./macro.snapshot.html) macro, but it takes a string as the
/// only argument and stores the snapshot in a separate file instead of inlining
/// it in the source code of the test.
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
        $crate::assertions::initialize_colors();
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!("{}", stringify!($to_snap).red(),);
        $crate::make_assertion!(
            "assert_matches_snapshot",
            args_str,
            $crate::assertions::matches_snapshot::snap_internal($to_snap, line, column, file),
            None,
        )
    }};
    ($to_snap:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!(
            "{}, {}",
            stringify!($to_snap).red(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
            "assert_matches_snapshot",
            args_str,
            $crate::assertions::matches_snapshot::snap_internal($to_snap, line, column, file),
            Some($description),
        )
    }};
}

/// Asserts if value is Ok(T).
/// panics if it is not
///
/// ```rust
/// use k9::assert_ok;
///
/// let result: Result<_, ()> = Ok(2);
/// assert_ok!(result);
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($left:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!("{}", stringify!($left).red());
        $crate::make_assertion!(
            "assert_ok",
            args_str,
            $crate::assertions::ok::assert_ok($left),
            None,
        )
    }};
    ($left:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
            "assert_ok",
            args_str,
            $crate::assertions::ok::assert_ok($left),
            Some(&$description),
        )
    }};
}

/// Asserts if value is Err(E).
/// panics if it is not
///
/// ```rust
/// use k9::assert_err;
///
/// let result: Result<(), _> = Err("invalid path");
/// assert_err!(result);
/// ```
#[macro_export]
macro_rules! assert_err {
    ($left:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!("{}", stringify!($left).red());
        $crate::make_assertion!(
            "assert_err",
            args_str,
            $crate::assertions::err::assert_err($left),
            None,
        )
    }};
    ($left:expr, $description:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let args_str = format!(
            "{}, {}",
            stringify!($left).red(),
            stringify!($description).dimmed(),
        );
        $crate::make_assertion!(
            "assert_err",
            args_str,
            $crate::assertions::err::assert_err($left),
            Some(&$description),
        )
    }};
}

/// Serializes the first argument into a string and compares it with
/// the second argument, which is a snapshot string that was automatically generated
/// during previous test runs. Panics if the values are not equal.
///
/// If second argument is missing, assertion will always fail by prompting to
/// re-run the test in `update snapshots` mode.
///
/// If run in `update snapshots` mode, serialization of the first argument will
/// be made into a string literal and inserted into source code as the second
/// argument of this macro. (It will actually modify the file in the filesystem)
///
/// Typical workflow for this assertion is:
///
/// ```should_panic
/// // Step 1:
/// // - Take a result of some computation and pass it as a single argument to the macro
/// // - Run the test
/// // - Test will fail promting to re-run it in update mode
/// use std::collections::BTreeMap;
///
/// k9::snapshot!((1..=3).rev().enumerate().collect::<BTreeMap<_, _>>());
/// ```
///
/// ```text
/// # Step 2:
/// #   Run tests with K9_UPDATE_SNAPSHOTS=1 env variable set
/// $ K9_UPDATE_SNAPSHOTS=1 cargo test
/// ```
///
/// ```
/// // Step 3:
/// // After test run finishes and process exits successfully, the source code of the
/// //       test file will be updated with the serialized value of the first argument.
/// // All subsequent runs of this test will pass
/// use std::collections::BTreeMap;
///
/// k9::snapshot!(
///     (1..=3).rev().enumerate().collect::<BTreeMap<_, _>>(),
///     "
/// {
///     0: 3,
///     1: 2,
///     2: 1,
/// }
/// "
/// );
/// ```
///
/// ```should_panic
/// // If the logic behind first argument ever changes and affects the serialization
/// // the test will fail and print the difference between the "old" and the "new" values
/// use std::collections::BTreeMap;
///
/// k9::snapshot!(
///     /// remove `.rev()`
///     (1..=3).enumerate().collect::<BTreeMap<_, _>>(),
///     "
/// {
///     0: 3,
///     1: 2,
///     2: 1,
/// }
/// "
/// );
/// ```
///
/// The test above will now fail with the following message:
/// ```text
/// Difference:
/// {
/// -     0: 3,
/// +     0: 1,
///       1: 2,
/// -     2: 1,
/// +     2: 3,
/// }
/// ```
#[macro_export]
macro_rules! snapshot {
    ($to_snap:expr) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!("{}", stringify!($to_snap).red(),);
        $crate::make_assertion!(
            "snapshot",
            args_str,
            $crate::assertions::snapshot::snapshot($to_snap, None, line, column, file),
            None,
        )
    }};
    ($to_snap:expr, $inline_snap:literal) => {{
        use $crate::__macros__::colored::*;
        $crate::assertions::initialize_colors();
        let line = line!();
        let column = column!();
        let file = file!();
        let args_str = format!(
            "{}, {}",
            stringify!($to_snap).red(),
            stringify!($inline_snap).green(),
        );
        $crate::make_assertion!(
            "snapshot",
            args_str,
            $crate::assertions::snapshot::snapshot(
                $to_snap,
                Some($inline_snap),
                line,
                column,
                file,
            ),
            None,
        )
    }};
}
