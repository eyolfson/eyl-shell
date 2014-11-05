use libc::{uint32_t};

use raw;
use raw::types::objects;

pub static WL_REGION_DESTROY: uint32_t = 0;
pub static WL_REGION_ADD: uint32_t = 1;
pub static WL_REGION_SUBTRACT: uint32_t = 2;

#[inline]
pub unsafe fn wl_region_destroy(wl_region: *mut objects::wl_region) {
    raw::wl_proxy_marshal(
        wl_region as *mut objects::wl_proxy,
        WL_REGION_DESTROY
    );
    raw::wl_proxy_destroy(wl_region as *mut objects::wl_proxy);
}
