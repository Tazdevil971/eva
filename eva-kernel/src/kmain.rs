#[macro_export]
macro_rules! kmain {
    ($name:expr) => {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn __eva_kmain() {
            const MAIN: fn() = $name;
            (MAIN)();
        }
    };
}

pub use kmain;

pub fn invoke() {
    unsafe extern "C" {
        unsafe fn __eva_kmain();
    }

    unsafe {
        __eva_kmain();
    }

    // TODO: What about here? What should we do when main returns?
}
