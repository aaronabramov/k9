# K9 - Rust Testing Library

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
![Rust CI](https://github.com/aaronabramov/k9/workflows/Rust%20CI/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/k9.svg
[crates-url]: https://crates.io/crates/k9
[docs-badge]: https://docs.rs/k9/badge.svg
[docs-url]: https://docs.rs/k9

![k9_header](https://user-images.githubusercontent.com/940133/98482607-2140c200-21c8-11eb-84f0-af488323a49a.png)

## Snapshot testing + better assertions

### Available test macros

- `snapshot`
- `assert_equal`
- `assert_greater_than`
- `assert_greater_than_or_equal`
- `assert_lesser_than`
- `assert_lesser_than_or_equal`
- `assert_matches_regex`
- `assert_err_matches_regex`
- `assert_matches_snapshot`
- `assert_matches_inline_snapshot`
- `assert_ok`
- `assert_err`

See [https://docs.rs/k9](https://docs.rs/k9) for API documentation


## `snapshot!()` macro

Snapshot macro provides the functionality to capture the `Debug` representation
of any value and make sure it does not change over time. 

If it does change, the test will fail and print the difference between "old" and
"new" values.

If the change is expected and valid, running `cargo test` with
`K9_UPDATE_SNAPSHOTS=1` env variable set will automatically take the new value
and insert it into the test source code file as a second argument, after which
all subsequent test runs should start passing again.

![inline_snapshot_demo](https://user-images.githubusercontent.com/940133/102737400-ed030a00-430c-11eb-90ac-66d4d24c9acd.gif)


## `assert_equal!()` macro

Rust already provides a good built-in test runner and a set of assertion macros like `assert!` and `assert_eq!`.
They work great for for quick unit tests, but once the codebase and test suites grows to a certain point it gets
harder and harder to test things and keep tests readable.


For example, when testing that two structs are equal using `assert_eq!` macro the output does not provide a lot of help
in understanding why exactly this test failed.

```rust

#[derive(PartialEq, Debug)]
struct Person {
    name: &'static str,
    age: usize,
}

#[test]
fn test_eq() {
    let person1 = Person {name: "Bob", age: 12 };
    let person2 = Person {name: "Alice", age: 20 };
    assert_eq!(person1, person2, "These two must be the same person!");
}
```

All we get is usually a wall of wite text collapsed into a single line and you have to find the difference between two structs yourself. Which becomes very time consuming when structs are 10+ fields.

```
---- eq::test_eq stdout ----
thread 'eq::test_eq' panicked at 'assertion failed: `(left == right)`
  left: `Person { name: "Bob", age: 12 }`,
 right: `Person { name: "Alice", age: 20 }`: These two must be the same person!', src/eq.rs:13:5
```

using `k9::assert_equal` macro improves this output and prints the difference between two structs:

```rust
use k9::assert_equal;
assert_equal!(person1, person2, "These two must be the same person!");
```

![assert_equal_example](https://user-images.githubusercontent.com/940133/84608052-35310380-ae76-11ea-97fe-751ee76a7735.png)

# Non-equality based assertions

Testing equality is very simple and can definitely work for most of the cases, but one of the disadvantages of only using `assert!` and `assert_eq!` is the error messages when something fails.
For example, if you're testing that your code produces valid URL

```rust
let url = generate_some_url();
assert_eq!(URL_REGEX.is_match(url), true);
```

What you get is

```
thread 'eq::test_eq3' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`', src/eq.rs:19:5
```

Which doesn't help much. Especially, if you're new to the code base, seeing things like `expected 'true' but got 'false'` will make you go and look at the code before you even know what the problem can be, which can be very time consuming.

What we probably want to see is:

![assert_matches_regex_example](https://user-images.githubusercontent.com/940133/84608051-35310380-ae76-11ea-87c8-c7c8b9ee3903.png)

Which gives us enough context on what the problem is and how to fix it without for us having to go and run/debug the test first.
