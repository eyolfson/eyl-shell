use raw;
use raw::types::objects;

#[inline]
pub unsafe fn wl_shell_destroy(wl_shell: *mut objects::wl_shell) {
    raw::wl_proxy_destroy(wl_shell as *mut objects::wl_proxy);
}
