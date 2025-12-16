pub mod raw_mutex;
pub mod condvar;

pub use condvar::Condvar;
pub type Mutex<T> = lock_api::Mutex<raw_mutex::RawMutex, T>;
