extern crate wayland;

fn main() {
    let mut display = wayland::Display::new();
    let mut registry = wayland::Registry::new(&mut display);
    let compositor = registry.get_compositor();
}
