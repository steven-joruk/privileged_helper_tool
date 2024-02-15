use privileged_helper_tool::{LaunchDaemonListener, UnixStreamAuthenticator};
use std::ffi::CString;
use tokio::{
    io::AsyncWriteExt,
    net::{UnixListener, UnixStream},
};

/// Update this with your actual team ID, or whatever other verification you
/// want to perform.
/// See https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/RequirementLang/RequirementLang.html
const REQUIREMENT_TEXT: &str =
    "anchor apple generic and certificate leaf[subject.OU] = \"MY_TEAM_ID\"";

#[tokio::main]
async fn main() {
    if REQUIREMENT_TEXT.contains("MY_TEAM_ID") {
        panic!("You need to update REQUIREMENT_TEXT");
    }

    println!("Service starting");

    let name = CString::new("PrimarySocket").unwrap();
    let listener = UnixListener::from_launchd(name).unwrap();

    println!(
        "Listening on {}",
        listener
            .local_addr()
            .unwrap()
            .as_pathname()
            .unwrap()
            .display()
    );

    while let Ok((client, _addr)) = listener.accept().await {
        tokio::spawn(handle_client(client));
    }
}

async fn handle_client(mut client: UnixStream) {
    println!("A client connected");

    let allowed = !client
        .matches_code_requirement(REQUIREMENT_TEXT)
        .unwrap_or(false);

    if !allowed {
        client
            .write_all(b"Go away, you're not signed.")
            .await
            .unwrap();
        println!("The client's code signature doesn't satisfy the requirement.");
        return;
    }

    client.write_all(b"Hello there.").await.unwrap();

    println!("A client disconnected");
}
