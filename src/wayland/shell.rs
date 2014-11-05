use raw;

pub struct Shell {
    ptr: *mut raw::wl_shell
}

impl Shell {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shell) -> Shell {
        Shell { ptr: ptr }
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shell_destroy(self.ptr)
        }
    }
}
