use raw;

pub struct ShmPool {
    ptr: *mut raw::wl_shm_pool
}

impl ShmPool {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shm_pool) -> ShmPool {
        ShmPool { ptr: ptr }
    }
}

impl Drop for ShmPool {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shm_pool_destroy(self.ptr)
        }
    }
}
