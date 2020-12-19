use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, Mutex};

/// String with concurrent access. Allows mutation without &mut reference to itself.
/// It makes passing it to different parts of tests easier when performance is not important.
/// Useful for accumulating output from a system under test and later using it with
/// [assert_matches_inline_snapshot](crate::assert_matches_inline_snapshot).
///
/// ```rust
/// let snap = k9::Snap::new();
/// let closure_captured_snap_by_ref = || {
///    snap.push("a");
/// };
///
/// closure_captured_snap_by_ref();
/// closure_captured_snap_by_ref();
/// closure_captured_snap_by_ref();
//
/// k9::snapshot!(snap.to_string(), "aaa");
/// ```
#[derive(Clone)]
pub struct Snap(Arc<Mutex<String>>);

impl Snap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(String::new())))
    }

    pub fn from<S: Into<String>>(s: S) -> Self {
        Self(Arc::new(Mutex::new(s.into())))
    }

    /// Push a substring
    pub fn push<S: Into<String>>(&self, s: S) {
        let mut content = self.0.lock().unwrap();
        let s = s.into();
        content.push_str(&s);
    }

    /// Push a substring and add a newline `\n` at the end
    pub fn pushln<S: Into<String>>(&self, s: S) {
        self.push(s);
        self.push("\n");
    }
}

impl From<&Snap> for String {
    fn from(snap: &Snap) -> Self {
        snap.0.lock().unwrap().clone()
    }
}

impl Display for Snap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.lock().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap() {
        let snap = Snap::from("\n");

        let closure_captured_snap_by_ref = || {
            snap.pushln("new line");
        };

        closure_captured_snap_by_ref();
        closure_captured_snap_by_ref();
        closure_captured_snap_by_ref();

        crate::assert_equal!(
            snap.to_string(),
            "
new line
new line
new line
"
            .to_string()
        );
    }
}
