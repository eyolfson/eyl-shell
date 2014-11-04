#![allow(non_camel_case_types)]

use std::ptr;

use libc::{c_void, uint32_t};

use raw;
use raw::types::objects;

pub static WL_DISPLAY_SYNC: uint32_t = 0;
pub static WL_DISPLAY_GET_REGISTRY: uint32_t = 1;

#[inline]
pub unsafe fn wl_display_get_registry(
    wl_display: *mut objects::wl_display
) -> *mut objects::wl_registry {
    let registry = raw::wl_proxy_marshal_constructor(
        wl_display as *mut objects::wl_proxy,
        WL_DISPLAY_GET_REGISTRY,
        & raw::wl_registry_interface,
        ptr::null_mut::<c_void>()
    );
    registry as *mut objects::wl_registry
}
