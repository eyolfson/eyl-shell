extern crate wayland;

fn main() {
    let mut display = wayland::Display::new();
    let registry = wayland::Registry::new(&mut display);

    display.roundtrip();
}
