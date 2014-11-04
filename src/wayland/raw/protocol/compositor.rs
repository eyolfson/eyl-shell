#![allow(non_camel_case_types)]

use std::ptr;

use raw::{
    c_void,
    wl_proxy,
    wl_proxy_destroy,
    wl_proxy_marshal_constructor,
    wl_region_interface,
    wl_surface_interface,
    uint32_t,
};
use raw::protocol::region::wl_region;
use raw::protocol::surface::wl_surface;

#[repr(C)]
pub struct wl_compositor;

pub static WL_COMPOSITOR_CREATE_SURFACE: uint32_t = 0;
pub static WL_COMPOSITOR_CREATE_REGION: uint32_t = 1;

#[inline]
pub unsafe fn wl_compositor_create_surface(
    compositor: *mut wl_compositor
) -> *mut wl_surface {
    let id = wl_proxy_marshal_constructor(
        compositor as *mut wl_proxy,
        WL_COMPOSITOR_CREATE_SURFACE,
        & wl_surface_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut wl_surface
}

#[inline]
pub unsafe fn wl_compositor_create_region(
    compositor: *mut wl_compositor
) -> *mut wl_region {
    let id = wl_proxy_marshal_constructor(
        compositor as *mut wl_proxy,
        WL_COMPOSITOR_CREATE_REGION,
        & wl_region_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut wl_region
}

#[inline]
pub unsafe fn wl_compositor_destroy(compositor: *mut wl_compositor) {
    wl_proxy_destroy(compositor as *mut wl_proxy);
}
