/// [assert_equal!](crate::assert_equal) takes a [std::fmt::Debug](std::fmt::Debug) trait object as an argument
/// which doesn't work well with multiline strings, since newline characters will be displayed as `\n`
/// For example this string:
/// ```
/// let s = "A
/// B
/// C";
/// ```
///
/// will be printed as a single line `"A\nB\nC"`, which is not useful
/// for multiline comparison diff
///
/// Using this struct makes the original string serialize into a proper multiline string
/// which will produce a nice line by line difference comparison when used together with
/// [assert_equal!](crate::assert_equal).
///
/// ```should_panic
/// use k9::{MultilineString, assert_equal};
///
/// let s1 = "A\nB\nC".to_string();
/// let s2 = "A\nD\nC";
/// assert_equal!(MultilineString(s1), MultilineString::new(s2));
/// ```
#[derive(Eq, PartialEq)]
pub struct MultilineString(pub String);

impl MultilineString {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}
/// Make diff to display string as multi-line string
impl std::fmt::Debug for MultilineString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
