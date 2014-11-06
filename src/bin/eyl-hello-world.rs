extern crate wayland;

fn main() {
    let mut display = wayland::Display::new();
    let mut registry = wayland::Registry::new(&mut display);
    let mut surface = registry.compositor().create_surface();
    let shell_surface = registry.shell().get_shell_surface(&mut surface);
}
