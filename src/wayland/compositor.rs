use raw;

pub struct Compositor {
    ptr: *mut raw::wl_compositor
}

impl Compositor {
    pub fn new(ptr: *mut raw::wl_compositor) -> Compositor {
        Compositor { ptr: ptr }
    }
}

impl Drop for Compositor {
    fn drop(&mut self) {
        unsafe {
            raw::wl_compositor_destroy(self.ptr);
        }
    }
}
