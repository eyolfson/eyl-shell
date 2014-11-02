use std::c_str;
use std::ptr;

use display::Display;
use compositor::Compositor;
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
    let compositor_cmp = unsafe {
        raw::strcmp(interface, raw::wl_compositor_interface.name)
    };
    if compositor_cmp == 0 {
        unsafe {
            let compositor = raw::wl_registry_bind(
                wl_registry,
                name,
                & raw::wl_compositor_interface,
                version
            );
        let c = Compositor::from_ptr(compositor as *mut raw::wl_compositor);
        }
    }
    let interface_c_str = unsafe { c_str::CString::new(interface, false) };
    let interface_str = interface_c_str.as_str().unwrap();
    println!("interface: {}, name: {}, version: {}",
             interface_str, name, version);
}

#[allow(unused_variables)]
extern fn global_remove(
    data: *mut raw::c_void,
    wl_registry: *mut raw::wl_registry,
    name: raw::uint32_t
) {

}

static REGISTRY_LISTENER: raw::wl_registry_listener =
    raw::wl_registry_listener {
        global: global,
        global_remove: global_remove
    };

impl Registry {
    pub fn new(display: &mut Display) -> Registry {
        unsafe {
            let ptr = raw::wl_display_get_registry(display.to_ptr());
            Registry { ptr: ptr }
        }
    }
    pub fn add_listener(&mut self) {
        unsafe {
            raw::wl_registry_add_listener(
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
            raw::wl_registry_destroy(self.ptr);
        }
    }
}
