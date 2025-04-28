#![no_std]
#![no_main]

extern crate eva_bsp;

use eva_scheduler::sync::mutex::Mutex;
use eva_scheduler::thread::{self, Priority};
use eva_scheduler::scheduler;

static GLOBAL_VAR: Mutex<u32> = Mutex::new(0);

eva_bsp::eva_main!(main);

fn main() {
    rtt_target::rprintln!("Hello world!");

    let thread = thread::spawn(other_thread, 4096, Priority::MIN);

    rtt_target::rprintln!("1) Before lock");
    {
        let mut lock = GLOBAL_VAR.lock();
        eva_scheduler::raw_thread::yield_now();
        *lock += 1;
        rtt_target::rprintln!("3) After update");
    }
    rtt_target::rprintln!("4) After lock");
    rtt_target::rprintln!("5) Before preempt!");
    thread.join();
    rtt_target::rprintln!("7) After preempt!");
}

fn other_thread() {
    rtt_target::rprintln!("2) Other thread!");
    let mut lock = GLOBAL_VAR.lock();
    *lock += 1;
    rtt_target::rprintln!("6) Variable: {}", lock);
}
