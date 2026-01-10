#![feature(restricted_std)]

extern crate eva_bsp_linux;

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;

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
    let pusher = thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            println!("Entering pusher!");
            for i in 0..100 {
                QUEUE.push(i);
                println!("Pushed 1 item!");
            }
            println!("Exiting pusher!");
        })
        .unwrap();

    let popper = thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            println!("Entering popper!");
            for i in 0..100 {
                let item = QUEUE.pop();
                assert_eq!(i, item);
                println!("Popped: {}", item);
            }
            println!("Exiting popper!");
        })
        .unwrap();

    pusher.join().unwrap();
    popper.join().unwrap();
    println!("Exiting main!");
}
