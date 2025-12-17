#![no_std]
#![no_main]

extern crate eva_bsp_linux;

use core::num::NonZeroU32;
use core::ptr;
use core::ptr::NonNull;
use core::time::Duration;

use eva_kernel::kprintln;
use eva_kernel::rt;
use eva_kernel::rt::sync::Mutex;
use eva_kernel::rt::tls;

static GLOBAL_VAR: Mutex<u32> = Mutex::new(0);

eva_kernel::kmain!(main);

static mut MY_KEY: tls::TlsKey = tls::TlsKey(NonZeroU32::new(1).unwrap());

extern "C" fn my_dtor(data: *mut ()) {
    kprintln!("Run dtor for: {:?}", data);
}

fn main() {
    kprintln!("Hello world!");

    unsafe {
        MY_KEY = eva_abi::eva_rt_tls_key_create(my_dtor);
    }

    let thread =
        unsafe { eva_abi::eva_rt_spawn(4096 * 16, 0, other_thread, ptr::null_mut()).unwrap() };

    kprintln!("1) Before lock");
    {
        let mut lock = GLOBAL_VAR.lock();
        unsafe {
            eva_abi::eva_rt_yield_now();
        }
        *lock += 1;
        kprintln!("3) After update");
    }
    kprintln!("4) After lock");
    kprintln!("5) Before preempt!");
    unsafe {
        eva_abi::eva_rt_join_unchecked(thread).expect("already joined!");
    }
    kprintln!("7) After preempt!");

    let _thread2 = rt::spawn(4096 * 16, 0, other_other_thread, ptr::null_mut());

    loop {
        kprintln!(
            "8) Main thread! {}",
            eva_kernel::time::get_time().as_secs_f32()
        );
        unsafe {
            eva_abi::eva_rt_sleep_for(Duration::from_millis(1666));
        }
    }
}

extern "C" fn other_thread(_user: *mut ()) {
    kprintln!("2) Other thread!");
    let mut lock = GLOBAL_VAR.lock();
    *lock += 1;
    kprintln!("6) Variable: {}", lock);

    tls::set_specific(unsafe { MY_KEY }, NonNull::new(0x69 as _));
}

extern "C" fn other_other_thread(_user: *mut ()) {
    loop {
        kprintln!(
            "9) Other other thread! {}",
            eva_kernel::time::get_time().as_secs_f32()
        );
        rt::sleep_for(Duration::from_millis(1000));
    }
}
