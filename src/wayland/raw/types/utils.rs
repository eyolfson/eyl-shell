use libc::{c_char, c_int, c_void, int32_t, size_t, uint32_t};

#[repr(C)]
pub enum wl_argument {
    i(int32_t),
    u(uint32_t),
    f(wl_fixed_t),
    s(*const c_char),
    o(*mut wl_object),
    n(uint32_t),
    a(*mut wl_array),
    h(int32_t),
}

#[repr(C)]
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut c_void,
}

#[repr(C)]
pub type wl_dispatcher_func_t = extern fn(
    _: *const c_void,
    _: *mut c_void,
    _: uint32_t,
    _: *const wl_message,
    _: *mut wl_argument);

#[repr(C)]
pub type wl_fixed_t = uint32_t;

#[repr(C)]
pub struct wl_interface {
    pub name: *const c_char,
    pub version: c_int,
    pub method_count: c_int,
    pub methods: *const wl_message,
    pub event_count: c_int,
    pub events: *const wl_message,
}

#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

#[repr(C)]
pub type wl_log_func_t = extern fn(_: *const c_char, ...);

#[repr(C)]
pub struct wl_message {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub types: *mut *const wl_interface,
}

#[repr(C)]
pub struct wl_object;
