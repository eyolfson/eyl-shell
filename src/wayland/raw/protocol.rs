#![allow(non_camel_case_types)]

use std::ptr;

use raw;

use super::{c_char, c_int, c_void, uint32_t};

static WL_COMPOSITOR_CREATE_SURFACE: uint32_t = 0;
static WL_COMPOSITOR_CREATE_REGION: uint32_t = 1;
static WL_DISPLAY_GET_REGISTRY: uint32_t = 1;
static WL_REGION_DESTROY: uint32_t = 0;
static WL_REGION_ADD: uint32_t = 1;
static WL_REGION_SUBTRACT: uint32_t = 2;
static WL_REGISTRY_BIND: uint32_t = 0;
static WL_SURFACE_DESTROY: uint32_t = 0;
static WL_SURFACE_ATTACH: uint32_t = 1;
static WL_SURFACE_DAMAGE: uint32_t = 2;
static WL_SURFACE_FRAME: uint32_t = 3;
static WL_SURFACE_SET_OPAQUE_REGION: uint32_t = 4;
static WL_SURFACE_SET_INPUT_REGION: uint32_t = 5;
static WL_SURFACE_COMMIT: uint32_t = 6;
static WL_SURFACE_SET_BUFFER_TRANSFORM: uint32_t = 7;
static WL_SURFACE_SET_BUFFER_SCALE: uint32_t = 8;

#[repr(C)] pub struct wl_compositor;
#[repr(C)] pub struct wl_region;
#[repr(C)] pub struct wl_registry;
#[repr(C)] pub struct wl_surface;

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

#[inline]
pub unsafe fn wl_compositor_create_surface(
    compositor: *mut wl_compositor
) -> *mut wl_surface {
    let id = raw::wl_proxy_marshal_constructor(
        compositor as *mut raw::wl_proxy,
        WL_COMPOSITOR_CREATE_SURFACE,
        & raw::wl_surface_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut wl_surface
}

#[inline]
pub unsafe fn wl_compositor_create_region(
    compositor: *mut wl_compositor
) -> *mut wl_region {
    let id = raw::wl_proxy_marshal_constructor(
        compositor as *mut raw::wl_proxy,
        WL_COMPOSITOR_CREATE_REGION,
        & raw::wl_region_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut wl_region
}

#[inline]
pub unsafe fn wl_compositor_destroy(compositor: *mut wl_compositor) {
    raw::wl_proxy_destroy(compositor as *mut raw::wl_proxy);
}

#[inline]
pub unsafe fn wl_display_get_registry(
    display: *mut raw::wl_display
) -> *mut raw::wl_registry {
    let registry = raw::wl_proxy_marshal_constructor(
        display as *mut raw::wl_proxy,
        WL_DISPLAY_GET_REGISTRY,
        & raw::wl_registry_interface,
        ptr::null_mut::<c_void>()
    );
    registry as *mut raw::wl_registry
}

#[inline]
pub unsafe fn wl_registry_destroy(registry: *mut raw::wl_registry) {
    raw::wl_proxy_destroy(registry as *mut raw::wl_proxy);
}

#[inline]
pub unsafe fn wl_registry_add_listener(
    registry: *mut raw::wl_registry,
    listener: *const wl_registry_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        registry as *mut raw::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline]
pub unsafe fn wl_registry_bind(
    registry: *mut raw::wl_registry,
    name: uint32_t,
    interface: *const raw::util::wl_interface,
    version: uint32_t
) -> *mut c_void {
    let id = raw::wl_proxy_marshal_constructor(
        registry as *mut raw::wl_proxy,
        WL_REGISTRY_BIND,
        interface,
        name,
        (*interface).name,
        version,
        ptr::null_mut::<c_void>()
    );
    id as *mut c_void
}

#[inline]
pub unsafe fn wl_surface_destroy(surface: *mut wl_surface) {
    raw::wl_proxy_marshal(surface as *mut raw::wl_proxy, WL_SURFACE_DESTROY);
    raw::wl_proxy_destroy(surface as *mut raw::wl_proxy);
}

#[inline]
pub unsafe fn wl_region_destroy(region: *mut wl_region) {
    raw::wl_proxy_marshal(region as *mut raw::wl_proxy, WL_REGION_DESTROY);
    raw::wl_proxy_destroy(region as *mut raw::wl_proxy);
}
