pub use raw::protocol::wl_buffer::*;
pub use raw::protocol::wl_compositor::*;
pub use raw::protocol::wl_display::*;
pub use raw::protocol::wl_region::*;
pub use raw::protocol::wl_registry::*;
pub use raw::protocol::wl_seat::*;
pub use raw::protocol::wl_shell::*;
pub use raw::protocol::wl_shm::*;
pub use raw::protocol::wl_shm_pool::*;
pub use raw::protocol::wl_surface::*;
pub use raw::types::enums::*;
pub use raw::types::listeners::*;
pub use raw::types::objects::*;
pub use raw::types::utils::*;

use libc::{c_char, c_int, c_void, uint32_t};

pub mod protocol;
pub mod types;

#[link(name = "wayland-client")]
extern {
    pub static wl_buffer_interface: wl_interface;
    pub static wl_callback_interface: wl_interface;
    pub static wl_compositor_interface: wl_interface;
    pub static wl_data_device_interface: wl_interface;
    pub static wl_data_device_manager_interface: wl_interface;
    pub static wl_data_offer_interface: wl_interface;
    pub static wl_data_source_interface: wl_interface;
    pub static wl_display_interface: wl_interface;
    pub static wl_keyboard_interface: wl_interface;
    pub static wl_output_interface: wl_interface;
    pub static wl_pointer_interface: wl_interface;
    pub static wl_region_interface: wl_interface;
    pub static wl_registry_interface: wl_interface;
    pub static wl_seat_interface: wl_interface;
    pub static wl_shell_interface: wl_interface;
    pub static wl_shell_surface_interface: wl_interface;
    pub static wl_shm_interface: wl_interface;
    pub static wl_shm_pool_interface: wl_interface;
    pub static wl_subcompositor_interface: wl_interface;
    pub static wl_subsurface_interface: wl_interface;
    pub static wl_surface_interface: wl_interface;
    pub static wl_touch_interface: wl_interface;

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
        interface: *const wl_interface,
        ...
    ) -> *mut wl_proxy;
    pub fn wl_proxy_set_user_data(proxy: *mut wl_proxy, user_data: *mut c_void);
    pub fn wl_proxy_get_user_data(proxy: *mut wl_proxy) -> *mut c_void;
    // wl_list
    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(elm: *const wl_list) -> c_int;
    pub fn wl_list_empty(elm: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
}
