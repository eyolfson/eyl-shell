#![allow(non_camel_case_types)]

use libc::{c_char, c_void, uint32_t};

use raw::types::objects;

#[repr(C)]
pub struct wl_registry_listener {
    pub global: extern fn(
        data: *mut c_void,
        wl_registry: *mut objects::wl_registry,
        name: uint32_t,
        interface: *const c_char,
        version: uint32_t
    ),
    pub global_remove: extern fn(
        data: *mut c_void,
        wl_registry: *mut objects::wl_registry,
        name: uint32_t
    )
}

#[repr(C)]
pub struct wl_shm_listener {
    pub format: extern fn(
        data: *mut c_void,
        wl_shm: *mut objects::wl_shm,
        format: uint32_t
    )
}
