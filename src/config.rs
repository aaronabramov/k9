use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Config {
    assertions_will_panic: AtomicBool,
    terminal_width_override: AtomicUsize, // 0 = disabled
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        assertions_will_panic: AtomicBool::new(true),
        terminal_width_override: AtomicUsize::new(0)
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
