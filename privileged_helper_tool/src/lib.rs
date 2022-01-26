#![warn(clippy::pedantic)]
#![allow(clippy::double_must_use)]

#[cfg(feature = "authenticator")]
mod authenticator;
#[cfg(feature = "authenticator")]
pub use authenticator::*;

#[cfg(feature = "launchd")]
mod launchd;
#[cfg(feature = "launchd")]
pub use launchd::*;
