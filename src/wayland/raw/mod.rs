#![allow(non_camel_case_types)]

extern crate libc;

pub use self::libc::{c_char, c_int, c_void, uint32_t};

pub mod protocol;
pub mod util;

#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

pub type wl_compositor = c_void;
pub type wl_display = c_void;
pub type wl_event_queue = c_void;
pub type wl_proxy = c_void;
pub type wl_registry = c_void;

#[link(name = "wayland-client")]
extern {
    pub static wl_registry_interface: util::wl_interface;

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
    pub fn wl_display_roundtrip_queue(display: *mut wl_display,
                                      queue: *mut wl_event_queue) -> c_int;
    pub fn wl_display_roundtrip(display: *mut wl_display) -> c_int;
    pub fn wl_proxy_add_listener(
        proxy: *mut wl_proxy,
        implementation: *mut extern fn(c_void) -> c_void,
        data: *mut c_void) -> c_int;
    pub fn wl_proxy_destroy(proxy: *mut wl_proxy);
    pub fn wl_proxy_marshal_constructor(proxy: *mut wl_proxy,
                                        opcode: uint32_t,
                                        interface: *const util::wl_interface,
                                        ...) -> *mut wl_proxy;

    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(elm: *const wl_list) -> c_int;
    pub fn wl_list_empty(elm: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
}
