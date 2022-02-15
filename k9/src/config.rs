use colored::*;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Config {
    /// Configurable so we can test all assertions in our own test suite without panicing.
    pub assertions_will_panic: AtomicBool,
    /// 0 === disabled
    pub terminal_width_override: AtomicUsize,
    // Snapshot update mode
    pub update_mode: bool,
    /// Whether this binary is built with buck
    pub built_with_buck: bool,
    /// Whether we should always enable colored output
    pub force_enable_colors: bool,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        assertions_will_panic: AtomicBool::new(true),
        terminal_width_override: AtomicUsize::new(0),
        update_mode: is_update_mode(),
        built_with_buck: is_buck_build(),
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

fn is_buck_build() -> bool {
    std::env::var("BUCK_BUILD_ID").is_ok()
}

fn is_update_mode() -> bool {
    // If runtime ENV variable is set, it takes precedence
    let runtime_var = std::env::var("K9_UPDATE_SNAPSHOTS").map_or(false, |_| true);

    if !runtime_var && is_buck_build() {
        // If not, we'll also check compile time variable. This is going to be the case with `buck`
        // when env variables are passed to `rustc` but not to the actual binary (when running `buck test ...`)
        //
        // NOTE: using compile time vars is a bit sketchy, because technically you can compile the test suite
        // once and re-run the compiled version multiple times in scenarios where you don't want to update
        if option_env!("K9_UPDATE_SNAPSHOTS").is_some() {
            return true;
        }
    }

    runtime_var
}

fn should_force_enable_colors() -> bool {
    // If we are running with buck, stdout will not be a tty and we'll lose
    // colored output. Detect that case so we can force enable colored
    // output.
    // If this is not set, fall back to the usual `colored` behavior.
    is_buck_build()
}

pub fn update_instructions() -> colored::ColoredString {
    if is_buck_build() {
        // This only works with FB internal infra + buck, but i don't think anyone in the real world
        // would use buck to build anything so it's probably fine to hardcode it here. ¯\_(ツ)_/¯
        "buck test //path/to/your/buck/target/... -- --env K9_UPDATE_SNAPSHOTS=1".yellow()
    } else {
        "run with `K9_UPDATE_SNAPSHOTS=1` to update/create snapshots".yellow()
    }
}
