#[cfg(feature = "nasl-builtin-libssh")]
mod libssh;
#[cfg(feature = "nasl-builtin-libssh")]
pub use libssh::{AuthMethods, SessionId, Socket, SshSession};

#[cfg(not(feature = "nasl-builtin-libssh"))]
mod russh;
#[cfg(not(feature = "nasl-builtin-libssh"))]
pub use russh::{AuthMethods, SessionId, Socket, SshSession};

pub use error::SshError;
pub use sessions::SshSessions as Ssh;

const MIN_SESSION_ID: SessionId = 9000;

mod error;
mod impls;
mod sessions;
mod utils;

#[cfg(test)]
mod tests;
