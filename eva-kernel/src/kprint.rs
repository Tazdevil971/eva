use core::ffi::CStr;
use core::fmt::Arguments;

use crate::port::{self, Impl as _};

#[unsafe(export_name = "eva_kprint_fmt")]
pub fn kprint_fmt(args: Arguments) {
    port::GlobalImpl::kprint_fmt(args);
}

pub(crate) fn kputs(str: &CStr) {
    // TODO(davide.mor): Switch to .display() once it gets stabilized
    if let Ok(str) = str.to_str() {
        kprint_fmt(format_args!("{str}"));
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::kprint::kprint_fmt(::core::format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n");
    };
    ($fmt:expr $(, $($args:tt)*)?) => {
        $crate::kprint!(::core::concat!($fmt, "\n") $(, $($args)*)?)
    };
}

#[macro_export]
macro_rules! kdbg {
    () => {
        $crate::kprintln!("[{}:{}:{}]", ::core::file!(), ::core::line!(), ::core::column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::kprintln!("[{}:{}:{}] {} = {:#?}",
                    ::core::file!(), ::core::line!(), ::core::column!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::kdbg!($val)),+,)
    };
}

pub use kdbg;
pub use kprint;
pub use kprintln;
