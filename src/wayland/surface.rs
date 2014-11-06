use raw;

pub struct Surface {
    ptr: *mut raw::wl_surface
}

impl Surface {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_surface) -> Surface {
        Surface { ptr: ptr }
    }
    pub unsafe fn to_ptr(&mut self) -> *mut raw::wl_surface {
        self.ptr
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            raw::wl_surface_destroy(self.ptr)
        }
    }
}
