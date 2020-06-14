use crate::utils::add_linebreaks;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod equal;
pub mod err_matches_regex;
pub mod matches_regex;
pub mod matches_snapshot;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref ASSERTIONS_WILL_PANIC: AtomicBool = { AtomicBool::new(true) };
}

pub fn set_panic(v: bool) {
    ASSERTIONS_WILL_PANIC.store(v, Ordering::Relaxed)
}

pub fn should_panic() -> bool {
    ASSERTIONS_WILL_PANIC.load(Ordering::Relaxed)
}

#[derive(Debug)]
pub struct Assertion {
    // Description of what's being asserted to provide a bit more context in the error mesasge
    pub description: Option<String>,
    pub failure_message: Option<String>,
}

impl Assertion {
    pub fn assert(&self) -> Option<String> {
        self.failure_message.as_ref().map(|failure_message| {
            let message = format!(
                "{description}{failure_message}",
                description = self
                    .description
                    .as_ref()
                    .map(|s| add_linebreaks(&s))
                    .unwrap_or("".into()),
                failure_message = failure_message,
            );

            if should_panic() {
                panic!(message);
            }
            message
        })
    }
}

#[macro_export]
macro_rules! make_assertion {
    ($message:expr, $description:expr) => {{
        if let Some(message) = $message {
            ($crate::assertions::Assertion {
                description: Some($description.into()),
                failure_message: Some(message),
            })
            .assert()
        } else {
            None
        }
    }};
    ($message:expr) => {{
        if let Some(message) = $message {
            ($crate::assertions::Assertion {
                description: None,
                failure_message: Some(message),
            })
            .assert()
        } else {
            None
        }
    }};
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
        $crate::make_assertion!($crate::assertions::equal::assert_equal($left, $right))
    }};
    ($left:expr, $right:expr, $description:expr) => {{
        $crate::make_assertion!(
            $crate::assertions::equal::assert_equal($left, $right),
            $description
        )
    }};
}
