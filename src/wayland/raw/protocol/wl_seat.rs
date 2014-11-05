use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub static WL_SEAT_GET_POINTER: uint32_t = 0;
pub static WL_SEAT_GET_KEYBOARD: uint32_t = 1;
pub static WL_SEAT_GET_TOUCH: uint32_t = 2;

#[inline]
pub unsafe fn wl_seat_add_listener(
    wl_seat: *mut objects::wl_seat,
    listener: *const listeners::wl_seat_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_seat as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline]
pub unsafe fn wl_seat_destroy(wl_seat: *mut objects::wl_seat) {
    raw::wl_proxy_destroy(wl_seat as *mut objects::wl_proxy);
}
