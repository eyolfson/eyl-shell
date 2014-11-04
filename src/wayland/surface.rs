use raw;

pub struct Surface {
    ptr: *mut raw::wl_surface
}

impl Surface {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_surface) -> Surface {
        Surface { ptr: ptr }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            raw::wl_surface_destroy(self.ptr)
        }
    }
}
