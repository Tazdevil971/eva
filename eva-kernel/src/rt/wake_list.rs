use core::cell::UnsafeCell;
use core::ptr;
use core::ptr::NonNull;
use core::time::Duration;

use crate::rt;
use crate::rt::pause::{PauseCell, PauseToken};
use crate::rt::thread::ThreadPtr;

use bytemuck::Zeroable;
use eva_utils::linked_list::{self, LinkOps as _, LinkedList};
use eva_utils::unchecked_ref::UncheckedRef;
use eva_utils::{assert_send, assert_sync};
use scopeguard::defer;

/*

TODO: We use UnsafeCell inside of the wait lists for two main reasons:
- We need something that can be zeroable, UnsafeCell is transparent so we are good
- Space is a constraint, since we are going to use it in Mutex and Condvar

This makes this code _more_ unsafe than it should, in the future we might use an ad-hoc structure.
A GlobalCell for example? A cell that uses a global counter?

Also UncheckedRef is hideous, we can fix it by using a combination of Pin to run panic
in the destructor, and enforce manual explicit unsafe destruction

*/

#[derive(Debug)]
pub struct PriorityWakeup {
    link: linked_list::AtomicLink<Self>,
    thread: ThreadPtr,
}

assert_send!(PriorityWakeup);
assert_sync!(PriorityWakeup);

impl PriorityWakeup {
    pub fn is_signaled(&self) -> bool {
        !self.link.is_linked()
    }
}

#[derive(Zeroable)]
struct PriorityWakeupAdapter;

impl linked_list::Adapter for PriorityWakeupAdapter {
    type Ptr = UncheckedRef<PriorityWakeup>;
    type Value = PriorityWakeup;
    type Link = linked_list::AtomicLink<PriorityWakeup>;

    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link> {
        unsafe {
            let ptr = ptr::addr_of_mut!((*raw.as_ptr()).link);
            NonNull::new_unchecked(ptr)
        }
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<PriorityWakeup>) -> UncheckedRef<PriorityWakeup> {
        unsafe { UncheckedRef::from_raw(raw) }
    }

    fn ptr_to_raw(&self, ptr: UncheckedRef<PriorityWakeup>) -> NonNull<PriorityWakeup> {
        ptr.into_raw()
    }
}

#[derive(Zeroable)]
pub struct PriorityWakeList {
    list: PauseCell<UnsafeCell<LinkedList<PriorityWakeupAdapter>>>,
}

assert_send!(PriorityWakeList);
assert_sync!(PriorityWakeList);

impl PriorityWakeList {
    pub const fn new() -> Self {
        Self {
            list: PauseCell::unsafe_cell(LinkedList::new(PriorityWakeupAdapter)),
        }
    }

    pub fn with_wakeup<F, T>(&self, token: PauseToken, f: F) -> T
    where
        F: FnOnce(PauseToken, &PriorityWakeup) -> T,
    {
        let thread = rt::current_raw();

        let node = PriorityWakeup {
            link: linked_list::AtomicLink::unlinked(),
            thread,
        };

        let priority = thread.priority;

        // Insert the node in the appropriate position
        {
            let list = unsafe {
                // SAFETY: Since the scheduler is paused and we do not yield, we ensure that this is the only alive mutable reference
                self.list.as_mut_unchecked(token)
            };
            let mut cursor = list.cursor_front_mut();

            // Search for a tread with higher or equal priority
            while let Some(value) = cursor.current() {
                let priority2 = value.thread.priority;
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
            if !node.link.is_linked() {
                return;
            }

            let list = unsafe {
                // SAFETY: Since the scheduler is paused and we do not yield, we ensure that this is the only alive mutable reference
                self.list.as_mut_unchecked(token)
            };
            let mut cursor = unsafe {
                // SAFETY: This node is inside the list and the pointer is valid
                list.cursor_mut_from_raw(NonNull::from(&node))
            };

            cursor.remove_current();
        }

        f(token, &node)
    }

    pub fn wakeup_one(&self, token: PauseToken) -> bool {
        let list = unsafe {
            // SAFETY: Since the scheduler is paused and we do not yield, we ensure that this is the only alive mutable reference
            self.list.as_mut_unchecked(token)
        };

        let Some(node) = list.pop_back() else {
            return false;
        };

        let _ = rt::resume_paused_raw(token, node.thread);
        true
    }

    pub fn wakeup_all(&self, token: PauseToken) {
        let list = unsafe {
            // SAFETY: Since the scheduler is paused and we do not yield, we ensure that this is the only alive mutable reference
            self.list.as_mut_unchecked(token)
        };

        while let Some(node) = list.pop_back() {
            let _ = rt::resume_paused_raw(token, node.thread);
        }
    }
}

#[derive(Debug)]
pub struct TimedWakeup {
    link: linked_list::AtomicLink<Self>,
    thread: ThreadPtr,
    timeout: Duration,
}

assert_send!(TimedWakeup);
assert_sync!(TimedWakeup);

impl TimedWakeup {
    pub(super) fn is_expired(&self) -> bool {
        !self.link.is_linked()
    }
}

#[derive(Zeroable)]
struct TimedWakeupAdapter;

impl linked_list::Adapter for TimedWakeupAdapter {
    type Ptr = UncheckedRef<TimedWakeup>;
    type Value = TimedWakeup;
    type Link = linked_list::AtomicLink<TimedWakeup>;

    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link> {
        unsafe {
            let ptr = ptr::addr_of_mut!((*raw.as_ptr()).link);
            NonNull::new_unchecked(ptr)
        }
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<TimedWakeup>) -> UncheckedRef<TimedWakeup> {
        unsafe { UncheckedRef::from_raw(raw) }
    }

    fn ptr_to_raw(&self, ptr: UncheckedRef<TimedWakeup>) -> NonNull<TimedWakeup> {
        ptr.into_raw()
    }
}

pub(super) struct TimedWakeList {
    list: PauseCell<UnsafeCell<LinkedList<TimedWakeupAdapter>>>,
}

impl TimedWakeList {
    pub(super) const fn new() -> Self {
        Self {
            list: PauseCell::unsafe_cell(LinkedList::new(TimedWakeupAdapter)),
        }
    }

    pub(super) fn with_wakeup<F, T>(&self, token: PauseToken, timeout: Duration, f: F) -> T
    where
        F: FnOnce(PauseToken, &TimedWakeup) -> T,
    {
        let node = TimedWakeup {
            link: linked_list::AtomicLink::unlinked(),
            thread: rt::current_raw(),
            timeout,
        };

        // Insert the node in the appropriate position
        {
            let list = unsafe { self.list.as_mut_unchecked(token) };
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
            if !node.link.is_linked() {
                return;
            }

            let list = unsafe { self.list.as_mut_unchecked(token) };
            let mut cursor = unsafe {
                // SAFETY: This node is inside the list and the pointer is valid
                list.cursor_mut_from_raw(NonNull::from(&node))
            };

            cursor.remove_current();
        }

        f(token, &node)
    }

    pub(super) fn wakeup_until(&self, token: PauseToken, instant: Duration) {
        let list = unsafe { self.list.as_mut_unchecked(token) };

        while let Some(node) = list.pop_back_if(|ptr| ptr.timeout < instant) {
            let _ = rt::resume_paused_raw(token, node.thread);
        }
    }
}
