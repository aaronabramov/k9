use k9::*;

#[test]
fn inline_snapshot() {
    assert_matches_inline_snapshot!(format!("{}", std::f64::consts::E), r##"2.718281828459045"##);
    k9::assert_matches_inline_snapshot!(
        format!("{}", std::f64::consts::E),
        r##"2.718281828459045"##
    );
}

#[test]
fn passing() {}
