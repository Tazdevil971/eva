use core::alloc::Layout;
use core::fmt;
use core::time::Duration;

/// Trait for implementing various platform specific functionalities.
pub trait Impl {
    /// Return the layout needed for the switchctx block.
    fn switchctx_layout() -> Layout;

    /// Initialize both the switchctx and stack of a thread.
    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
    );

    /// Set the global switchctx pointer.
    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8);

    /// Perform a yield operation.
    fn yield_now();

    /// Print a string to the kernel log output.
    fn kprint_fmt(args: fmt::Arguments);

    /// Retrieve time from start of execution.
    fn get_time() -> Duration;
}

/// Set the global portability implementation.
#[macro_export]
macro_rules! set_global_portability_impl {
    ($t:ty) => {
        const _: () = {
            #[unsafe(no_mangle)]
            unsafe fn __eva_kernel_1_0_switchctx_layout() -> core::alloc::Layout {
                unsafe { <$t as $crate::portability::Impl>::switchctx_layout() }
            }

            #[unsafe(no_mangle)]
            unsafe fn __eva_kernel_1_0_init_switchctx(
                switchctx_ptr: *mut u8,
                stack_ptr: *mut u8,
                stack_size: usize,
                entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
                arg1: *mut (),
                arg2: *mut (),
            ) {
                unsafe {
                    <$t as $crate::portability::Impl>::init_switchctx(
                        switchctx_ptr,
                        stack_ptr,
                        stack_size,
                        entry,
                        arg1,
                        arg2,
                    )
                }
            }

            #[unsafe(no_mangle)]
            unsafe fn __eva_kernel_1_0_set_global_switchctx(switchctx_ptr: *mut u8) {
                unsafe { <$t as $crate::portability::Impl>::set_global_switchctx(switchctx_ptr) }
            }

            #[unsafe(no_mangle)]
            fn __eva_kernel_1_0_yield_now() {
                <$t as $crate::portability::Impl>::yield_now()
            }

            #[unsafe(no_mangle)]
            fn __eva_kernel_1_0_kprint_fmt(args: ::core::fmt::Arguments) {
                <$t as $crate::portability::Impl>::kprint_fmt(args)
            }

            #[unsafe(no_mangle)]
            fn __eva_kernel_1_0_get_time() -> ::core::time::Duration {
                <$t as $crate::portability::Impl>::get_time()
            }
        };
    };
}

pub struct GlobalImpl;

impl Impl for GlobalImpl {
    fn switchctx_layout() -> Layout {
        unsafe extern "Rust" {
            unsafe fn __eva_kernel_1_0_switchctx_layout() -> Layout;
        }

        unsafe { __eva_kernel_1_0_switchctx_layout() }
    }

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        stack_size: usize,
        entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
    ) {
        unsafe extern "Rust" {
            unsafe fn __eva_kernel_1_0_init_switchctx(
                switchctx_ptr: *mut u8,
                stack_ptr: *mut u8,
                stack_size: usize,
                entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
                arg1: *mut (),
                arg2: *mut (),
            );
        }

        unsafe {
            __eva_kernel_1_0_init_switchctx(switchctx_ptr, stack_ptr, stack_size, entry, arg1, arg2)
        }
    }

    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8) {
        unsafe extern "Rust" {
            unsafe fn __eva_kernel_1_0_set_global_switchctx(switchctx_ptr: *mut u8);
        }

        unsafe { __eva_kernel_1_0_set_global_switchctx(switchctx_ptr) }
    }

    fn yield_now() {
        unsafe extern "Rust" {
            unsafe fn __eva_kernel_1_0_yield_now();
        }

        unsafe { __eva_kernel_1_0_yield_now() }
    }

    fn kprint_fmt(args: fmt::Arguments) {
        unsafe extern "Rust" {
            unsafe fn __eva_kernel_1_0_kprint_fmt(args: fmt::Arguments);
        }

        unsafe { __eva_kernel_1_0_kprint_fmt(args) }
    }

    fn get_time() -> Duration {
        unsafe extern "Rust" {
            unsafe fn __eva_kernel_1_0_get_time() -> Duration;
        }

        unsafe { __eva_kernel_1_0_get_time() }
    }
}
