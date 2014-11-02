#![allow(non_camel_case_types)]

use std::ptr;

use raw;

use super::{c_char, c_int, c_void, uint32_t};

static WL_COMPOSITOR_CREATE_SURFACE: uint32_t = 0;
static WL_COMPOSITOR_CREATE_REGION: uint32_t = 1;
static WL_DISPLAY_GET_REGISTRY: uint32_t = 1;
static WL_REGISTRY_BIND: uint32_t = 0;

pub type wl_compositor = c_void;
pub type wl_region = c_void;
pub type wl_registry = c_void;
pub type wl_surface = c_void;

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

pub unsafe fn wl_compositor_create_surface(
    wl_compositor: *mut wl_compositor
) -> *mut wl_surface {
    let id = raw::wl_proxy_marshal_constructor(
        wl_compositor as *mut raw::wl_proxy,
        WL_COMPOSITOR_CREATE_SURFACE,
        & raw::wl_surface_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut wl_surface
}
pub unsafe fn wl_compositor_create_region(
    wl_compositor: *mut wl_compositor
) -> *mut wl_region {
    let id = raw::wl_proxy_marshal_constructor(
        wl_compositor as *mut raw::wl_proxy,
        WL_COMPOSITOR_CREATE_REGION,
        & raw::wl_region_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut wl_region
}

pub unsafe fn wl_compositor_destroy(wl_compositor: *mut wl_compositor) {
    raw::wl_proxy_destroy(wl_compositor as *mut raw::wl_proxy);
}

pub unsafe fn wl_display_get_registry(display: *mut raw::wl_display)
                                      -> *mut raw::wl_registry {
    let registry = raw::wl_proxy_marshal_constructor(
        display as *mut raw::wl_proxy,
        WL_DISPLAY_GET_REGISTRY,
        & raw::wl_registry_interface,
        ptr::null_mut::<c_void>()
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
