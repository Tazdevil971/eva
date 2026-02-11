use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::ptr::{self, NonNull};

use crate::rt;
use crate::rt::sync::Mutex;

use eva_abi::OsError;
pub use eva_abi::{TlsDtor, TlsKey};
use eva_utils::linked_list::{self, LinkedList};
use eva_utils::slot_map::SlotMap;

static KEY_STORE: Mutex<SlotMap<Option<TlsDtor>>> = Mutex::new(SlotMap::new());

#[unsafe(export_name = "eva_rt_tls_key_create")]
pub fn key_create(dtor: Option<TlsDtor>) -> TlsKey {
    TlsKey(KEY_STORE.lock().insert(dtor))
}

#[unsafe(export_name = "eva_rt_tls_key_delete")]
pub fn key_delete(key: TlsKey) -> Result<(), OsError> {
    match KEY_STORE.lock().take(key.0) {
        Some(_) => Ok(()),
        None => Err(OsError::InvalidTlsKey),
    }
}

fn key_exists(key: TlsKey) -> bool {
    KEY_STORE.lock().exists(key.0)
}

fn key_run_dtor(key: TlsKey, data: NonNull<()>) {
    // NOTE: We want to keep the lock short lived to prevent deadlocks
    let dtor = KEY_STORE.lock().get(key.0).copied().flatten();
    if let Some(dtor) = dtor {
        (dtor)(data.as_ptr());
    }
}

#[unsafe(export_name = "eva_rt_tls_set_specific")]
pub fn set_specific(key: TlsKey, data: Option<NonNull<()>>) -> Result<(), OsError> {
    let local = rt::local_raw();
    if let Some(data) = data {
        local.store.set(key, data);
    } else {
        local.store.delete(key);
    }

    Ok(())
}

#[unsafe(export_name = "eva_rt_tls_get_specific")]
pub fn get_specific(key: TlsKey) -> Option<NonNull<()>> {
    rt::local_raw().store.get(key)
}

#[derive(Debug)]
struct KeyNode {
    link: linked_list::Link<Self>,
    key: TlsKey,
    data: NonNull<()>,
}

struct KeyNodeAdapter;

impl linked_list::Adapter for KeyNodeAdapter {
    type Ptr = Box<KeyNode>;
    type Value = KeyNode;
    type Link = linked_list::Link<KeyNode>;

    unsafe fn raw_to_link(&self, raw: NonNull<Self::Value>) -> NonNull<Self::Link> {
        unsafe {
            let ptr = ptr::addr_of_mut!((*raw.as_ptr()).link);
            NonNull::new_unchecked(ptr)
        }
    }

    unsafe fn ptr_from_raw(&self, raw: NonNull<KeyNode>) -> Box<KeyNode> {
        unsafe { Box::from_raw(raw.as_ptr()) }
    }

    fn ptr_to_raw(&self, ptr: Box<KeyNode>) -> NonNull<KeyNode> {
        unsafe { NonNull::new_unchecked(Box::into_raw(ptr)) }
    }
}

unsafe impl linked_list::OwningAdapter for KeyNodeAdapter {}

#[derive(Debug)]
pub(super) struct LocalStore {
    list: UnsafeCell<LinkedList<KeyNodeAdapter>>,
}

impl LocalStore {
    pub const fn new() -> Self {
        Self {
            list: UnsafeCell::new(LinkedList::new(KeyNodeAdapter)),
        }
    }

    pub fn get(&self, key: TlsKey) -> Option<NonNull<()>> {
        unsafe {
            self.list
                .get()
                .as_mut()
                .unwrap_unchecked()
                .iter()
                .find(|node| node.key == key)
                .map(|node| node.data)
        }
    }

    pub fn set(&self, key: TlsKey, data: NonNull<()>) {
        // First check if the key exists
        if !key_exists(key) {
            return;
        }

        let list = unsafe { self.list.get().as_mut().unwrap_unchecked() };
        let node = list.iter_mut().find(|node| node.key == key);
        if let Some(node) = node {
            node.data = data;
        } else {
            list.push_back(Box::new(KeyNode {
                link: linked_list::Link::unlinked(),
                key,
                data,
            }));
        }
    }

    pub fn delete(&self, key: TlsKey) {
        let list = unsafe { self.list.get().as_mut().unwrap_unchecked() };
        let mut cursor = list.cursor_front_mut();
        while let Some(value) = cursor.current() {
            if value.key == key {
                cursor.remove_current();
                break;
            }

            cursor.move_next();
        }
    }

    fn pop_node(&self) -> Option<Box<KeyNode>> {
        unsafe { self.list.get().as_mut().unwrap_unchecked().pop_front() }
    }

    pub fn run_dtors(&self) {
        while let Some(node) = self.pop_node() {
            key_run_dtor(node.key, node.data);
        }
    }
}
