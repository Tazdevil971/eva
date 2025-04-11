#![no_std]
#![no_main]

extern crate eva_bsp_linux;

use eva_scheduler::sync::mutex::Mutex;
use eva_scheduler::raw_thread::{self, Priority};
use eva_scheduler::kprintln;

static GLOBAL_VAR: Mutex<u32> = Mutex::new(0);

eva_bsp_linux::eva_main!(main);

fn main() {
    kprintln!("Hello world!");

    let thread = unsafe {
        raw_thread::spawn(4096 * 4, Priority::MIN, other_thread, core::ptr::null_mut())
    };

    kprintln!("1) Before lock");
    {
        let mut lock = GLOBAL_VAR.lock();
        raw_thread::yield_now();
        *lock += 1;
        kprintln!("3) After update");
    }
    kprintln!("4) After lock");
    kprintln!("5) Before preempt!");
    unsafe {
        thread.join();
    }
    kprintln!("7) After preempt!");
}

extern "C" fn other_thread(_user: *mut ()) {
    kprintln!("2) Other thread!");
    let mut lock = GLOBAL_VAR.lock();
    *lock += 1;
    kprintln!("6) Variable: {}", lock);
}
