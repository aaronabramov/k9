use k9::*;
use std::collections::BTreeSet;

#[test]
fn experimental_snapshot() {
    snapshot!(
        "Hello\nWorld",
        "
Hello
World
"
    );

    let map: BTreeSet<u8> = vec![1, 2, 3, 0, 5, 8].into_iter().collect();

    snapshot!(map);

    snapshot!("uses single quotes for literal");

    snapshot!(r#"should"not"use"quotes"in"literal"#);

    snapshot!(r####"should use more than two "## for escaping"####);
}
