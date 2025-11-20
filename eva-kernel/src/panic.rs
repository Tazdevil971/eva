use core::panic::PanicInfo;

use crate::kprintln;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::rt::abort();
    
    kprintln!("{}", info);
    loop {}
}
