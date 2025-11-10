use core::panic::PanicInfo;

use crate::kprintln;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{}", info);

    super::panic_shutdown();
    loop {}
}
