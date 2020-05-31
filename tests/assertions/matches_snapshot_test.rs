use k9::assert_matches_snapshot;
use std::collections::BTreeMap;

#[test]
fn basic_test() {
    let map = vec![("a", 1), ("b", 2), ("c", 3)]
        .into_iter()
        .collect::<BTreeMap<_, _>>();

    assert_matches_snapshot!(format!("{:#?}", map));
}
