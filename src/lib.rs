/*!
see https://github.com/aaronabramov/k9 for high level description of the library
*/

#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::unnested_or_patterns)]
pub mod assertions;
pub mod config;
pub mod string_diff;

mod constants;
mod parsing;
mod paths;
mod types;
mod utils;

// re-export things so macros have access to them
pub mod __macros__ {
    pub use colored;
}
