use libc::{c_int, c_void};

use raw;
use raw::types::objects;
use raw::types::listeners;

#[inline]
pub unsafe fn wl_shm_add_listener(
    wl_shm: *mut objects::wl_shm,
    listener: *const listeners::wl_shm_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_shm as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline]
pub unsafe fn wl_shm_destroy(wl_shm: *mut objects::wl_shm) {
    raw::wl_proxy_destroy(wl_shm as *mut objects::wl_proxy);
}
