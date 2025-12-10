pub mod raw_mutex;

pub type Mutex<T> = lock_api::Mutex<raw_mutex::RawMutex, T>;
