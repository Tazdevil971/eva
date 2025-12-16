use alloc::boxed::Box;
use core::cell::RefCell;
use core::mem;
use core::ptr::NonNull;

use crate::rt;
use crate::rt::pause::PauseCell;
use crate::rt::sync::Mutex;
use crate::utils::linked_list::{self, Link, LinkedList};
use crate::utils::slot_map::SlotMap;

pub type Key = crate::utils::slot_map::SlotId;
pub type Dtor = extern "C" fn(NonNull<()>);

static KEY_STORE: Mutex<SlotMap<Dtor>> = Mutex::new(SlotMap::new());

pub fn key_create(dtor: Dtor) -> Key {
    KEY_STORE.lock().insert(dtor)
}

pub fn key_delete(key: Key) {
    KEY_STORE.lock().take(key);
}

pub fn set_specific(key: Key, data: Option<NonNull<()>>) {
    let current = rt::current();
    if let Some(data) = data {
        current.tcb().local_store.set(key, data);
    } else {
        current.tcb().local_store.delete(key);
    }
}

pub fn get_specific(key: Key) -> Option<NonNull<()>> {
    rt::current().tcb().local_store.get(key)
}

#[derive(Debug)]
struct KeyNode {
    link: PauseCell<Link>,
    key: Key,
    data: NonNull<()>,
}

struct KeyNodeAdapter;

unsafe impl linked_list::Adapter for KeyNodeAdapter {
    type Ptr = Box<KeyNode>;
    type Value = KeyNode;

    fn offset_of_link(&self) -> usize {
        mem::offset_of!(KeyNode, link)
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<KeyNode>) -> Box<KeyNode> {
        unsafe { Box::from_raw(raw.as_ptr()) }
    }

    fn ptr_to_raw(&self, ptr: Box<KeyNode>) -> NonNull<KeyNode> {
        unsafe { NonNull::new_unchecked(Box::into_raw(ptr)) }
    }
}

#[derive(Debug)]
pub(super) struct LocalStore {
    list: RefCell<LinkedList<KeyNodeAdapter>>,
}

impl LocalStore {
    pub const fn new() -> Self {
        Self {
            list: RefCell::new(LinkedList::new(KeyNodeAdapter)),
        }
    }

    pub fn get(&self, key: Key) -> Option<NonNull<()>> {
        self.list
            .borrow()
            .iter()
            .find(|node| node.key == key)
            .map(|node| node.data)
    }

    pub fn set(&self, key: Key, data: NonNull<()>) {
        let mut list = self.list.borrow_mut();
        let node = list.iter_mut().find(|node| node.key == key);
        if let Some(node) = node {
            node.data = data;
        } else {
            list.push_back(Box::new(KeyNode {
                link: PauseCell::new(Link::unlinked()),
                key,
                data,
            }));
        }
    }

    pub fn delete(&self, key: Key) {
        let mut list = self.list.borrow_mut();
        let mut cursor = list.cursor_front_mut();
        while let Some(value) = cursor.current() {
            if value.key == key {
                cursor.remove_current();
                break;
            }

            cursor.move_next();
        }
    }

    pub fn run_dtors(&self) {
        while let Some(node) = self.list.borrow_mut().pop_front() {
            if let Some(dtor) = KEY_STORE.lock().get(node.key) {
                (dtor)(node.data)
            }
        }
    }
}
