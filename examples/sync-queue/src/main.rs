#![feature(restricted_std)]

extern crate eva_bsp_stm32f767;

use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};
use std::ptr;

use eva_kernel::{kprintln, rt};

struct SyncQueue<T> {
    inner: Mutex<VecDeque<T>>,
    tx: Condvar,
    rx: Condvar,
}

impl<T> SyncQueue<T> {
    const CAPACITY: usize = 10;

    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
            tx: Condvar::new(),
            rx: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) {
        let mut lock = self.inner.lock().unwrap();
        while lock.len() >= Self::CAPACITY {
            lock = self.tx.wait(lock).unwrap();
        }

        // We have space
        lock.push_front(item);
        // Notify one of the readers
        self.rx.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut lock = self.inner.lock().unwrap();
        let item = loop {
            if let Some(item) = lock.pop_back() {
                break item;
            }

            // The queue is empty, wait for a signal
            lock = self.rx.wait(lock).unwrap();
        };

        self.tx.notify_one();
        item
    }
}

static QUEUE: SyncQueue<u32> = SyncQueue::new();

fn main() {
    // Spawn the two threads

    /*
    let thread1 = rt::spawn(4096 * 16, 0, thread1, c"Thread1", ptr::null_mut()).unwrap();
    let thread2 = rt::spawn(4096 * 16, 0, thread2, c"Thread2", ptr::null_mut()).unwrap();

    unsafe {
        rt::join_unchecked(thread1).unwrap();
        rt::join_unchecked(thread2).unwrap();
    }
    */

    let thread1 = std::thread::spawn(|| {
        kprintln!("Entering thread1!");
        for i in 0..100 {
            QUEUE.push(i);
            kprintln!("Pushed 1 item!");
        }
        kprintln!("Exiting thread1!");
    });

    let thread2 = std::thread::spawn(|| {
        kprintln!("Entering thread2!");
        for i in 0..100 {
            let item = QUEUE.pop();
            assert_eq!(i, item);
            kprintln!("Popped: {}", item);
        }
        kprintln!("Exiting thread2!");
    });

    thread1.join();
    thread2.join();
}

extern "C" fn thread1(_user1: *mut ()) {
    kprintln!("Entering thread1!");
    for i in 0..100 {
        QUEUE.push(i);
        kprintln!("Pushed 1 item!");
    }
    kprintln!("Exiting thread1!");
}

extern "C" fn thread2(_user1: *mut ()) {
    kprintln!("Entering thread2!");
    for i in 0..100 {
        let item = QUEUE.pop();
        assert_eq!(i, item);
        kprintln!("Popped: {}", item);
    }
    kprintln!("Exiting thread2!");
}
