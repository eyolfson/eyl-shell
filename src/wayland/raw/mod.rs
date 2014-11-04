#![allow(non_camel_case_types)]

extern crate libc;

pub use raw::libc::{c_char, c_int, c_void, size_t, strcmp, uint32_t};

pub use raw::protocol::{
    wl_compositor,
    wl_compositor_create_surface,
    wl_compositor_destroy,
};
pub use raw::protocol::registry::{
    wl_registry,
    wl_registry_add_listener,
    wl_registry_bind,
    wl_registry_destroy,
    wl_registry_listener,
};
pub use raw::protocol::{
    wl_display_get_registry,
};
pub use raw::protocol::{
    wl_surface,
    wl_surface_destroy,
};
pub use raw::protocol::{
    wl_region,
    wl_region_destroy,
};

pub use raw::util::{wl_array, wl_interface, wl_list, wl_message};

pub mod protocol;
pub mod util;

#[repr(C)] pub struct wl_display;
#[repr(C)] pub struct wl_event_queue;
#[repr(C)] pub struct wl_proxy;

#[link(name = "wayland-client")]
extern {
    pub static wl_compositor_interface: util::wl_interface;
    pub static wl_region_interface: util::wl_interface;
    pub static wl_registry_interface: util::wl_interface;
    pub static wl_shell_interface: util::wl_interface;
    pub static wl_surface_interface: util::wl_interface;

    pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
    pub fn wl_display_disconnect(display: *mut wl_display);
    pub fn wl_display_dispatch(display: *mut wl_display) -> c_int;
    pub fn wl_display_dispatch_queue(
        display: *mut wl_display,
        queue: *mut wl_event_queue
    ) -> c_int;
    pub fn wl_display_dispatch_queue_pending(
        display: *mut wl_display,
        queue: *mut wl_event_queue
    ) -> c_int;
    pub fn wl_display_dispatch_pending(display: *mut wl_display) -> c_int;
    pub fn wl_display_get_error(display: *mut wl_display) -> c_int;
    pub fn wl_display_flush(display: *mut wl_display) -> c_int;
    pub fn wl_display_roundtrip_queue(
        display: *mut wl_display,
        queue: *mut wl_event_queue
    ) -> c_int;
    pub fn wl_display_roundtrip(display: *mut wl_display) -> c_int;
    // wl_proxy
    pub fn wl_proxy_add_listener(
        proxy: *mut wl_proxy,
        implementation: *mut extern fn(),
        data: *mut c_void
    ) -> c_int;
    pub fn wl_proxy_destroy(proxy: *mut wl_proxy);
    pub fn wl_proxy_marshal(
        p: *mut wl_proxy,
        opcode: uint32_t,
        ...
    );
    pub fn wl_proxy_marshal_constructor(
        proxy: *mut wl_proxy,
        opcode: uint32_t,
        interface: *const util::wl_interface,
        ...
    ) -> *mut wl_proxy;
    // wl_list
    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(elm: *const wl_list) -> c_int;
    pub fn wl_list_empty(elm: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
}
