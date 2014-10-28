#![allow(non_camel_case_types)]

extern crate libc;

use self::libc::{c_char, c_void};

pub type wl_display = c_void;

#[link(name = "wayland-client")]
extern {
    pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
    pub fn wl_display_disconnect(display: *mut wl_display);
}
