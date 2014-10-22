fn main() {
    let logfile_path = match std::os::homedir() {
        Some(p) => p.join(".eyl-shell.log"),
        None    => fail!("finding homedir")
    };
    let mut logfile = std::io::File::create(&logfile_path);
    match logfile.write(b"It works!\n") {
        Ok(()) => (),
        Err(_) => fail!("writing to logfile")
    }
}
