## About

This crate provides two traits that are useful for creating privileged helper
tools on macOS, but they could be used in an unprivileged context as well.

### LaunchDaemonListener

By configuring a [launchd.plist](https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html)
you can have your daemon start on demand when a client attempts to connect to its configured socket.

Using the `LaunchDaemonListener` trait you can get process to the socket
provided by launchd.

### UnixStreamAuthenticator

When creating privileged helper tools it's essential to ensure that only the
tools they're intended to help are able to use them.

`UnixStreamAuthenticator` provides an easy API for verifying that clients are
code signed by you or your team, allowing you to block access by any other
tools.

## Installation

**You'll need to modify the requirement text in [authenticating.rs](examples/authenticating/src/main.rs).**

```bash
cargo build

sudo cp target/debug/authenticating /Library/PrivilegedHelperTools/com.example.authenticating
sudo cp examples/authenticating/com.example.authenticating.plist /Library/LaunchDaemons/
sudo launchctl load /Library/LaunchDaemons/com.example.authenticating.plist

# Creating a signed client
cp target/debug/client target/debug/signed_client
codesign -s $YOUR_CERT_NAME target/debug/signed_client
```

## To do

1. Explain how to diagnose issues
2. Support customising the code signing flags.
3. Examples
   * [Shutdown](https://tokio.rs/tokio/topics/shutdown)
   * `SMJobBless`
   * Improve the client so it can connect to either listener.

## Missing features

Support for these won't be implemented until a need arises:
* Socket address families other than `AF_UNIX`
* More than one socket
* Providing flags to code signature verification

## Why not XPC?

XPC is often recommended on Apple's forums and it's usually touted as being easy
to use and high performance. I've not found that to be the case:

* [There have been several vulnerabilities with privileged helper tools using XPC](https://wojciechregula.blog/post/learn-xpc-exploitation-part-1-broken-cryptography/),
  it's been difficult even for the developers of security products to get right.
  Apple have only recently [provided an easy way to authenticate clients](https://developer.apple.com/documentation/xpc/3755524-xpc_connection_set_peer_code_sig),
  but didn't back-port the new function to older verions fof macOS.
* XPC connections aren't guaranteed to be stable - launchd will sometimes kill
  XPC services without warning. You're expected to use a stateless protocol, or
  to re-send state when the connection is resumed.
* Diagnosing XPC connectivity issues can be difficult, you'll often just see
  that the connection was invalidated.
* [My benchmarks](https://github.com/steven-joruk/macos-ipc-benchmarks)
  have shown UDS to be marginally faster for small messages sizes and
  significantly faster for connection creation.
* Lots of crates support UDS as a transport, or can be made to with very little
  effort.
