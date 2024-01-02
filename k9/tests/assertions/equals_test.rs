use crate::assertion_message;
use k9::assert_equal;
use k9::MultilineString;

#[test]
fn test_assert_equal() {
    super::setup_test_env();

    assert!(assert_equal!(1, 1).is_none());
    assert!(assert_equal!(vec![1, 2, 3], [1, 2, 3]).is_none());
    assert!(assert_equal!(1, 1, "some description").is_none());
    assert!(assert_equal!(1, 1, "some formatted description {} {:?}", 1, "dogs").is_none());
    assert!(assert_equal!("lol", &String::from("lol")).is_none());

    k9_stable::snapshot!(
        assertion_message(assert_equal!(2, 9)),
        "

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(2, 9);

Assertion Failure!


Expected `Left` to equal `Right`:

- 2
+ 9

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

"
    );
    k9_stable::snapshot!("hello", "hello");
    assert!(assert_equal!(123, 123, "Expected two integers to be the same").is_none());
}

#[test]
fn type_inference() {
    assert_eq!(1, "1".parse().unwrap());
    // Must infer the type for `parse` without asking to specify it
    assert!(assert_equal!(1, "1".parse().unwrap()).is_none());
}

#[test]
fn multiline_struct_equality_test() {
    super::setup_test_env();
    #[derive(PartialEq, Debug)]
    struct X {
        a: String,
        b: i32,
        c: (String, u32),
        d: Option<()>,
    }

    let x1 = X {
        a: "test".to_string(),
        b: 4,
        c: ("test2".to_string(), 4),
        d: Some(()),
    };

    let x2 = X {
        a: "test".to_string(),
        b: 4,
        c: ("test2".to_string(), 9),
        d: None,
    };

    let err = assert_equal!(x1, x2)
        .expect("must fail")
        .get_failure_message();

    k9_stable::snapshot!(
        crate::strip_ansi(&err),
        r#"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(x1, x2);

Assertion Failure!


Expected `Left` to equal `Right`:

  X {
      a: "test",
      b: 4,
      c: (
          "test2",
-         4,
+         9,
      ),
-     d: Some(
-         (),
-     ),
+     d: None,
  }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

"# // trailing comment after inline snap
    );

    // test colors
    k9_stable::snapshot!(
        &err,
        r#"

\u{1b}[2m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\u{1b}[0m
\u{1b}[33massert_equal!\u{1b}[0m(\u{1b}[31mx1\u{1b}[0m, \u{1b}[32mx2\u{1b}[0m);

Assertion Failure!


Expected `\u{1b}[31mLeft\u{1b}[0m` to equal `\u{1b}[32mRight\u{1b}[0m`:

  \u{1b}[2mX {\u{1b}[0m
  \u{1b}[2m    a: "test",\u{1b}[0m
  \u{1b}[2m    b: 4,\u{1b}[0m
  \u{1b}[2m    c: (\u{1b}[0m
  \u{1b}[2m        "test2",\u{1b}[0m
\u{1b}[31m-\u{1b}[0m \u{1b}[31m        4,\u{1b}[0m
\u{1b}[32m+\u{1b}[0m \u{1b}[32m        9,\u{1b}[0m
  \u{1b}[2m    ),\u{1b}[0m
\u{1b}[31m-\u{1b}[0m \u{1b}[31m    d: Some(\u{1b}[0m
\u{1b}[31m-\u{1b}[0m \u{1b}[31m        (),\u{1b}[0m
\u{1b}[31m-\u{1b}[0m \u{1b}[31m    ),\u{1b}[0m
\u{1b}[32m+\u{1b}[0m \u{1b}[32m    d: None,\u{1b}[0m
  \u{1b}[2m}\u{1b}[0m

\u{1b}[2m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\u{1b}[0m

"#
    );
}

#[test]
fn with_context() {
    let a = vec![1, 2, 3];
    let b = [1, 2, 99];

    super::setup_test_env();
    k9_stable::snapshot!(
        assertion_message(assert_equal!(a, b, "Expected {:?} to equal {:?}", a, b)),
        r#"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(a, b, "Expected {:?} to equal {:?}", a, b);

Expected [1, 2, 3] to equal [1, 2, 99]


Expected `Left` to equal `Right`:

  [
      1,
      2,
-     3,
+     99,
  ]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

"#
    );
}

#[test]
fn multiline_string() {
    super::setup_test_env();

    let x1 = "A\nB\nC";
    let x2 = "A\nD\nC".to_string();

    let err = assert_equal!(MultilineString::new(x1), MultilineString(x2))
        .expect("must fail")
        .get_failure_message();

    k9_stable::snapshot!(
        crate::strip_ansi(&err),
        "

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(MultilineString::new(x1), MultilineString(x2));

Assertion Failure!


Expected `Left` to equal `Right`:

  A
- B
+ D
  C

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

"
    );
}
