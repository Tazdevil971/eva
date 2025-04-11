use super::{try_pause, unpause};

struct PauseImpl;

unsafe impl critical_section::Impl for PauseImpl {
    unsafe fn acquire() -> bool {
        try_pause().is_ok()
    }

    unsafe fn release(old_state: bool) {
        if old_state {
            unsafe {
                unpause();
            }
        }
    }
}

critical_section::set_impl!(PauseImpl);