use core::time::Duration;

use crate::portability::{self, Impl};

pub fn get_time() -> Duration {
    portability::GlobalImpl::get_time()
}
