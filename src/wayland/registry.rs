use std::c_str;
use std::ptr;

use display::Display;
use raw;

pub struct Registry {
    ptr: *mut raw::wl_registry
}

#[allow(unused_variables)]
extern fn global(
    data: *mut raw::c_void,
    wl_registry: *mut raw::wl_registry,
    name: raw::uint32_t,
    interface: *const raw::c_char,
    version: raw::uint32_t
) {
    let n = unsafe {c_str::CString::new(interface, false)};
    match n.as_str() {
        Some(ref x) => println!("{}", x),
        None => fail!("cannot convert interface to str")
    }
}

#[allow(unused_variables)]
extern fn global_remove(
    data: *mut raw::c_void,
    wl_registry: *mut raw::wl_registry,
    name: raw::uint32_t
) {

}
static REGISTRY_LISTENER: raw::protocol::wl_registry_listener =
    raw::protocol::wl_registry_listener {
        global: global,
        global_remove: global_remove
    };

impl Registry {
    pub fn new(display: &mut Display) -> Registry {
        unsafe {
            let ptr = raw::protocol::wl_display_get_registry(display.ptr());
            Registry { ptr: ptr }
        }
    }
    pub fn add_listener(&mut self) {
        unsafe {
            raw::protocol::wl_registry_add_listener(
                self.ptr,
                &REGISTRY_LISTENER,
                ptr::null_mut()
            );
        }
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        unsafe {
            raw::protocol::wl_registry_destroy(self.ptr);
        }
    }
}
