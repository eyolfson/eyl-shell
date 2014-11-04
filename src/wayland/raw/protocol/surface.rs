#![allow(non_camel_case_types)]

use raw::{
    uint32_t,
    wl_proxy,
    wl_proxy_destroy,
    wl_proxy_marshal,
};

#[repr(C)]
pub struct wl_surface;

pub static WL_SURFACE_DESTROY: uint32_t = 0;
pub static WL_SURFACE_ATTACH: uint32_t = 1;
pub static WL_SURFACE_DAMAGE: uint32_t = 2;
pub static WL_SURFACE_FRAME: uint32_t = 3;
pub static WL_SURFACE_SET_OPAQUE_REGION: uint32_t = 4;
pub static WL_SURFACE_SET_INPUT_REGION: uint32_t = 5;
pub static WL_SURFACE_COMMIT: uint32_t = 6;
pub static WL_SURFACE_SET_BUFFER_TRANSFORM: uint32_t = 7;
pub static WL_SURFACE_SET_BUFFER_SCALE: uint32_t = 8;

#[inline]
pub unsafe fn wl_surface_destroy(surface: *mut wl_surface) {
    wl_proxy_marshal(surface as *mut wl_proxy, WL_SURFACE_DESTROY);
    wl_proxy_destroy(surface as *mut wl_proxy);
}
