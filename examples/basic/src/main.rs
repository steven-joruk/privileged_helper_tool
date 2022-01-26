use privileged_helper_tool::LaunchDaemonListener;
use std::ffi::CString;
use tokio::{
    io::AsyncReadExt,
    net::{UnixListener, UnixStream},
};

#[tokio::main]
async fn main() {
    println!("Service starting");

    let name = CString::new("PrimarySocket").unwrap();
    let listener = UnixListener::from_launchd(name).unwrap();

    while let Ok((client, _addr)) = listener.accept().await {
        tokio::spawn(handle_client(client));
    }

    println!("Service exiting");
}

async fn handle_client(mut client: UnixStream) {
    println!("A client connected");

    let mut s = String::new();
    while let Ok(len) = client.read_to_string(&mut s).await {
        if len == 0 {
            break;
        }

        println!("Received {}", s);
        s.clear();
    }

    println!("A client disconnected");
}
