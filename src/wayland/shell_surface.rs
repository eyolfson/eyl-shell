use raw;

pub struct ShellSurface {
    ptr: *mut raw::wl_shell_surface
}

impl ShellSurface {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shell_surface) -> ShellSurface {
        ShellSurface { ptr: ptr }
    }
}

impl Drop for ShellSurface {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shell_surface_destroy(self.ptr)
        }
    }
}
