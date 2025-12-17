use core::time::Duration;

use crate::port::{self, Impl};

#[unsafe(export_name = "eva_get_time")]
pub fn get_time() -> Duration {
    port::GlobalImpl::get_time()
}
