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

impl Drop for Registry {
    fn drop(&mut self) {
        unsafe {
            raw::protocol::wl_registry_destroy(self.ptr);
        }
    }
}
