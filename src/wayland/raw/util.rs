#![allow(non_camel_case_types)]

use super::{c_char, c_int};

#[repr(C)]
struct wl_message {
    name: *const c_char,
    signature: *const c_char,
    types: *mut *const wl_interface
}

#[repr(C)]
pub struct wl_interface {
    name: *const c_char,
    version: c_int,
    method_count: c_int,
    methods: *const wl_message,
    event_count: c_int,
    events: *const wl_message
}
