#![allow(non_camel_case_types)]

use std::ptr;

use raw::{
    c_char,
    c_int,
    c_void,
    wl_interface,
    wl_proxy,
    wl_proxy_add_listener,
    wl_proxy_destroy,
    wl_proxy_marshal_constructor,
    uint32_t,
};

pub static WL_REGISTRY_BIND: uint32_t = 0;

#[repr(C)]
pub struct wl_registry;

#[repr(C)]
pub struct wl_registry_listener {
    pub global: extern fn(
        data: *mut c_void,
        wl_registry: *mut wl_registry,
        name: uint32_t,
        interface: *const c_char,
        version: uint32_t
    ),
    pub global_remove: extern fn(
        data: *mut c_void,
        wl_registry: *mut wl_registry,
        name: uint32_t
    )
}

#[inline]
pub unsafe fn wl_registry_destroy(registry: *mut wl_registry) {
    wl_proxy_destroy(registry as *mut wl_proxy);
}

#[inline]
pub unsafe fn wl_registry_add_listener(
    registry: *mut wl_registry,
    listener: *const wl_registry_listener,
    data: *mut c_void
) -> c_int {
    wl_proxy_add_listener(
        registry as *mut wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline]
pub unsafe fn wl_registry_bind(
    registry: *mut wl_registry,
    name: uint32_t,
    interface: *const wl_interface,
    version: uint32_t
) -> *mut c_void {
    let id = wl_proxy_marshal_constructor(
        registry as *mut wl_proxy,
        WL_REGISTRY_BIND,
        interface,
        name,
        (*interface).name,
        version,
        ptr::null_mut::<c_void>()
    );
    id as *mut c_void
}
