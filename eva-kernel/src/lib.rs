#![no_std]

extern crate alloc;

/// Kernel allocator.
pub mod allocator;
/// Kernel main entry.
pub mod kmain;
/// Printing stuff.
pub mod kprint;
/// Portability interface.
pub mod portability;
/// Scheduler and threading primitives.
pub mod scheduler;

/// Internal utilities.
mod utils;
