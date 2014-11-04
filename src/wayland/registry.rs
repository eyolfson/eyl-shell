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
    data: *mut raw::c_void,
    registry: *mut raw::wl_registry,
    name: raw::uint32_t,
    interface: *const raw::c_char,
    version: raw::uint32_t
) {
    unsafe {
        let r: &mut Registry = mem::transmute(data);
        if raw::strcmp(interface, raw::wl_compositor_interface.name) == 0 {
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
    data: *mut raw::c_void,
    registry: *mut raw::wl_registry,
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
