#![no_std]
#![no_main]

extern crate eva_bsp_linux;

use core::ptr;
use core::time::Duration;

use eva_kernel::kprintln;
use eva_kernel::rt;
use eva_kernel::rt::sync::Mutex;

static GLOBAL_VAR: Mutex<u32> = Mutex::new(0);

eva_kernel::kmain!(main);

fn main() {
    kprintln!("Hello world!");

    let thread = rt::spawn(
        4096 * 16,
        0,
        other_thread,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    kprintln!("1) Before lock");
    {
        let mut lock = GLOBAL_VAR.lock();
        rt::yield_now();
        *lock += 1;
        kprintln!("3) After update");
    }
    kprintln!("4) After lock");
    kprintln!("5) Before preempt!");
    unsafe {
        rt::join(thread).expect("already joined!");
    }
    kprintln!("7) After preempt!");

    let _thread2 = rt::spawn(
        4096 * 16,
        0,
        other_other_thread,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    loop {
        kprintln!(
            "8) Main thread! {}",
            eva_kernel::time::get_time().as_secs_f32()
        );
        rt::sleep(Duration::from_millis(1666));
    }
}

extern "C" fn other_thread(_user1: *mut (), _user2: *mut (), _user3: *mut ()) {
    kprintln!("2) Other thread!");
    let mut lock = GLOBAL_VAR.lock();
    *lock += 1;
    kprintln!("6) Variable: {}", lock);
}

extern "C" fn other_other_thread(_user1: *mut (), _user2: *mut (), _user3: *mut ()) {
    loop {
        kprintln!(
            "9) Other other thread! {}",
            eva_kernel::time::get_time().as_secs_f32()
        );
        rt::sleep(Duration::from_millis(1000));
    }
}
