use core::alloc::Layout;
use core::ptr;

use crate::raw_thread::RawThread;
use crate::scheduler;

pub unsafe trait Impl {
    fn switchctx_layout() -> Layout;

    unsafe fn init_switchctx(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
        arg1: *mut (),
        arg2: *mut (),
    );

    unsafe fn set_global_switchctx(switchctx_ptr: *mut u8);

    unsafe fn enter_first_thread(
        switchctx_ptr: *mut u8,
        stack_ptr: *mut u8,
        entry: unsafe extern "C" fn() -> !,
    ) -> !;

    fn preempt();
}

#[macro_export]
macro_rules! set_portability_impl {
    ($t:ty) => {
        const _: () = {
            #[unsafe(no_mangle)]
            unsafe fn _eva_scheduler_1_0_switchctx_layout() -> core::alloc::Layout {
                unsafe { <$t as $crate::portability::Impl>::switchctx_layout() }
            }

            #[unsafe(no_mangle)]
            unsafe fn _eva_scheduler_1_0_init_switchctx(
                switchctx_ptr: *mut u8,
                stack_ptr: *mut u8,
                entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
                arg1: *mut (),
                arg2: *mut (),
            ) {
                unsafe {
                    <$t as $crate::portability::Impl>::init_switchctx(
                        switchctx_ptr,
                        stack_ptr,
                        entry,
                        arg1,
                        arg2,
                    )
                }
            }

            #[unsafe(no_mangle)]
            unsafe fn _eva_scheduler_1_0_set_global_switchctx(switchctx_ptr: *mut u8) {
                unsafe { <$t as $crate::portability::Impl>::set_global_switchctx(switchctx_ptr) }
            }

            #[unsafe(no_mangle)]
            unsafe fn _eva_scheduler_1_0_enter_first_thread(
                switchctx_ptr: *mut u8,
                stack_ptr: *mut u8,
                entry: unsafe extern "C" fn() -> !,
            ) -> ! {
                unsafe {
                    <$t as $crate::portability::Impl>::enter_first_thread(
                        switchctx_ptr,
                        stack_ptr,
                        entry,
                    )
                }
            }

            #[unsafe(no_mangle)]
            fn _eva_scheduler_1_0_preempt() {
                <$t as $crate::portability::Impl>::preempt()
            }
        };
    };
}

pub(crate) fn switchctx_layout() -> Layout {
    unsafe extern "Rust" {
        unsafe fn _eva_scheduler_1_0_switchctx_layout() -> Layout;
    }

    unsafe { _eva_scheduler_1_0_switchctx_layout() }
}

pub(crate) unsafe fn init_switchctx(
    switchctx_ptr: *mut u8,
    stack_ptr: *mut u8,
    entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
    arg1: *mut (),
    arg2: *mut (),
) {
    unsafe extern "Rust" {
        unsafe fn _eva_scheduler_1_0_init_switchctx(
            switchctx_ptr: *mut u8,
            stack_ptr: *mut u8,
            entry: unsafe extern "C" fn(*mut (), *mut ()) -> !,
            arg1: *mut (),
            arg2: *mut (),
        );
    }

    unsafe { _eva_scheduler_1_0_init_switchctx(switchctx_ptr, stack_ptr, entry, arg1, arg2) }
}

pub(crate) unsafe fn set_global_switchctx(switchctx_ptr: *mut u8) {
    unsafe extern "Rust" {
        unsafe fn _eva_scheduler_1_0_set_global_switchctx(switchctx_ptr: *mut u8);
    }

    unsafe { _eva_scheduler_1_0_set_global_switchctx(switchctx_ptr) }
}

pub(crate) unsafe fn enter_first_thread(
    switchctx_ptr: *mut u8,
    stack_ptr: *mut u8,
    entry: unsafe extern "C" fn() -> !,
) -> ! {
    unsafe extern "Rust" {
        unsafe fn _eva_scheduler_1_0_enter_first_thread(
            switchctx_ptr: *mut u8,
            stack_ptr: *mut u8,
            entry: unsafe extern "C" fn() -> !,
        ) -> !;
    }

    unsafe { _eva_scheduler_1_0_enter_first_thread(switchctx_ptr, stack_ptr, entry) }
}

pub(crate) fn preempt() {
    unsafe extern "Rust" {
        unsafe fn _eva_scheduler_1_0_preempt();
    }

    unsafe { _eva_scheduler_1_0_preempt() }
}

pub use scheduler::{enter, rotate};
