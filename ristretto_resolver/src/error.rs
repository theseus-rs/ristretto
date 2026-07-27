//! Resolver errors and the crate-wide result alias.
//!
//! [`Error`] separates invalid configuration and coordinates from model, version-selection,
//! transport, checksum, cache, and destination failures. Transport errors retain a safe resource
//! URL for diagnostics; repository credentials are represented by redacting types elsewhere and
//! are never included in these messages.

use crate::ArtifactCoordinate;

/// Result type used throughout the resolver.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors produced while configuring, resolving, or downloading dependencies.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An artifact coordinate or key was malformed.
    #[error("invalid artifact coordinate: {0}")]
    InvalidCoordinate(String),
    /// A version or version range was malformed.
    #[error("invalid version specification: {0}")]
    InvalidVersion(String),
    /// Resolver configuration was contradictory or incomplete.
    #[error("invalid resolver configuration: {0}")]
    InvalidConfiguration(String),
    /// A required artifact could not be found in any eligible repository.
    #[error("artifact not found: {0}")]
    ArtifactNotFound(ArtifactCoordinate),
    /// Repository metadata could not be found.
    #[error("metadata not found for {0}")]
    MetadataNotFound(String),
    /// A dependency descriptor could not be interpreted.
    #[error("invalid dependency model for {coordinate}: {message}")]
    Model {
        /// Coordinate whose model failed.
        coordinate: ArtifactCoordinate,
        /// Explanation of the model failure.
        message: String,
    },
    /// Version constraints could not be mediated.
    #[error("unsatisfiable version conflict for {artifact}: {versions:?}")]
    VersionConflict {
        /// Artifact conflict key.
        artifact: String,
        /// Participating version specifications.
        versions: Vec<String>,
    },
    /// Property interpolation failed.
    #[error("property interpolation failed: {0}")]
    Interpolation(String),
    /// A dependency, parent, or relocation cycle was detected.
    #[error("resolution cycle detected: {0}")]
    Cycle(String),
    /// No transport was registered for a repository URI scheme.
    #[error("no transport registered for URI scheme '{0}'")]
    UnsupportedTransport(String),
    /// A transport failed.
    #[error("transport error for {url}: {message}")]
    Transport {
        /// URL being accessed.
        url: String,
        /// Transport-provided failure.
        message: String,
    },
    /// A remote checksum did not match the downloaded bytes.
    #[error("checksum mismatch for {coordinate}: expected {expected}, calculated {actual}")]
    ChecksumMismatch {
        /// Artifact that failed validation.
        coordinate: ArtifactCoordinate,
        /// Expected checksum.
        expected: String,
        /// Calculated checksum.
        actual: String,
    },
    /// A strict repository did not provide a supported checksum.
    #[error("repository did not provide a checksum for {0}")]
    MissingChecksum(ArtifactCoordinate),
    /// An artifact destination failed.
    #[error("artifact destination error: {0}")]
    Destination(String),
    /// An operation requiring the network was attempted in offline mode.
    #[error("offline mode prevented retrieval of {0}")]
    Offline(String),
    /// An I/O operation failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A URL was invalid.
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// A POM could not be parsed.
    #[error(transparent)]
    Pom(#[from] ristretto_pom::Error),
    /// Repository XML metadata could not be parsed.
    #[error(transparent)]
    Xml(#[from] quick_xml::DeError),
}

#[cfg(not(target_family = "wasm"))]
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Transport {
            url: error
                .url()
                .map_or_else(|| "<unknown>".to_string(), ToString::to_string),
            message: error.to_string(),
        }
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;

    #[test]
    fn converts_reqwest_errors_without_urls() {
        let error = reqwest::Client::new()
            .get("://invalid")
            .build()
            .expect_err("invalid request URL");
        let converted = Error::from(error);
        assert!(matches!(
            converted,
            Error::Transport { url, .. } if url == "<unknown>"
        ));
    }
}
