#![allow(non_camel_case_types)]

extern crate libc;

use self::libc::{c_char, c_int, c_void};

#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

pub type wl_display = c_void;
pub type wl_event_queue = c_void;
pub type wl_registry = c_void;

#[link(name = "wayland-client")]
extern {
    pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
    pub fn wl_display_disconnect(display: *mut wl_display);

    pub fn wl_display_dispatch(display: *mut wl_display) -> c_int;
    pub fn wl_display_dispatch_queue(display: *mut wl_display,
                                     queue: *mut wl_event_queue) -> c_int;
    pub fn wl_display_dispatch_queue_pending(display: *mut wl_display,
                                             queue: *mut wl_event_queue)
                                             -> c_int;
    pub fn wl_display_dispatch_pending(display: *mut wl_display) -> c_int;
    pub fn wl_display_get_error(display: *mut wl_display) -> c_int;
    pub fn wl_display_flush(display: *mut wl_display) -> c_int;

    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(elm: *const wl_list) -> c_int;
    pub fn wl_list_empty(elm: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
}
