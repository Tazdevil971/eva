use core::cell::{Cell, RefCell};
use core::ptr::NonNull;

use crate::pause::{PauseMutex, PauseToken, ask_for_preempt, is_paused, with_pause};
use crate::raw_thread::{self, RawThread, current, suspend_pt};

struct WaitQueueEntry {
    prev: PauseMutex<Cell<Option<WaitQueueEntryPtr>>>,
    next: PauseMutex<Cell<Option<WaitQueueEntryPtr>>>,
    thread: RawThread,
}

#[derive(Debug, Clone, Copy)]
struct WaitQueueEntryPtr(NonNull<WaitQueueEntry>);

unsafe impl Send for WaitQueueEntryPtr {}
unsafe impl Sync for WaitQueueEntryPtr {}

impl WaitQueueEntryPtr {
    unsafe fn new(entry: &WaitQueueEntry) -> Self {
        Self(NonNull::from(entry))
    }

    unsafe fn entry<'a>(&self) -> &'a WaitQueueEntry {
        unsafe { self.0.as_ref() }
    }

    unsafe fn iter(
        start: Option<WaitQueueEntryPtr>,
        token: PauseToken,
    ) -> impl Iterator<Item = WaitQueueEntryPtr> {
        let mut iter = start;
        core::iter::from_fn(move || unsafe {
            let cur = iter?;
            iter = cur.entry().next.get(token);
            Some(cur)
        })
    }
}

#[derive(Debug)]
struct WaitQueueInner {
    first: Option<WaitQueueEntryPtr>,
    last: Option<WaitQueueEntryPtr>,
}

#[derive(Debug)]
pub struct WaitQueue {
    inner: PauseMutex<RefCell<WaitQueueInner>>,
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self {
            inner: PauseMutex::new(RefCell::new(WaitQueueInner {
                first: None,
                last: None,
            })),
        }
    }

    pub fn wait<F>(&self, mut cond: F)
    where
        F: FnMut(PauseToken) -> bool,
    {
        assert!(!is_paused(), "Cannot wait if the kernel is already paused!");
        let thread = raw_thread::current();

        let entry = WaitQueueEntry {
            thread,
            prev: PauseMutex::default(),
            next: PauseMutex::default(),
        };

        let new = unsafe { WaitQueueEntryPtr::new(&entry) };

        while with_pause(|token| {
            if cond(token) {
                return false;
            }

            let mut inner = self.inner.borrow_ref_mut(token);

            if let Some(mut last) = inner.last {
                unsafe { last.entry() }.next.set(token, Some(new));
                unsafe { new.entry() }.prev.set(token, Some(last));

                inner.last = Some(new);
            } else {
                inner.first = Some(new);
                inner.last = Some(new);
            }

            suspend_pt(token);
            ask_for_preempt(token);
            true
        }) {}
    }

    unsafe fn wake_internal(token: PauseToken, thread: RawThread) {
        let current = current();
        unsafe {
            // Resume the new thread
            thread.resume_pt(token);

            // Check if the current priority is lower than the waken one
            if current.priority() > thread.priority() {
                // Schedule an immediate preemption
                ask_for_preempt(token);
            }
        }
    }

    pub fn wake_first(&self, token: PauseToken) {
        let mut inner = self.inner.borrow_ref_mut(token);

        if let Some(first) = inner.first {
            // Find the next one
            let next = unsafe { first.entry() }.next.get(token);

            inner.first = next;
            // In case there is no one, also reset the last pointer
            if next.is_none() {
                inner.last = None;
            }

            unsafe {
                Self::wake_internal(token, first.entry().thread);
            }
        }
    }

    pub fn wake_last(&self, token: PauseToken) {
        let mut inner = self.inner.borrow_ref_mut(token);

        if let Some(last) = inner.last {
            // Find the previous one
            let prev = unsafe { last.entry() }.prev.get(token);

            inner.last = prev;
            // In case there is no one, also reset the first pointer
            if prev.is_none() {
                inner.first = None;
            }

            unsafe {
                Self::wake_internal(token, last.entry().thread);
            }
        }
    }

    pub fn wake_all(&self, token: PauseToken) {
        let mut inner = self.inner.borrow_ref_mut(token);

        // Wake all of the threads
        unsafe {
            WaitQueueEntryPtr::iter(inner.first, token).for_each(|entry| unsafe {
                Self::wake_internal(token, entry.entry().thread);
            });
        }

        // Reset the queue
        inner.first = None;
        inner.last = None;
    }

    pub fn wake_highest(&self, token: PauseToken) {
        let mut inner = self.inner.borrow_ref_mut(token);

        // Find the highest priority thread
        let max = unsafe {
            WaitQueueEntryPtr::iter(inner.first, token)
                .min_by_key(|entry| unsafe { entry.entry().thread.priority() })
        };

        if let Some(max) = max {
            unsafe {
                let prev = max.entry().prev.get(token);
                let next = max.entry().next.get(token);

                if let Some(prev) = prev {
                    prev.entry().next.set(token, next);
                } else {
                    inner.first = next;
                }

                if let Some(next) = next {
                    next.entry().prev.set(token, prev);
                } else {
                    inner.last = prev;
                }
            }

            unsafe {
                Self::wake_internal(token, max.entry().thread);
            }
        }
    }
}
