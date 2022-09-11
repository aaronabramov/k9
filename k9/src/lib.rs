/*!
see <https://github.com/aaronabramov/k9> for high level description of the library
*/


#![cfg_attr(feature = "custom_comparison_formatters", feature(specialization))]

pub mod assertions;
pub mod config;
pub mod snapshot;
pub mod string_diff;

mod multiline_string;
mod paths;
mod snap;
mod types;
mod utils;

pub use multiline_string::MultilineString;
pub use snap::Snap;

// re-export things so macros have access to them
pub mod __macros__ {
    pub use colored;
}
