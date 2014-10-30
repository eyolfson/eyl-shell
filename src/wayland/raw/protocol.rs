#![allow(non_camel_case_types)]

use std::ptr;

use raw;

use super::libc::{c_void, uint32_t};

static WL_DISPLAY_GET_REGISTRY: uint32_t = 1;

pub unsafe fn wl_display_get_registry(display: *mut raw::wl_display)
                                      -> *mut raw::wl_registry {
    let registry = raw::wl_proxy_marshal_constructor(
        display as *mut raw::wl_proxy,
        WL_DISPLAY_GET_REGISTRY,
        & raw::wl_registry_interface,
        ptr::null::<*mut c_void>()
    );
    registry as *mut raw::wl_registry
}

pub unsafe fn wl_registry_destroy(wl_registry: *mut raw::wl_registry) {
    raw::wl_proxy_destroy(wl_registry as *mut raw::wl_proxy);
}

// pub fn wl_registry_add_listener(registry: *mut raw::wl_registry,
//                                 listener: *const wl_registry_listener,
//                                 data: *mut c_void) -> c_int {
//     return 0;
// }
