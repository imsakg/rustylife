//! Offline, deterministic building blocks for personal progress tracking.
#![forbid(unsafe_code)]

pub mod date;
pub mod habit;

pub type Result<T> = std::result::Result<T, String>;
