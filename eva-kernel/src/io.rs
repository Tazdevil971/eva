use core::ffi::CStr;
use core::fmt::{self, Arguments};

use crate::port::{self, Impl as _};

#[unsafe(export_name = "eva_io_kwrite")]
pub fn kwrite(data: &[u8]) -> usize {
    port::GlobalImpl::kwrite(data)
}

#[unsafe(export_name = "eva_io_kread")]
pub fn kread(data: &mut [u8]) -> usize {
    port::GlobalImpl::kread(data)
}

pub fn kprint_fmt(args: Arguments) {
    struct KIo;

    impl fmt::Write for KIo {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            kwrite(s.as_bytes());
            Ok(())
        }
    }

    let _ = fmt::Write::write_fmt(&mut KIo, args);
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
        $crate::io::kprint_fmt(::core::format_args!($($arg)*));
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
