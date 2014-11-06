use libc::{c_char, c_void, int32_t, uint32_t};

use raw::types::objects;

#[repr(C)]
pub struct wl_buffer_listener {
    pub release: extern fn(
        data: *mut c_void,
        wl_buffer: *mut objects::wl_buffer
    )
}

#[repr(C)]
pub struct wl_output_listener {
    pub geometry: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output,
        x: int32_t,
        y: int32_t,
        physical_width: int32_t,
        physical_height: int32_t,
        subpixel: int32_t,
        make: *const c_char,
        model: *const c_char,
        transform: int32_t
    ),
    pub mode: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output,
        flags: uint32_t,
        width: int32_t,
        height: int32_t,
        refresh: int32_t
    ),
    pub done: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output
    ),
    pub scale: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output,
        factor: int32_t
    )
}

#[repr(C)]
pub struct wl_registry_listener {
    pub global: extern fn(
        data: *mut c_void,
        wl_registry: *mut objects::wl_registry,
        name: uint32_t,
        interface: *const c_char,
        version: uint32_t
    ),
    pub global_remove: extern fn(
        data: *mut c_void,
        wl_registry: *mut objects::wl_registry,
        name: uint32_t
    )
}

#[repr(C)]
pub struct wl_seat_listener {
    pub capabilities: extern fn(
        data: *mut c_void,
        wl_seat: *mut objects::wl_seat,
        capabilities: uint32_t
    ),
    pub name: extern fn(
        data: *mut c_void,
        wl_seat: *mut objects::wl_seat,
        name: *const c_char
    )
}

#[repr(C)]
pub struct wl_shm_listener {
    pub format: extern fn(
        data: *mut c_void,
        wl_shm: *mut objects::wl_shm,
        format: uint32_t
    )
}
