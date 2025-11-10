#![no_std]
#![no_main]

extern crate eva_bsp_stm32f767;

use core::time::Duration;

use eva_kernel::kprintln;
use eva_kernel::scheduler::sync::mutex::Mutex;
use eva_kernel::scheduler::thread;

static GLOBAL_VAR: Mutex<u32> = Mutex::new(0);

eva_kernel::kmain!(main);

fn main() {
    kprintln!("Hello world!");

    let thread = unsafe { thread::spawn(4096 * 4, 0, other_thread, core::ptr::null_mut()) };

    kprintln!("1) Before lock");
    {
        let mut lock = GLOBAL_VAR.lock();
        thread::yield_now();
        *lock += 1;
        kprintln!("3) After update");
    }
    kprintln!("4) After lock");
    kprintln!("5) Before preempt!");
    unsafe {
        thread.join();
    }
    kprintln!("7) After preempt!");

    let thread2 = unsafe { thread::spawn(4096 * 4, 0, other_other_thread, core::ptr::null_mut()) };

    loop {
        kprintln!(
            "8) Main thread! {}",
            eva_kernel::time::get_time().as_secs_f32()
        );
        eva_kernel::scheduler::time::sleep(Duration::from_millis(1666));
    }
}

extern "C" fn other_thread(_user: *mut ()) {
    kprintln!("2) Other thread!");
    let mut lock = GLOBAL_VAR.lock();
    *lock += 1;
    kprintln!("6) Variable: {}", lock);
}

extern "C" fn other_other_thread(_user: *mut ()) {
    loop {
        kprintln!(
            "9) Other other thread! {}",
            eva_kernel::time::get_time().as_secs_f32()
        );
        eva_kernel::scheduler::time::sleep(Duration::from_millis(1000));
    }
}
