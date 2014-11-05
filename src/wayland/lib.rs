#![feature(globs)]

extern crate libc;

pub use compositor::Compositor;
pub use display::Display;
pub use list::List;
pub use region::Region;
pub use registry::Registry;
pub use seat::Seat;
pub use shell::Shell;
pub use shm::Shm;
pub use shm_pool::ShmPool;
pub use surface::Surface;

mod compositor;
mod display;
mod list;
mod region;
mod registry;
mod seat;
mod shell;
mod shm;
mod shm_pool;
mod surface;

pub mod raw;
