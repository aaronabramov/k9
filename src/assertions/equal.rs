use crate::string_diff::colored_diff;
use colored::*;

/// Trait used to turn types into a string we can then diff to show pretty
/// assertions. This allows customizations for certain types to make the
/// generated strings more human readable.
pub trait FormattableForComparison {
    fn format(&self) -> String;
}

#[cfg(not(feature = "custom_comparison_formatters"))]
mod specialization {
    use super::FormattableForComparison;
    use std::fmt::Debug;

    impl<T> FormattableForComparison for T
    where
        T: Debug,
    {
        fn format(&self) -> String {
            format!("{:#?}", self)
        }
    }
}

#[cfg(feature = "custom_comparison_formatters")]
mod specialization {
    use super::FormattableForComparison;
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
    use std::fmt::Debug;
    use std::iter::FromIterator;

    impl<T> FormattableForComparison for T
    where
        T: Debug,
    {
        default fn format(&self) -> String {
            format!("{:#?}", self)
        }
    }

    /// Specialize for HashMaps if the key is `Ord` - doing a diff with
    /// sorted keys will highlight only the values that have changed
    /// rather than showing ordering differences.
    impl<K, V> FormattableForComparison for HashMap<K, V>
    where
        K: Debug + Ord,
        V: Debug,
    {
        fn format(&self) -> String {
            let sorted = BTreeMap::from_iter(self.iter());
            format!("{:#?}", sorted)
        }
    }

    /// Specialize for HashSets if the key is `Ord` - doing a diff with
    /// sorted keys will highlight only the values that have changed
    /// rather than showing ordering differences.
    impl<T, S> FormattableForComparison for HashSet<T, S>
    where
        T: Debug + Ord,
    {
        fn format(&self) -> String {
            let sorted = BTreeSet::from_iter(self.iter());
            format!("{:#?}", sorted)
        }
    }
}

pub fn assert_equal<
    T1: FormattableForComparison + PartialEq,
    T2: FormattableForComparison + PartialEq,
>(
    left: T1,
    right: T2,
    fail: bool,
) -> Option<String> {
    if fail {
        let diff_string = colored_diff(&left.format(), &right.format())
            .unwrap_or_else(|| "no visual difference between values".to_string());

        let message = format!(
            "
Expected `{left_desc}` to equal `{right_desc}`:
{diff_string}",
            left_desc = "Left".red(),
            right_desc = "Right".green(),
            diff_string = &diff_string
        );

        Some(message)
    } else {
        None
    }
}
