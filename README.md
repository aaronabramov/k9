# K9 - Rust Testing Library

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
![Rust CI](https://github.com/aaronabramov/k9/workflows/Rust%20CI/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/k9.svg
[crates-url]: https://crates.io/crates/k9
[docs-badge]: https://docs.rs/k9/badge.svg
[docs-url]: https://docs.rs/k9

![k9_header](https://user-images.githubusercontent.com/940133/83342567-ae492c00-a2b6-11ea-8ccd-bb3e67df21f9.jpg)

## Rust testing library that provides pretty assertion macros and snapshot testing.

Rust already provides a good built-in test runner and a set of assertion macros like `assert!` and `assert_eq!`.
They work great for for quick unit tests, but once the codebase and test suites grows to a certain point it gets 
harder and harder to test things and keep tests readable.

This crate is aiming to solve two issues:
- Provide better output when a test fails
- Provide a set of assertion macros for non trivial testing use cases.

For example, when testing that two srtucts are equal using `assert_eq!` macro the output does not provide a lot of help
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

All we get is usually a wall of wite text collapsed into a single line and you have to find the difference between two stucts yourself. Which becomes very time consuming when structs are 10+ fields.

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

# Snapshot testing

When testing high level APIs some data structures that are returned by functions can become pretty large and manually testing every single field can become impossible
```rust
let response = make_api_call();
assert_eq!(response.field_a, "some value");
assert_eq!(response.field_b, "some value");
assert_eq!(response.field_c, "some value");
assert_eq!(response.field_d, "some value");
// 100 more response fields
```
This is very unmaintainable and even small refactoring will end up in changing a lot of tests manually.

Snapshot tests provide an automated way of testing that these large structs don't change their values over time, or, if they change, pinpoint 
to the exact difference between the value before code changes and after code changes.

Snapshot testing involves multiple stages.
First thing you need to do when creating a new test is using `assert_matches_snapshot!` macro that expects a string argument
```rust
assert_matches_snapshot!(format!("{:#?}", response));
```

First time you run the tests it will fail, saynig that there is no existing snapshot found in the project.

To create a snapshot you need to run tests with `K9_UPDATE_SNAPSHOTS=1` environment variable
```sh
K9_UPDATE_SNAPSHOTS=1 cargo test
```

this will create a snapshot file for this test shat will hold the context of the passed string:
```
src/
├── __k9_snapshots__
│   └── my_snapshot_test
│       └── snapshot_test.snap
```

These files are expected to be checked into the repository and go through a code review.

Then, after modifying your code (and potentially the value of returned `response` object) you can run the tetsts again. It will 
serialized a new `response` object into string and compare it with the previous (stored in the repo) snapshot while highliting the difference

![assert_matches_snapshot_example](https://user-images.githubusercontent.com/940133/84608050-34986d00-ae76-11ea-8fe1-4940ee5ad4ad.png)

During this step you can examine whether the changes to the response object are intended or whether it's a newly introduced bug that needs to be fixed.

Once the snapshot looks correct, it can be updated again using `K9_UPDATE_SNAPSHOTS=1` variable.

