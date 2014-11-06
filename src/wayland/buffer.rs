use raw;

pub struct Buffer {
    ptr: *mut raw::wl_buffer
}

impl Buffer {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_buffer) -> Buffer {
        Buffer { ptr: ptr }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            raw::wl_buffer_destroy(self.ptr)
        }
    }
}
