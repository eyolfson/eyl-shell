#![allow(non_camel_case_types)]

use std::ptr;

use raw;

use super::{c_char, c_int, c_void, uint32_t};

static WL_DISPLAY_GET_REGISTRY: uint32_t = 1;
static WL_REGISTRY_BIND: uint32_t = 0;

#[repr(C)]
pub struct wl_registry_listener {
    pub global: extern fn(
        data: *mut c_void,
        wl_registry: *mut raw::wl_registry,
        name: uint32_t,
        interface: *const c_char,
        version: uint32_t
    ),
    pub global_remove: extern fn(
        data: *mut c_void,
        wl_registry: *mut raw::wl_registry,
        name: uint32_t
    )
}

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

pub unsafe fn wl_registry_add_listener(
    wl_registry: *mut raw::wl_registry,
    listener: *const wl_registry_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_registry as *mut raw::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

pub unsafe fn wl_registry_bind(
    wl_registry: *mut raw::wl_registry,
    name: uint32_t,
    interface: *const raw::util::wl_interface,
    version: uint32_t
) -> *mut c_void {
    let id = raw::wl_proxy_marshal_constructor(
        wl_registry as *mut raw::wl_proxy,
        WL_REGISTRY_BIND,
        interface,
        name,
        (*interface).name,
        version,
        ptr::null::<*mut c_void>()
    );
    id as *mut c_void
}
