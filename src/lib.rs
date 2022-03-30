//! Offline, deterministic building blocks for personal progress tracking.
#![forbid(unsafe_code)]

pub mod cli;
pub mod date;
pub mod goal;
pub mod habit;
pub mod store;
pub mod visual;

pub type Result<T> = std::result::Result<T, String>;
