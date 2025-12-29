#![no_std]

extern crate alloc;

/// Kernel allocator.
pub mod allocator;
/// Kernel main entry.
pub mod kmain;
/// Printing stuff.
pub mod io;
/// Portability interface.
pub mod port;
/// Scheduler and threading primitives.
pub mod rt;
/// Time related utilities.
pub mod time;

/// Internal ABI implementation.
mod c_abi;
/// Internal panic implementation.
// mod panic;
/// Internal utilities.
mod utils;
