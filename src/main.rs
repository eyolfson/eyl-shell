extern crate wayland;

fn main() {
    let logfile_path = match std::os::homedir() {
        Some(ref p) => p.join(".eyl-shell.log"),
        None        => fail!("finding homedir")
    };
    let mut logfile = std::io::File::create(&logfile_path);
    let display = wayland::Display::new();
    match logfile.write(b"Connected to display\n") {
        Ok(()) => (),
        Err(_) => fail!("writing to logfile")
    }
}
