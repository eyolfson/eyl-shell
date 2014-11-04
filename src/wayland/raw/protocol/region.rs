#![allow(non_camel_case_types)]

use raw::{
    uint32_t,
    wl_proxy,
    wl_proxy_destroy,
    wl_proxy_marshal,
};

#[repr(C)]
pub struct wl_region;

pub static WL_REGION_DESTROY: uint32_t = 0;
pub static WL_REGION_ADD: uint32_t = 1;
pub static WL_REGION_SUBTRACT: uint32_t = 2;

#[inline]
pub unsafe fn wl_region_destroy(region: *mut wl_region) {
    wl_proxy_marshal(region as *mut wl_proxy, WL_REGION_DESTROY);
    wl_proxy_destroy(region as *mut wl_proxy);
}
