use core::cell::RefCell;
use core::mem;
use core::ptr::NonNull;
use core::time::Duration;

use crate::rt::pause::{PauseCell, PauseToken};
use crate::rt::thread::ThreadPtr;
use crate::utils::linked_list::{self, Link, LinkedList};
use crate::utils::scopeguard::defer;
use crate::utils::unchecked_ref::UncheckedRef;
use crate::utils::{assert_send, assert_sync};
use crate::{kdbg, rt};

#[derive(Debug)]
pub struct PriorityWakeup {
    link: PauseCell<Link>,
    thread: ThreadPtr,
}

assert_send!(PriorityWakeup);
assert_sync!(PriorityWakeup);

impl PriorityWakeup {
    pub fn is_signaled(&self, token: PauseToken) -> bool {
        !self.link.borrow(token).is_linked()
    }
}

struct PriorityWakeupAdapter;

unsafe impl linked_list::Adapter for PriorityWakeupAdapter {
    type Ptr = UncheckedRef<PriorityWakeup>;
    type Value = PriorityWakeup;

    fn offset_of_link(&self) -> usize {
        mem::offset_of!(PriorityWakeup, link)
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<PriorityWakeup>) -> UncheckedRef<PriorityWakeup> {
        unsafe { UncheckedRef::from_raw(raw) }
    }

    fn ptr_to_raw(&self, ptr: UncheckedRef<PriorityWakeup>) -> NonNull<PriorityWakeup> {
        ptr.into_raw()
    }
}

pub struct PriorityWakeList {
    list: PauseCell<RefCell<LinkedList<PriorityWakeupAdapter>>>,
}

assert_send!(PriorityWakeList);
assert_sync!(PriorityWakeList);

impl PriorityWakeList {
    pub const fn new() -> Self {
        Self {
            list: PauseCell::new(RefCell::new(LinkedList::new(PriorityWakeupAdapter))),
        }
    }

    pub fn with_wakeup<F, T>(&self, token: PauseToken, f: F) -> T
    where
        F: FnOnce(PauseToken, &PriorityWakeup) -> T,
    {
        let thread = rt::current();

        let node = PriorityWakeup {
            link: PauseCell::new(Link::unlinked()),
            thread,
        };

        let priority = thread.tcb().priority;

        // Insert the node in the appropriate position
        {
            let mut list = self.list.borrow_ref_mut(token);
            let mut cursor = list.cursor_front_mut();

            // Search for a tread with higher or equal priority
            while let Some(value) = cursor.current() {
                let priority2 = value.thread.tcb().priority;
                if priority2 >= priority {
                    break;
                }

                // Move to the next one
                cursor.move_next();
            }

            cursor.insert_before(unsafe {
                // SAFETY: The node is always removed from the list before going out of scope
                UncheckedRef::erase_lifetime(&node)
            });
        }

        // Remove the node at the end of the scope
        defer! {
            // Do nothing if the node is already unlinked
            if !node.link.borrow(token).is_linked() {
                return;
            }

            let mut list = self.list.borrow_ref_mut(token);
            let mut cursor = unsafe {
                // SAFETY: This node is inside the list and the pointer is valid
                list.cursor_mut_from_raw(NonNull::from(&node))
            };

            cursor.remove_current();
        }

        f(token, &node)
    }

    pub fn wakeup_one(&self, token: PauseToken) -> bool {
        let mut list = self.list.borrow_ref_mut(token);

        let Some(node) = list.pop_back() else {
            return false;
        };

        rt::resume_paused(token, node.thread).expect("thread in wake list but awake");
        true
    }

    #[allow(unused)]
    pub fn wakeup_all(&self, token: PauseToken) {
        let mut list = self.list.borrow_ref_mut(token);

        while let Some(node) = list.pop_back() {
            rt::resume_paused(token, node.thread).expect("thread in wake list but awake");
        }
    }
}

#[derive(Debug)]
pub struct TimedWakeup {
    link: PauseCell<Link>,
    thread: ThreadPtr,
    timeout: Duration,
}

assert_send!(TimedWakeup);
assert_sync!(TimedWakeup);

impl TimedWakeup {
    pub(super) fn is_expired(&self, token: PauseToken) -> bool {
        !self.link.borrow(token).is_linked()
    }
}

struct TimedWakeupAdapter;

unsafe impl linked_list::Adapter for TimedWakeupAdapter {
    type Ptr = UncheckedRef<TimedWakeup>;
    type Value = TimedWakeup;

    fn offset_of_link(&self) -> usize {
        mem::offset_of!(TimedWakeup, link)
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<TimedWakeup>) -> UncheckedRef<TimedWakeup> {
        unsafe { UncheckedRef::from_raw(raw) }
    }

    fn ptr_to_raw(&self, ptr: UncheckedRef<TimedWakeup>) -> NonNull<TimedWakeup> {
        ptr.into_raw()
    }
}

pub(super) struct TimedWakeList {
    list: PauseCell<RefCell<LinkedList<TimedWakeupAdapter>>>,
}

impl TimedWakeList {
    pub(super) const fn new() -> Self {
        Self {
            list: PauseCell::new(RefCell::new(LinkedList::new(TimedWakeupAdapter))),
        }
    }

    pub(super) fn with_wakeup<F, T>(&self, token: PauseToken, timeout: Duration, f: F) -> T
    where
        F: FnOnce(PauseToken, &TimedWakeup) -> T,
    {
        let node = TimedWakeup {
            link: PauseCell::new(Link::unlinked()),
            thread: rt::current(),
            timeout,
        };

        // Insert the node in the appropriate position
        {
            let mut list = self.list.borrow_ref_mut(token);
            let mut cursor = list.cursor_front_mut();

            while let Some(value) = cursor.current() {
                if value.timeout < node.timeout {
                    break;
                }

                cursor.move_next();
            }

            cursor.insert_before(unsafe {
                // SAFETY: The node is always removed from the list before going out of scope
                UncheckedRef::erase_lifetime(&node)
            });
        }

        // Defer the node at the end of the scope
        defer! {
            if !node.link.borrow(token).is_linked() {
                return;
            }

            let mut list = self.list.borrow_ref_mut(token);
            let mut cursor = unsafe {
                // SAFETY: This node is inside the list and the pointer is valid
                list.cursor_mut_from_raw(NonNull::from(&node))
            };

            cursor.remove_current();
        }

        f(token, &node)
    }

    pub(super) fn wakeup_until(&self, token: PauseToken, instant: Duration) {
        let mut list = self.list.borrow_ref_mut(token);

        while let Some(node) = list.pop_back_if(|ptr| ptr.timeout < instant) {
            kdbg!(&list);
            rt::resume_paused(token, node.thread).expect("thread in wake list but awake");
        }
    }
}
