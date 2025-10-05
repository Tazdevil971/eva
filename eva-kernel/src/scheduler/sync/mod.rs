pub mod raw_mutex;

pub mod mutex {
    use super::*;

    pub type Mutex<T> = lock_api::Mutex<raw_mutex::RawMutex, T>;
}
