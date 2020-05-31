use anyhow::Result;
use k9::{assert_equal, assert_equal_r, assert_matches_snapshot};

#[test]
fn test_assert_equal() -> Result<()> {
    assert_equal!(1, 1);
    assert_equal_r!("lol", &String::from("lol"))?;

    let err = assert_equal_r!(1, 2).unwrap_err();
    assert_matches_snapshot!(err);

    assert_equal_r!(123, 123, "Expected two integers to be the same")?;
    Ok(())
}

#[test]
fn multiline_struct_equality_test() -> Result<()> {
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

    let err = assert_equal_r!(x1, x2).unwrap_err();

    assert_matches_snapshot!(err);
    Ok(())
}

#[test]
fn with_context() {
    let err = assert_equal_r!(1, 2, "Expected those two things to be equal").unwrap_err();
    assert_matches_snapshot!(err);
}
