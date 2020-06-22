use crate::assert_matches_inline_snapshot;
use anyhow::Result;
use k9::{assert_equal, assert_matches_snapshot};

#[test]
fn test_assert_equal() -> Result<()> {
    super::setup_test_env();

    assert!(assert_equal!(1, 1).is_none());
    assert!(assert_equal!("lol", &String::from("lol")).is_none());

    let assertion = assert_equal!(1, 2).expect("must fail");
    assert_matches_inline_snapshot!(assertion.get_failure_message(),       "
[2m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[33massert_equal![0m([31m1[0m, [32m2[0m);

Assertion Failure!


Expected `[31mLeft[0m` to equal `[32mRight[0m`:

[31m-[0m [31m1[0m
[32m+[0m [32m2[0m

[2m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
");

    assert!(assert_equal!(123, 123, "Expected two integers to be the same").is_none());
    Ok(())
}

#[test]
fn multiline_struct_equality_test() -> Result<()> {
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

    assert_matches_inline_snapshot!(
        crate::strip_ansi(&err),
        "
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(x1, x2);

Assertion Failure!


Expected `Left` to equal `Right`:

  X {
      a: \"test\",
      b: 4,
      c: (
          \"test2\",
-         4,
+         9,
      ),
-     d: Some(
-         (),
-     ),
+     d: None,
  }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"
    );

    assert_matches_inline_snapshot!(&err,   "
[2m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[33massert_equal![0m([31mx1[0m, [32mx2[0m);

Assertion Failure!


Expected `[31mLeft[0m` to equal `[32mRight[0m`:

  [2mX {[0m
  [2m    a: \"test\",[0m
  [2m    b: 4,[0m
  [2m    c: ([0m
  [2m        \"test2\",[0m
[31m-[0m [31m        4,[0m
[32m+[0m [32m        9,[0m
  [2m    ),[0m
[31m-[0m [31m    d: Some([0m
[31m-[0m [31m        (),[0m
[31m-[0m [31m    ),[0m
[32m+[0m [32m    d: None,[0m
  [2m}[0m

[2m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
");
    Ok(())
}

#[test]
fn with_context() {
    super::setup_test_env();
    let err = assert_equal!(1, 2, "Expected those two things to be equal")
        .expect("must fail")
        .get_failure_message();
    assert_matches_snapshot!(err).map(|a| a.panic());
}
