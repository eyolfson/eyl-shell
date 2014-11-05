use libc::uint32_t;

use raw;
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
pub unsafe fn wl_surface_destroy(wl_surface: *mut objects::wl_surface) {
    raw::wl_proxy_marshal(
        wl_surface as *mut objects::wl_proxy,
        WL_SURFACE_DESTROY
    );
    raw::wl_proxy_destroy(wl_surface as *mut objects::wl_proxy);
}
