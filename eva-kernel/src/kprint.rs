#[doc(hidden)]
pub mod __private {
    use crate::portability::Impl as _;
    use core::fmt::Arguments;

    pub fn print(args: Arguments) {
        crate::portability::GlobalImpl::kprint_fmt(args);
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::kprint::__private::print(::core::format_args!($($arg)*));
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
