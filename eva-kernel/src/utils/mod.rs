#![allow(dead_code)]

pub mod assert;
pub mod bitset;
pub mod linked_list;
pub mod slot_map;
pub mod unchecked_ref;

pub use scopeguard;

macro_rules! assert_send {
    ($ty:ty) => {
        const _: () = {
            const fn assert_send<T: Send>() {}
            assert_send::<$ty>()
        };
    };
}

macro_rules! assert_sync {
    ($ty:ty) => {
        const _: () = {
            const fn assert_sync<T: Sync>() {}
            assert_sync::<$ty>()
        };
    };
}

pub(crate) use assert_send;
pub(crate) use assert_sync;
