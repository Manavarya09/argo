/// Common authentication status across all provider detectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthStatus {
    /// Provider is installed and has valid local credentials.
    Authenticated,
    /// Provider is installed but no credentials present.
    NotAuthenticated,
    /// Provider's CLI/SDK is not installed at all.
    NotInstalled,
    /// Provider has credentials but they appear stale or expired.
    Stale,
}

impl AuthStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Authenticated)
    }
}
