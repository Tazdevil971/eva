#![no_std]
#![no_main]

extern crate eva_bsp;

use eva_scheduler::mutex::Mutex;
use eva_scheduler::raw_thread::{self, ThreadParams};

static GLOBAL_VAR: Mutex<u32> = Mutex::new(0);

#[unsafe(no_mangle)]
unsafe extern "C" fn __eva_start() {
    rtt_target::rprintln!("Hello world!");

    let thread = raw_thread::spawn(ThreadParams {
        stack_size: 4096,
        priority: 0,
        entry: other_thread,
        user: 0 as _,
    });

    rtt_target::rprintln!("Before lock");
    {
        let mut lock = GLOBAL_VAR.lock();
        eva_scheduler::raw_thread::preempt();
        *lock += 1;
        rtt_target::rprintln!("After update");
    }
    rtt_target::rprintln!("After lock");
    rtt_target::rprintln!("Before preempt!");
    unsafe {
        thread.join();
    }
    rtt_target::rprintln!("After preempt!");
}

unsafe extern "C" fn other_thread(_arg: *mut ()) {
    rtt_target::rprintln!("Other thread!");
    let mut lock = GLOBAL_VAR.lock();
    *lock += 1;
    rtt_target::rprintln!("Variable: {}", lock);
}
