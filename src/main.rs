mod raw {
    extern crate libc;

    use self::libc::{c_char, c_void};

    #[allow(non_camel_case_types)]
    pub type wl_display = c_void;

    #[link(name = "wayland-client")]
    extern {
        pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
        pub fn wl_display_disconnect(display: *mut wl_display);
    }
}

pub struct Display {
    ptr: *mut raw::wl_display
}

impl Display {
    pub fn new() -> Display {
        unsafe {
            let ptr = raw::wl_display_connect(std::ptr::null());
            assert!(!ptr.is_null());
            Display { ptr: ptr }
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            raw::wl_display_disconnect(self.ptr);
        }
    }
}

fn main() {
    let logfile_path = match std::os::homedir() {
        Some(ref p) => p.join(".eyl-shell.log"),
        None        => fail!("finding homedir")
    };
    let mut logfile = std::io::File::create(&logfile_path);
    let display = Display::new();
    match logfile.write(b"Connected to display\n") {
        Ok(()) => (),
        Err(_) => fail!("writing to logfile")
    }
}
