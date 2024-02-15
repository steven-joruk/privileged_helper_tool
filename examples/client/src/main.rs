use std::{io::Read, os::unix::net::UnixStream};

fn main() {
    let mut client = UnixStream::connect("/var/run/com.example.authenticating").unwrap();

    let mut s = String::new();
    let size = client.read_to_string(&mut s).unwrap();
    println!("read {size} bytes: {s}");
}
