use anyhow::Result;
use k9::{assert_equal, assert_matches_snapshot};

#[test]
fn test_assert_equal() -> Result<()> {
    k9::assertions::set_panic(false);

    assert!(assert_equal!(1, 1).is_none());
    assert!(assert_equal!("lol", &String::from("lol")).is_none());

    let failure_message = assert_equal!(1, 2).expect("must fail");
    assert_matches_snapshot!(failure_message);

    assert!(assert_equal!(123, 123, "Expected two integers to be the same").is_none());
    Ok(())
}

#[test]
fn multiline_struct_equality_test() -> Result<()> {
    k9::assertions::set_panic(false);
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

    let err = assert_equal!(x1, x2).expect("must fail");

    assert_matches_snapshot!(err);
    Ok(())
}

#[test]
fn with_context() {
    k9::assertions::set_panic(false);
    let err = assert_equal!(1, 2, "Expected those two things to be equal").expect("must fail");
    assert_matches_snapshot!(err);
}
