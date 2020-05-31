/*!
Rust testing library that provides a set of assertions that are similar to built in `assert` and `assert_eq`
but provide more context about the failure and use ANSI terminal colors to format the error messages.

It also includes `assert_matches_snapshot!` macro. When first run with `K9_UPDATE_SNAPSHOTS=1` it will
save the contents of the passed argument into a `__k9_snapshots__/my_test_file/my_test.snap` file, and
for every next run it will compare the passed value with the existing snapshot and fail if the values
are different.

```rust
use k9::{assert_equal, assert_matches_regex, assert_err_matches_regex, assert_matches_snapshot};

assert_equal!(1, 1);
assert_equal!("one", "one");


#[derive(Debug, PartialEq)]
struct A {
    name: &'static str,
    age: u32,
}

let a1 = A { name: "Susan", age: 44 };
let a2 = A { name: "Susan", age: 22 + 22 };

assert_equal!(&a1, &a2);


assert_matches_regex!(a1.name, "Su\\w{3}");

assert_matches_snapshot!(format!("{:#?}", a1));

let result: Result<(), &str> = Err("http request fail. code 123");
assert_err_matches_regex!(result, "code 123");

```
*/
pub mod assertion_error;
pub mod assertions;
pub mod string_diff;

mod utils;

pub use assertion_error::AssertionError;
pub type Result<T> = core::result::Result<T, AssertionError>;
