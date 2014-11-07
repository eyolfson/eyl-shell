extern crate wayland;

extern crate libc;

use std::ptr;

static WIDTH: i32 = 300;
static HEIGHT: i32 = 200;

#[link(name = "rt")]
extern {} // So we can link to shm_open and shm_unlink

struct ShmFd {
    fd: i32,
    ptr: *mut libc::c_void
}

impl ShmFd {
    pub fn new() -> ShmFd {
        unsafe {
            let fd = libc::funcs::posix88::mman::shm_open(
                "/eyl-hello-world".to_c_str().as_ptr(),
                libc::O_CREAT | libc::O_RDWR,
                libc::S_IRUSR | libc::S_IWUSR
            );
            assert!(fd >= 0);
            let size = WIDTH * HEIGHT * 4;
            assert!(libc::ftruncate(fd, size as i64) != -1);
            let ptr = libc::mmap(ptr::null_mut(), size as u64,
                                 libc::PROT_WRITE | libc::PROT_READ, 1, fd, 0);
            assert!(ptr != libc::MAP_FAILED);
            for i in range(0i32, WIDTH * HEIGHT) {
                let p: *mut u32 = (ptr as *mut u32).offset(i as int);
                if i % 2 == 0 {
                    std::ptr::write(&mut *p, 0x00FF00FF);
                }
                else {
                    std::ptr::write(&mut *p, 0x00000000);
                }
            }
            ShmFd { fd: fd, ptr: ptr }
        }
    }
    pub fn fd(&self) -> i32 {
        self.fd
    }
}

impl Drop for ShmFd {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, WIDTH as u64 * HEIGHT as u64 * 4);
            libc::funcs::posix88::mman::shm_unlink(
                "/eyl-hello-world".to_c_str().as_ptr()
            );
        }
    }
}

fn main() {
    let mut display = wayland::Display::new();
    let mut registry = wayland::Registry::new(&mut display);
    let shm_fd = ShmFd::new();
    let mut pool = registry.shm().create_pool(shm_fd.fd(), WIDTH * HEIGHT * 4);
    let mut surface = registry.compositor().create_surface();
    let mut buffer = pool.create_buffer(
        0, WIDTH, HEIGHT, WIDTH * 4, wayland::raw::WL_SHM_FORMAT_ARGB8888
    );
    surface.attach(&mut buffer, 0, 0);
    let mut shell_surface = registry.shell().get_shell_surface(&mut surface);
    shell_surface.set_toplevel();
    surface.commit();
    loop {
        display.dispatch();
    }
}
