use std::ptr;

use display::Display;
use raw;

pub struct Registry {
    ptr: *mut raw::wl_registry
}

impl Registry {
    pub fn new(display: &mut Display) -> Registry {
        unsafe {
            let ptr = raw::protocol::wl_display_get_registry(display.ptr());
            Registry { ptr: ptr }
        }
    }
}
