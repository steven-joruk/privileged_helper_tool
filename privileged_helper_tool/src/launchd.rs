#![allow(clippy::module_name_repetitions)]

use std::{
    ffi::{c_void, CStr},
    os::{
        fd::OwnedFd,
        raw::{c_char, c_int},
        unix::prelude::FromRawFd,
    },
};
use tokio::net::UnixListener;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to retrieve the socket from launchd")]
    Activation(i32),

    #[error("You must only define one socket in the launchd plist. Found {0}.")]
    TooManySockets(u32),

    #[error("Unable to set the socket in to non-blocking mode")]
    SetNonBlocking(#[source] std::io::Error),

    #[error("Failed to convert the UnixListener from std to tokio")]
    ConvertUnixListener(#[source] std::io::Error),
}

extern "C" {
    fn free(p: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn launch_activate_socket(name: *const c_char, fds: *mut *mut c_int, count: *mut u32) -> c_int;
}

pub trait LaunchDaemonListener {
    /// # Errors
    ///
    /// For now your launch daemon's configuration must only specify a single
    /// `AF_UNIX` socket. See `man launchd.plist`.
    #[must_use]
    fn from_launchd(socket_entry_name: impl AsRef<CStr>) -> Result<UnixListener, Error> {
        let mut fds = std::ptr::null_mut();
        let mut count = 0;

        let status = unsafe {
            launch_activate_socket(socket_entry_name.as_ref().as_ptr(), &mut fds, &mut count)
        };

        if status != 0 {
            return Err(Error::Activation(status));
        }

        if count != 1 {
            for index in 0..count {
                unsafe { close(*fds.add(index as usize)) };
            }

            unsafe { free(fds.cast()) };
            return Err(Error::TooManySockets(count));
        };

        let fd = unsafe { OwnedFd::from_raw_fd(*fds.offset(0_isize)) };
        unsafe { free(fds.cast()) };

        let std_listener = std::os::unix::net::UnixListener::from(fd);
        std_listener
            .set_nonblocking(true)
            .map_err(Error::SetNonBlocking)?;

        UnixListener::from_std(std_listener).map_err(Error::ConvertUnixListener)
    }
}

impl LaunchDaemonListener for UnixListener {}
