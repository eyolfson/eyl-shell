use libc::{c_int, c_void, int32_t, uint32_t};

use raw;
use raw::types::listeners;
use raw::types::objects;

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
pub unsafe fn wl_surface_add_listener(
    wl_surface: *mut objects::wl_surface,
    listener: *const listeners::wl_surface_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_surface as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline]
pub unsafe fn wl_surface_set_user_data(
    wl_surface: *mut objects::wl_surface,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_surface as *mut objects::wl_proxy, user_data
    );
}

#[inline]
pub unsafe fn wl_surface_get_user_data(
    wl_surface: *mut objects::wl_surface
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_surface as *mut objects::wl_proxy)
}

#[inline]
pub unsafe fn wl_surface_destroy(wl_surface: *mut objects::wl_surface) {
    raw::wl_proxy_marshal(
        wl_surface as *mut objects::wl_proxy,
        WL_SURFACE_DESTROY
    );
    raw::wl_proxy_destroy(wl_surface as *mut objects::wl_proxy);
}

#[inline]
pub unsafe fn wl_surface_attach(
    wl_surface: *mut objects::wl_surface,
    buffer: *mut objects::wl_buffer,
    x: int32_t,
    y: int32_t
) {
    raw::wl_proxy_marshal(
        wl_surface as *mut objects::wl_proxy,
        WL_SURFACE_ATTACH,
        buffer,
        x,
        y
    );
}

#[inline]
pub unsafe fn wl_surface_commit(
    wl_surface: *mut objects::wl_surface,
) {
    raw::wl_proxy_marshal(
        wl_surface as *mut objects::wl_proxy,
        WL_SURFACE_COMMIT,
    );
}
