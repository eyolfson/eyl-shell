use raw;

pub struct Shm {
    ptr: *mut raw::wl_shm
}

impl Shm {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shm) -> Shm {
        Shm { ptr: ptr }
    }
}

impl Drop for Shm {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shm_destroy(self.ptr)
        }
    }
}
