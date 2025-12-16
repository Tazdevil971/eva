#![no_std]
#![no_main]

extern crate eva_bsp_linux;
extern crate alloc;

use alloc::collections::VecDeque;
use core::time::Duration;
use core::ptr;

use eva_kernel::rt::sync::{Mutex, Condvar};
use eva_kernel::{kprintln, rt};

eva_kernel::kmain!(main);

struct SingleQueue<T> {
    inner: Mutex<VecDeque<T>>,
    tx: Condvar,
    rx: Condvar
}

impl<T> SingleQueue<T> {
    const CAPACITY: usize = 10;

    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
            tx: Condvar::new(),
            rx: Condvar::new()
        }
    }

    pub fn push(&self, item: T) {
        let mut lock = self.inner.lock();
        loop {
            if lock.len() < Self::CAPACITY {
                // We have space
                lock.push_back(item);
                // Notify one of the readers
                self.rx.notify_one();
                return;
            }

            lock = self.tx.wait(lock);
        }
    }

    pub fn pop(&self) -> T {
        let mut lock = self.inner.lock();
        loop {
            // Try to pop a single item
            if let Some(item) = lock.pop_back() {
                // We have freed one space
                self.tx.notify_one();
                return item;
            }

            // The queue is empty, wait for a signal
            lock = self.rx.wait(lock);
        }
    }
}

static QUEUE: SingleQueue<u32> = SingleQueue::new();

fn main() {
    
    // Spawn the two threads
    let _thread1 = rt::spawn(
        4096 * 16,
        0,
        thread1,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    let _thread2 = rt::spawn(
        4096 * 16,
        0,
        thread2,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    // Spin
    loop {
        rt::sleep_until(Duration::MAX);
    }
}

extern "C" fn thread1(_user1: *mut (), _user2: *mut (), _user3: *mut ()) {
    kprintln!("Entering thread1!");
    for i in 0..100 {
        QUEUE.push(i);
        kprintln!("Pushed 1 item!");
    }
    kprintln!("Exiting thread1!");
}

extern "C" fn thread2(_user1: *mut (), _user2: *mut (), _user3: *mut ()) {
    kprintln!("Entering thread2!");
    for _ in 0..100 {
        let item = QUEUE.pop();
        kprintln!("Popped: {}", item);
    }
    kprintln!("Exiting thread2!");
}