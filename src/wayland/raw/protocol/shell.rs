use raw::{wl_proxy, wl_proxy_destroy};

#[repr(C)]
pub struct wl_shell;

#[inline]
pub unsafe fn wl_shell_destroy(shell: *mut wl_shell) {
    wl_proxy_destroy(shell as *mut wl_proxy);
}
