use crate::assertion_error::AssertionError;
use crate::string_diff::colored_diff;
use crate::utils::add_linebreaks;
use crate::Result;
use colored::*;
use std::fmt::Debug;

/// Asserts that two passed arguments are equal.
/// panics if they are note
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
        $crate::assert_equal_r!($left, $right).unwrap();
    }};
    ($left:expr, $right:expr, $context:expr) => {{
        $crate::assert_equal_r!($left, $right, $context).unwrap();
    }};
}

/// Same as `assert_equal!` but returns an assertion `Result` instead
#[macro_export]
macro_rules! assert_equal_r {
    ($left:expr, $right:expr) => {{
        $crate::assertions::equal::assert_equal_impl($left, $right, None)
    }};
    ($left:expr, $right:expr, $context:expr) => {{
        $crate::assertions::equal::assert_equal_impl($left, $right, Some($context))
    }};
}

pub fn assert_equal_impl<T: Debug + PartialEq>(
    left: T,
    right: T,
    context: Option<&str>,
) -> Result<()> {
    if left != right {
        let diff_string = colored_diff(&format!("{:#?}", &left), &format!("{:#?}", &right))
            .unwrap_or_else(|| "no visual difference between values".to_string());

        let message = format!(
            "
{context}{assertion_desc}({left_desc}, {right_desc})

Expected `{left_desc}` to equal `{right_desc}`:
{diff_string}",
            context = context.map(add_linebreaks).unwrap_or("".into()),
            assertion_desc = "assert_equal!".dimmed(),
            left_desc = "Left".red(),
            right_desc = "Right".green(),
            diff_string = &diff_string
        );

        Err(AssertionError::new(message))
    } else {
        Ok(())
    }
}
