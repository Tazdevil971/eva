#[macro_export]
macro_rules! assert_send {
    ($ty:ty) => {
        const _: () = {
            const fn assert_send<T: ::core::marker::Send>() {}
            assert_send::<$ty>()
        };
    };
}

#[macro_export]
macro_rules! assert_sync {
    ($ty:ty) => {
        const _: () = {
            const fn assert_sync<T: ::core::marker::Sync>() {}
            assert_sync::<$ty>()
        };
    };
}

#[macro_export]
macro_rules! assert_no_uninit {
    ($ty:ty) => {
        const _: () = {
            const fn assert_no_uninit<T: $crate::__priv::bytemuck::NoUninit>() {}
            assert_no_uninit::<$ty>()
        };
    };
}

#[macro_export]
macro_rules! assert_any_bit_pattern {
    ($ty:ty) => {
        const _: () = {
            const fn assert_any_bit_pattern<T: $crate::__priv::bytemuck::AnyBitPattern>() {}
            assert_any_bit_pattern::<$ty>()
        };
    };
}

#[macro_export]
macro_rules! assert_zeroable {
    ($ty:ty) => {
        const _: () = {
            const fn assert_zeroable<T: $crate::__priv::bytemuck::Zeroable>() {}
            assert_zeroable::<$ty>()
        };
    };
}

#[macro_export]
macro_rules! assert_cast_ref_compatible {
    ($from:ty => $to:ty) => {
        const _: () = {
            // Sadly this right now is too strict, as it prohibits UnsafeCell!
            // $crate::assert_no_uninit!($from);
            // $crate::assert_any_bit_pattern!($to);

            assert!(::core::mem::size_of::<$from>() >= ::core::mem::size_of::<$to>());
            assert!(::core::mem::align_of::<$from>() >= ::core::mem::align_of::<$to>());
        };
    };
}
