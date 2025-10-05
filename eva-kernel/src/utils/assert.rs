macro_rules! harden_assert {
    ($($arg:tt)*) => {
        ::core::debug_assert!($($arg)*);
    };
}

pub(crate) use harden_assert;
