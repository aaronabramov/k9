use crate::assertion_message;
use k9::assert_equal;
use std::collections::{HashMap, HashSet};

#[cfg(not(feature = "custom_comparison_formatters"))]
#[test]
fn test_assert_hash_sets_large_diff_without_custom_format() {
    super::setup_test_env();

    let s1 = (1..100).collect::<HashSet<i32>>();
    let s2 = (1..101).collect::<HashSet<i32>>();

    let message = assertion_message(assert_equal!(s1, s2));

    // Since the order is randomized, we expect multiple insertions and removals
    assert!(message.matches('-').count() > 1);
    assert!(message.matches('+').count() > 1);
}

#[cfg(not(feature = "custom_comparison_formatters"))]
#[test]
fn test_assert_hash_maps_large_diff_without_custom_format() {
    super::setup_test_env();

    let s1 = (1..100).zip(1..100).collect::<HashMap<i32, i32>>();
    let s2 = (1..101).zip(1..101).collect::<HashMap<i32, i32>>();

    let message = assertion_message(assert_equal!(s1, s2));

    // Since the order is randomized, we expect multiple insertions and removals
    assert!(message.matches('-').count() > 1);
    assert!(message.matches('+').count() > 1);
}

#[cfg(feature = "custom_comparison_formatters")]
#[test]
fn test_assert_hash_sets_small_diff_with_custom_format() {
    super::setup_test_env();

    let s1 = (1..10).collect::<HashSet<i32>>();
    let s2 = (1..11).collect::<HashSet<i32>>();

    k9_stable::assert_matches_inline_snapshot!(
        assertion_message(assert_equal!(s1, s2)),
        r##"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(s1, s2);

Assertion Failure!


Expected `Left` to equal `Right`:

  {
      1,
      2,
      3,
      4,
      5,
      6,
      7,
      8,
      9,
+     10,
  }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"##
    );
}

#[cfg(feature = "custom_comparison_formatters")]
#[test]
fn test_assert_hash_maps_small_diff_with_custom_format() {
    super::setup_test_env();

    let s1 = ((1..10).zip(1..10)).collect::<HashMap<i32, i32>>();
    let s2 = ((1..11).zip(1..11)).collect::<HashMap<i32, i32>>();

    k9_stable::assert_matches_inline_snapshot!(
        assertion_message(assert_equal!(s1, s2)),
        r##"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
assert_equal!(s1, s2);

Assertion Failure!


Expected `Left` to equal `Right`:

  {
      1: 1,
      2: 2,
      3: 3,
      4: 4,
      5: 5,
      6: 6,
      7: 7,
      8: 8,
      9: 9,
+     10: 10,
  }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"##
    );
}

#[cfg(feature = "custom_comparison_formatters")]
#[test]
fn test_assert_hash_sets_large_diff_when_type_is_not_ord() {
    super::setup_test_env();

    #[derive(PartialEq, Eq, Debug, Hash)]
    struct MyInt(i32);

    let s1 = ((1..100).map(MyInt)).collect::<HashSet<MyInt>>();
    let s2 = ((1..101).map(MyInt)).collect::<HashSet<MyInt>>();

    let message = assertion_message(assert_equal!(s1, s2));

    // Since the order is randomized, we expect multiple insertions and removals
    assert!(message.matches('-').count() > 1);
    assert!(message.matches('+').count() > 1);
}

#[cfg(feature = "custom_comparison_formatters")]
#[test]
fn test_assert_hash_maps_large_diff_when_type_is_not_ord() {
    super::setup_test_env();

    #[derive(PartialEq, Eq, Debug, Hash)]
    struct MyInt(i32);

    let s1 = ((1..100).map(|i| (MyInt(i), i))).collect::<HashMap<MyInt, i32>>();
    let s2 = ((1..101).map(|i| (MyInt(i), i))).collect::<HashMap<MyInt, i32>>();

    let message = assertion_message(assert_equal!(s1, s2));

    eprintln!("message is {}", message);
    // Since the order is randomized, we expect multiple insertions and removals
    assert!(message.matches('-').count() > 1);
    assert!(message.matches('+').count() > 1);
}
