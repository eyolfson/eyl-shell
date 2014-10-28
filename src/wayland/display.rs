use std::ptr;

use raw;

pub struct Display {
    ptr: *mut raw::wl_display
}

impl Display {
    pub fn new() -> Display {
        unsafe {
            let ptr = raw::wl_display_connect(ptr::null());
            assert!(!ptr.is_null());
            Display { ptr: ptr }
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            raw::wl_display_disconnect(self.ptr);
        }
    }
}
