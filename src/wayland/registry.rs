use libc::{c_char, c_void, strcmp, uint32_t};
use std::mem;

use Display;
use Compositor;
use raw;

pub struct Registry {
    ptr: *mut raw::wl_registry,
    compositor: Option<Compositor>,
}

#[allow(unused_variables)]
extern fn global(
    data: *mut c_void,
    registry: *mut raw::wl_registry,
    name: uint32_t,
    interface: *const c_char,
    version: uint32_t
) {
    unsafe {
        let r: &mut Registry = mem::transmute(data);
        if strcmp(interface, raw::wl_compositor_interface.name) == 0 {
            let ptr = raw::wl_registry_bind(
                registry,
                name,
                & raw::wl_compositor_interface,
                version
            );
            let compositor = Compositor::from_ptr(
                ptr as *mut raw::wl_compositor
            );
            r.compositor = Some(compositor);
        }
    }
}

#[allow(unused_variables)]
extern fn global_remove(
    data: *mut c_void,
    registry: *mut raw::wl_registry,
    name: uint32_t
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
            let mut r = Registry { ptr: ptr, compositor: None };
            raw::wl_registry_add_listener(
                ptr,
                &REGISTRY_LISTENER,
                mem::transmute(&mut r)
            );
            display.roundtrip();
            r
        }
    }
    pub fn get_compositor(&mut self) -> &mut Compositor {
        match self.compositor {
            Some(ref mut c) => c,
            None => panic!("compositor not set"),
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
