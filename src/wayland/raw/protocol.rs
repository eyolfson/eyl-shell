#![allow(non_camel_case_types)]

pub fn wl_registry_add_listener(registry: *mut wl_registry,
                                listener: *const wl_registry_listener,
                                data: *mut c_void) -> c_int {
    return 0;
}
