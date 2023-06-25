use colored::*;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub enum BuildSystem {
    /// https://buck.build/
    Buck,
    /// https://developers.facebook.com/blog/post/2021/07/01/future-of-buck/
    Buck2,
    /// https://github.com/rust-lang/cargo
    Cargo,
}

impl BuildSystem {
    pub fn is_buck(&self) -> bool {
        match self {
            Self::Buck | Self::Buck2 => true,
            Self::Cargo => false,
        }
    }
}

pub struct Config {
    /// Configurable so we can test all assertions in our own test suite without panicking.
    pub assertions_will_panic: AtomicBool,
    /// 0 === disabled
    pub terminal_width_override: AtomicUsize,
    // Snapshot update mode
    pub update_mode: bool,
    /// What build system this project is being built with
    pub build_system: BuildSystem,
    /// Whether we should always enable colored output
    pub force_enable_colors: bool,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        assertions_will_panic: AtomicBool::new(true),
        terminal_width_override: AtomicUsize::new(0),
        update_mode: is_update_mode(),
        build_system: build_system(),
        force_enable_colors: should_force_enable_colors(),
    };
}

pub fn set_panic(v: bool) {
    CONFIG.assertions_will_panic.store(v, Ordering::Relaxed)
}

pub fn should_panic() -> bool {
    CONFIG.assertions_will_panic.load(Ordering::Relaxed)
}

pub fn set_terminal_with_override(width: usize) {
    CONFIG
        .terminal_width_override
        .store(width, Ordering::Relaxed);
}

pub fn terminal_width_override() -> usize {
    CONFIG.terminal_width_override.load(Ordering::Relaxed)
}

fn build_system() -> BuildSystem {
    if std::env::var("BUCK_BUILD_ID").is_ok() {
        BuildSystem::Buck
    } else if std::env::var("BUCK2_DAEMON_UUID").is_ok() {
        BuildSystem::Buck2
    } else {
        BuildSystem::Cargo
    }
}

fn is_update_mode() -> bool {
    // If runtime ENV variable is set, it takes precedence
    std::env::var("K9_UPDATE_SNAPSHOTS").map_or(false, |_| true)
}

fn should_force_enable_colors() -> bool {
    // If we are running with buck, stdout will not be a tty and we'll lose
    // colored output. Detect that case so we can force enable colored
    // output.
    // If this is not set, fall back to the usual `colored` behavior.
    if build_system().is_buck() {
        return true;
    }

    if let Ok(force_colors) = std::env::var("K9_FORCE_COLORS") {
        match force_colors.as_str() {
            "1" => return true,
            "0" => return false,
            _ => (),
        }
    }

    false
}

pub fn update_instructions() -> colored::ColoredString {
    match CONFIG.build_system {
        BuildSystem::Buck => {
            "buck test //path/to/your/buck/target/... -- --env K9_UPDATE_SNAPSHOTS=1".yellow()
        }
        BuildSystem::Buck2 => {
            "buck2 test //path/to/your/buck/target/... -- --env K9_UPDATE_SNAPSHOTS=1".yellow()
        }
        BuildSystem::Cargo => {
            "run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots".yellow()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_building() {
        // make sure we can construct the CONFIG. this is a regression test
        // after i accidentally put a circular reference in the config functions
        // and caused everything to stall.
        let _b = format!("{}", CONFIG.force_enable_colors);
    }
}
