#![allow(clippy::module_name_repetitions)]

use security_framework::os::macos::code_signing::{
    Flags, GuestAttributes, SecCode, SecRequirement,
};
use tokio::net::UnixStream;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to get the peer's credentials")]
    PeerCred(#[source] std::io::Error),

    #[error("Failed to copy the client's code object")]
    CopyGuestCodeObject(#[source] security_framework::base::Error),

    #[error("Failed to parse the code requirements text")]
    CodeRequirementParse(#[source] security_framework::base::Error),
}

fn unix_domain_socket_matches_code_requirement(
    stream: &UnixStream,
    requirement: impl AsRef<str>,
) -> Result<bool, Error> {
    let creds = stream.peer_cred().map_err(Error::PeerCred)?;

    let mut attrs = GuestAttributes::new();
    attrs.set_pid(creds.pid().expect("macOS supports pid retrieval"));
    let code = SecCode::copy_guest_with_attribues(None, &attrs, Flags::NONE)
        .map_err(Error::CopyGuestCodeObject)?;

    let requirement: SecRequirement = requirement
        .as_ref()
        .parse()
        .map_err(Error::CodeRequirementParse)?;

    Ok(code.check_validity(Flags::NONE, &requirement).is_ok())
}

pub trait UnixStreamAuthenticator {
    /// # Errors
    ///
    /// The code requirement must be valid and the stream must be connected.
    fn matches_code_requirement(&self, requirement: impl AsRef<str>) -> Result<bool, Error>;
}

impl UnixStreamAuthenticator for UnixStream {
    fn matches_code_requirement(&self, requirement: impl AsRef<str>) -> Result<bool, Error> {
        unix_domain_socket_matches_code_requirement(self, requirement)
    }
}
