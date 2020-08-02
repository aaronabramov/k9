use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Config {
    /// Configurable so we can test all assertions in our own test suite without panicing.
    pub assertions_will_panic: AtomicBool,
    /// 0 === disabled
    pub terminal_width_override: AtomicUsize,
    // Snapshot update mode
    pub update_mode: bool,
    // Snapshot expand mode
    pub expand_mode: bool,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        assertions_will_panic: AtomicBool::new(true),
        terminal_width_override: AtomicUsize::new(0),
        update_mode: is_update_mode(),
        expand_mode: is_expand_mode(),
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

fn is_update_mode() -> bool {
    // If runtime ENV variable is set, it takes precedence
    let runtime_var = std::env::var("K9_UPDATE_SNAPSHOTS").map_or(false, |_| true);
    let buck_build_id_present = std::env::var("BUCK_BUILD_ID").is_ok();

    if !runtime_var && buck_build_id_present {
        // If not, we'll also check compile time variable. This is going to be the case with `buck`
        // when env variables are passed to `rustc` but not to the actual binary (when running `buck test ...`)
        //
        // NOTE: using compile time vars is a bit sketchy, because technically you can compile the test suite
        // once and re-run the compiled version multiple times in scenarious where you don't want to update
        if option_env!("K9_UPDATE_SNAPSHOTS").is_some() {
            return true;
        }
    }

    runtime_var
}

fn is_expand_mode() -> bool {
    // If runtime ENV variable is set, it takes precedence
    let runtime_var = std::env::var("K9_EXPAND").map_or(false, |_| true);
    let buck_build_id_present = std::env::var("BUCK_BUILD_ID").is_ok();

    if !runtime_var && buck_build_id_present {
        // If not, we'll also check compile time variable. This is going to be the case with `buck`
        // when env variables are passed to `rustc` but not to the actual binary (when running `buck test ...`)
        //
        // NOTE: using compile time vars is a bit sketchy, because technically you can compile the test suite
        // once and re-run the compiled version multiple times in scenarious where you don't want to update
        if option_env!("K9_EXPAND").is_some() {
            return true;
        }
    }

    runtime_var
}
