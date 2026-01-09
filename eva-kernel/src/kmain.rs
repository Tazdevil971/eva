#[macro_export]
macro_rules! kmain {
    ($name:expr) => {
        const _: () = {
            #[unsafe(export_name = "main")]
            unsafe extern "C" fn __priv_kmain(argc: u32, argv: *const *const u8) -> i32 {
                const MAIN: fn() = $name;
                (MAIN)();
                0
            }
        };
    };
}

pub use kmain;

pub fn invoke() {
    unsafe extern "C" {
        unsafe fn main(argc: u32, argv: *const *const u8) -> i32;
    }

    unsafe {
        main(0, core::ptr::null_mut());
    }

    // TODO: What about here? What should we do when main returns?
}
