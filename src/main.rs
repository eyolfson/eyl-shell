extern crate wayland;

static mut LOGFILE: std::io::fs::File = match std::os::homedir() {
    Some(ref p) =>
        match std::io::File::create(&p.join(".eyl-shell.log")) {
            Ok(f) => f,
            Err(_) => fail!("?")
        },
    None        => fail!("finding homedir")
};

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
