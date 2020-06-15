/*!
see https://github.com/aaronabramov/k9 for high level description of the library
*/

pub mod assertions;
pub mod comparison_diff;
pub mod config;
pub mod string_diff;

mod utils;

// re-export things so macros have access to them
pub mod __macros__ {
    pub use colored;
}
