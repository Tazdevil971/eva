use alloc::boxed::Box;
use core::cell::RefCell;
use core::mem;
use core::ptr::NonNull;

use crate::rt;
use crate::rt::pause::PauseCell;
use crate::rt::sync::Mutex;
use crate::utils::linked_list::{self, Link, LinkedList};
use crate::utils::slot_map::{SlotId, SlotMap};

pub use eva_abi::error::TlsError;
pub use eva_abi::{TlsDtor, TlsKey};

static KEY_STORE: Mutex<SlotMap<Option<TlsDtor>>> = Mutex::new(SlotMap::new());

#[unsafe(export_name = "eva_rt_tls_key_create")]
pub fn key_create(dtor: Option<TlsDtor>) -> TlsKey {
    TlsKey(KEY_STORE.lock().insert(dtor))
}

#[unsafe(export_name = "eva_rt_tls_key_delete")]
pub fn key_delete(key: TlsKey) -> Result<(), TlsError> {
    match KEY_STORE.lock().take(key.0) {
        Some(_) => Ok(()),
        None => Err(TlsError::InvalidKey),
    }
}

#[unsafe(export_name = "eva_rt_tls_set_specific")]
pub fn set_specific(key: TlsKey, data: Option<NonNull<()>>) -> Result<(), TlsError> {
    if !KEY_STORE.lock().exists(key.0) {
        return Err(TlsError::InvalidKey);
    }

    let current = rt::current_raw();
    if let Some(data) = data {
        current.local_store.set(key.0, data);
    } else {
        current.local_store.delete(key.0);
    }

    Ok(())
}

#[unsafe(export_name = "eva_rt_tls_get_specific")]
pub fn get_specific(key: TlsKey) -> Option<NonNull<()>> {
    rt::current_raw().local_store.get(key.0)
}

#[derive(Debug)]
struct KeyNode {
    link: PauseCell<Link>,
    key: SlotId,
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

    pub fn get(&self, key: SlotId) -> Option<NonNull<()>> {
        self.list
            .borrow()
            .iter()
            .find(|node| node.key == key)
            .map(|node| node.data)
    }

    pub fn set(&self, key: SlotId, data: NonNull<()>) {
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

    pub fn delete(&self, key: SlotId) {
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
                if let Some(dtor) = dtor {
                    (dtor)(node.data.as_ptr())
                }
            }
        }
    }
}
