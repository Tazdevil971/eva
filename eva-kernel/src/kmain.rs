#[macro_export]
macro_rules! kmain {
    ($name:expr) => {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn eva_kmain() {
            const MAIN: fn() = $name;
            (MAIN)();
        }
    };
}

pub use kmain;

pub fn invoke() {
    unsafe extern "C" {
        unsafe fn eva_kmain();
    }

    unsafe {
        eva_kmain();
    }

    // TODO: What about here? What should we do when main returns?
}
