use core::ptr::addr_of;
use core::slice;

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

unsafe fn invoke_array(start: *const u8, end: *const u8) {
    unsafe {
        // Invoke all items inside of a .*_array section, given start and end
        let len = end.byte_offset_from_unsigned(start) / core::mem::size_of::<usize>();
        if len == 0 {
            return;
        }

        let ptr = start as *const unsafe extern "C" fn();
        let array = slice::from_raw_parts(ptr, len);

        for f in array {
            (*f)();
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn __eva_default_init() {}
#[unsafe(no_mangle)]
unsafe extern "C" fn __eva_default_fini() {}

pub fn invoke() {
    unsafe extern "C" {
        unsafe static mut __preinit_array_start: u8;
        unsafe static mut __preinit_array_end: u8;
        unsafe static mut __init_array_start: u8;
        unsafe static mut __init_array_end: u8;
        unsafe static mut __fini_array_start: u8;
        unsafe static mut __fini_array_end: u8;

        unsafe fn main(argc: u32, argv: *const *const u8) -> i32;
    }

    // Perform standard libc initialization
    unsafe {
        invoke_array(
            addr_of!(__preinit_array_start) as usize as *mut u8,
            addr_of!(__preinit_array_end) as usize as *mut u8,
        );

        invoke_array(
            addr_of!(__init_array_start) as usize as *mut u8,
            addr_of!(__init_array_end) as usize as *mut u8,
        );

        main(0, core::ptr::null_mut());

        invoke_array(
            addr_of!(__fini_array_start) as usize as *mut u8,
            addr_of!(__fini_array_end) as usize as *mut u8,
        );
    }

    // TODO: What about here? What should we do when main returns?
}
