use core::time::Duration;

use crate::port::{self, Impl};

pub fn get_time() -> Duration {
    port::GlobalImpl::get_time()
}
