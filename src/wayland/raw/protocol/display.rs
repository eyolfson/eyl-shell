#![allow(non_camel_case_types)]

use std::ptr;

use raw::{
    c_void,
    wl_proxy,
    wl_proxy_marshal_constructor,
    wl_registry_interface,
    uint32_t,
    wl_display,
};
use raw::protocol::registry::{wl_registry};

pub static WL_DISPLAY_GET_REGISTRY: uint32_t = 1;

#[inline]
pub unsafe fn wl_display_get_registry(
    display: *mut wl_display
) -> *mut wl_registry {
    let registry = wl_proxy_marshal_constructor(
        display as *mut wl_proxy,
        WL_DISPLAY_GET_REGISTRY,
        & wl_registry_interface,
        ptr::null_mut::<c_void>()
    );
    registry as *mut wl_registry
}
