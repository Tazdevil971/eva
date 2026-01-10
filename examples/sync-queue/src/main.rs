#![no_std]
#![no_main]

extern crate alloc;
extern crate eva_bsp_linux;

use alloc::collections::VecDeque;
use core::panic::PanicInfo;
use core::ptr;

use eva_kernel::rt::sync::{Condvar, Mutex};
use eva_kernel::{kprintln, rt};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eva_kernel::rt::abort();
    kprintln!("{}", info);

    loop {}
}

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
        let mut lock = self.inner.lock();
        while lock.len() >= Self::CAPACITY {
            lock = self.tx.wait(lock);
        }

        // We have space
        lock.push_front(item);
        // Notify one of the readers
        self.rx.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut lock = self.inner.lock();
        let item = loop {
            if let Some(item) = lock.pop_back() {
                break item;
            }

            // The queue is empty, wait for a signal
            lock = self.rx.wait(lock);
        };

        self.tx.notify_one();
        item
    }
}

static QUEUE: SyncQueue<u32> = SyncQueue::new();

fn main() {
    // Spawn the two threads

    let pusher = rt::spawn(64 * 1024, 0, pusher, c"Pusher", ptr::null_mut()).unwrap();
    let popper = rt::spawn(64 * 1024, 0, popper, c"Popper", ptr::null_mut()).unwrap();

    rt::join(pusher).unwrap();
    rt::join(popper).unwrap();

    kprintln!("Exiting main!");
}

eva_kernel::kmain!(main);

extern "C" fn pusher(_user: *mut ()) {
    kprintln!("Entering pusher!");
    for i in 0..100 {
        QUEUE.push(i);
        kprintln!("Pushed 1 item!");
    }
    kprintln!("Exiting pusher!");
}

extern "C" fn popper(_user: *mut ()) {
    kprintln!("Entering popper!");
    for i in 0..100 {
        let item = QUEUE.pop();
        assert_eq!(i, item);
        kprintln!("Popped: {}", item);
    }
    kprintln!("Exiting popper!");
}
